use std::{collections::HashMap, str::FromStr, sync::Arc};

use js_sys::{Float32Array, JsString, Promise};
use rayon::prelude::*;
use raytracer_lib::{public_consts, render_pixel, Framebuffer, Real, SceneDescription, SceneGraph};
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{
    Request, RequestInit, RequestMode, Response, WebGl2RenderingContext, WebGlContextAttributes,
    WebGlProgram, WebGlShader,
};

pub use wasm_bindgen_rayon::init_thread_pool;

fn js_to_err(err: JsValue) -> Box<dyn std::error::Error> {
    JsString::from(err).as_string().unwrap().into()
}

fn err_to_js(err: Box<dyn std::error::Error>) -> JsValue {
    JsString::from(err.to_string()).into()
}

#[derive(Deserialize, Debug)]
pub struct RayTracerArgs {
    width: u32,
    height: u32,
    #[serde(default)]
    rays_per_pixel: Option<u16>,
    #[serde(default)]
    recursion_depth: Option<u16>,
    #[serde(default)]
    aspect_ratio: Option<Real>,
    #[serde(default)]
    antialias_method: Option<String>,
}

// macro for wasm log does format!
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[wasm_bindgen]
pub struct RayTracer {
    scene: SceneGraph,
    fb: Arc<Framebuffer>,
    sqrt_rays_per_pixel: u16,
    antialias_method: raytracer_lib::AntialiasMethod,
    next_pixel: (u32, u32),
    pub complete: bool,
    context: WebGl2RenderingContext,
}

#[wasm_bindgen]
impl RayTracer {
    #[wasm_bindgen]
    pub fn test_rayon(&self) -> Promise {
        // par iter a sum of (0..1000)
        let sum = (0..1000000).into_par_iter().sum::<i32>();
        log!("FROM WASM> Sum: {}", sum);

        Promise::resolve(&JsValue::from_f64(sum as f64))
    }

    #[wasm_bindgen]
    pub fn init(canvas_id: String, scene_json: String, raytracer_args: JsValue) -> Promise {
        future_to_promise(async move {
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas = match document.get_element_by_id(&canvas_id) {
                Some(e) => e.dyn_into::<web_sys::HtmlCanvasElement>()?,
                None => return Err(JsValue::from_str("Failed to get canvas")),
            };

            // Parse the raytrace args from JSON
            let args: RayTracerArgs = serde_wasm_bindgen::from_value(raytracer_args)?;

            let scene_desc = SceneDescription::from_json(&scene_json).map_err(err_to_js)?;

            #[cfg(debug_assertions)]
            log!("{:#?}", scene_desc);

            let rays_per_pixel = args
                .rays_per_pixel
                .unwrap_or(public_consts::DEFAULT_RAYS_PER_PIXEL);
            let sqrt_rays_per_pixel = (rays_per_pixel as f64).sqrt() as u16;

            // error if rays_per_pixel is not a perfect square
            if sqrt_rays_per_pixel * sqrt_rays_per_pixel != rays_per_pixel {
                return Err(JsValue::from_str("rays_per_pixel must be a perfect square"));
            }

            let antialias_method = match args.antialias_method {
                Some(ref s) => raytracer_lib::AntialiasMethod::from_str(s).unwrap(),
                _ => raytracer_lib::AntialiasMethod::Normal,
            };

            // Set up WebGL2
            canvas.set_width(args.width);
            canvas.set_height(args.height);
            let context = {
                #[cfg(debug_assertions)]
                test_webgl2()?;

                let context_attributes = WebGlContextAttributes::new();
                context_attributes.set_alpha(true);
                context_attributes.set_antialias(true);

                context_attributes
                    .set_power_preference(web_sys::WebGlPowerPreference::HighPerformance);

                let context = if let Some(context) =
                    canvas.get_context_with_context_options("webgl2", &context_attributes)?
                {
                    context.dyn_into::<WebGl2RenderingContext>()?
                } else {
                    return Err(JsValue::from_str("Failed to get WebGL2 context"));
                };

                // Add some debug info
                #[cfg(debug_assertions)]
                {
                    let version = context.get_parameter(WebGl2RenderingContext::VERSION)?;
                    let vendor = context.get_parameter(WebGl2RenderingContext::VENDOR)?;
                    let renderer = context.get_parameter(WebGl2RenderingContext::RENDERER)?;

                    log!("WebGL2 version: {}", version.as_string().unwrap());
                    log!("WebGL2 vendor: {}", vendor.as_string().unwrap());
                    log!("WebGL2 renderer: {}", renderer.as_string().unwrap());
                }

                // You can also add this check after getting the context
                if context.is_null() {
                    return Err(JsValue::from_str("WebGL2 context is null"));
                }

                // Vertex shader - just pass through positions
                let vert_shader = compile_shader(
                    &context,
                    WebGl2RenderingContext::VERTEX_SHADER,
                    r#"#version 300 es
            in vec2 position;
            out vec2 texCoord;
            void main() {
                texCoord = position * 0.5 + 0.5;
                gl_Position = vec4(position, 0.0, 1.0);
            }
            "#,
                )?;

                // Fragment shader - sample from texture
                let frag_shader = compile_shader(
                    &context,
                    WebGl2RenderingContext::FRAGMENT_SHADER,
                    r#"#version 300 es
            precision highp float;
            uniform sampler2D tex;
            in vec2 texCoord;
            out vec4 fragColor;
            void main() {
                vec3 color = texture(tex, texCoord).rgb;
                fragColor = vec4(color, 1.0);
            }
            "#,
                )?;

                let program = link_program(&context, &vert_shader, &frag_shader)?;
                context.use_program(Some(&program));

                // Create vertex buffer for full-screen quad
                let vertices: [f32; 12] = [
                    -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0,
                ];

                let vertex_buffer = context.create_buffer().ok_or("Failed to create buffer")?;
                context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

                unsafe {
                    let vert_array = js_sys::Float32Array::view(&vertices);
                    context.buffer_data_with_array_buffer_view(
                        WebGl2RenderingContext::ARRAY_BUFFER,
                        &vert_array,
                        WebGl2RenderingContext::STATIC_DRAW,
                    );
                }

                let position_attrib = context.get_attrib_location(&program, "position") as u32;
                context.vertex_attrib_pointer_with_i32(
                    position_attrib,
                    2,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    0,
                    0,
                );
                context.enable_vertex_attrib_array(position_attrib);

                context
            };

            // fetch data from IndexedDB
            let mut scene_data: HashMap<String, Vec<u8>> = HashMap::new();
            for relative_path in &scene_desc.data_needed {
                let bytes = fetch_data(relative_path).await.map_err(err_to_js)?;
                scene_data.insert(relative_path.clone(), bytes);
            }

            let scene_graph = SceneGraph::from_description(
                &scene_desc,
                &scene_data,
                Some(args.width),
                Some(args.height),
                args.aspect_ratio,
                args.recursion_depth,
            )
            .map_err(err_to_js)?;

            Ok(JsValue::from(RayTracer {
                scene: scene_graph,
                fb: Arc::new(Framebuffer::new(args.width, args.height)),
                sqrt_rays_per_pixel,
                antialias_method,
                next_pixel: (0, 0),
                complete: false,
                context,
            }))
        })
    }

    #[wasm_bindgen]
    pub fn raytrace_next_pixels(&mut self, num_pixels: u32) -> Promise {
        let mut count = 0;
        let (mut i, mut j) = self.next_pixel;

        while i < self.scene.image_width && count < num_pixels {
            while j < self.scene.image_height && count < num_pixels {
                render_pixel(
                    self.fb.clone(),
                    &self.scene,
                    self.sqrt_rays_per_pixel,
                    self.antialias_method,
                    i,
                    j,
                );

                count += 1;
                j += 1;
            }

            if j >= self.scene.image_height {
                j = 0;
                i += 1;
            }
        }

        self.next_pixel = (i, j);

        // Check if we've completed the entire image
        if i >= self.scene.image_width {
            self.complete = true;
        }

        // Calculate total pixels processed
        let total_pixels = if self.complete {
            (self.scene.image_width * self.scene.image_height) as f64
        } else {
            (i * self.scene.image_height + j) as f64
        };

        Promise::resolve(&JsValue::from_f64(total_pixels))
    }

    #[wasm_bindgen]
    pub fn raytrace_parallel(&mut self) -> Promise {
        (0..self.scene.image_width).into_par_iter().for_each(|i| {
            for j in 0..self.scene.image_height {
                render_pixel(
                    self.fb.clone(),
                    &self.scene,
                    self.sqrt_rays_per_pixel,
                    self.antialias_method,
                    i,
                    j,
                );
            }
        });

        self.complete = true;
        Promise::resolve(&JsValue::undefined())
    }

    #[wasm_bindgen]
    pub fn render_to_canvas(&self) -> Result<(), JsValue> {
        let texture = self
            .context
            .create_texture()
            .ok_or("Failed to create texture")?;

        self.context
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        // Set texture parameters
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );

        unsafe {
            // Upload pixels to texture
            let pixels: &[f32] = std::slice::from_raw_parts(
                self.fb.get_pixels().as_ptr() as *const f32,
                self.fb.get_pixels().len() * 3,
            );

            let pixels_array = Float32Array::view(pixels);
            self.context
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    WebGl2RenderingContext::RGB32F as i32,
                    self.scene.image_width as i32,
                    self.scene.image_height as i32,
                    0,
                    WebGl2RenderingContext::RGB,
                    WebGl2RenderingContext::FLOAT,
                    Some(&pixels_array),
                )?;
        }

        // Draw full-screen quad
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);

        Ok(())
    }
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program")))
    }
}

async fn fetch_data(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let js_to_err = |err: JsValue| -> Box<dyn std::error::Error> {
        JsString::from(err).as_string().unwrap().into()
    };

    let request = Request::new_with_str_and_init(url, &opts).map_err(js_to_err)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(js_to_err)?;
    let resp: Response = resp_value.dyn_into().map_err(js_to_err)?;

    // Get the response as ArrayBuffer
    let buf = JsFuture::from(resp.array_buffer().map_err(js_to_err)?)
        .await
        .map_err(js_to_err)?;
    let uint8_array = js_sys::Uint8Array::new(&buf);
    let mut bytes = vec![0; uint8_array.length() as usize];
    uint8_array.copy_to(&mut bytes);

    Ok(bytes)
}

#[cfg(debug_assertions)]
fn test_webgl2() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let test_canvas = document.create_element("canvas")?;
    let test_canvas: web_sys::HtmlCanvasElement = test_canvas.dyn_into()?;
    match test_canvas.get_context("webgl2") {
        Ok(Some(_)) => log!("WebGL2 is supported by the browser"),
        Ok(None) => log!("WebGL2 is NOT supported by the browser"),
        Err(e) => log!("Error checking WebGL2 support: {:?}", e),
    };

    Ok(())
}
