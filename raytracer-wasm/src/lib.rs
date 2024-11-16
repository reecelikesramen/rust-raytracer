use std::str::FromStr;

use js_sys::Float32Array;
use raytracer_lib::{parse_scene, public_consts, render_mut, Framebuffer};
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlContextAttributes, WebGlProgram, WebGlShader};

#[derive(Deserialize, Debug)]
pub struct RayTracerArgs {
    width: u32,
    height: u32,
    #[serde(default)]
    rays_per_pixel: Option<u16>,
    #[serde(default)]
    recursion_depth: Option<u16>,
    #[serde(default)]
    aspect_ratio: Option<f64>,
    #[serde(default)]
    disable_shadows: bool,
    #[serde(default)]
    render_normals: bool,
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
pub struct PixelRenderer {
    context: WebGl2RenderingContext,
    program: WebGlProgram,
    width: u32,
    height: u32,
    fb: Framebuffer,
}

#[wasm_bindgen]
impl PixelRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Result<PixelRenderer, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        document.body().unwrap().append_child(&canvas)?;

        canvas.set_width(width);
        canvas.set_height(height);

        let context_attributes = WebGlContextAttributes::new();
        context_attributes.set_alpha(true);
        context_attributes.set_antialias(true);

        context_attributes.set_power_preference(web_sys::WebGlPowerPreference::HighPerformance);

        let context = if let Some(context) =
            canvas.get_context_with_context_options("webgl2", &context_attributes)?
        {
            context.dyn_into::<WebGl2RenderingContext>()?
        } else {
            return Err(JsValue::from_str("Failed to get WebGL2 context"));
        };

        // Add some debug info
        let version = context.get_parameter(WebGl2RenderingContext::VERSION)?;
        let vendor = context.get_parameter(WebGl2RenderingContext::VENDOR)?;
        let renderer = context.get_parameter(WebGl2RenderingContext::RENDERER)?;

        log!("WebGL2 version: {}", version.as_string().unwrap());
        log!("WebGL2 vendor: {}", vendor.as_string().unwrap());
        log!("WebGL2 renderer: {}", renderer.as_string().unwrap());

        // You can also add this check after getting the context
        if context.is_null() {
            return Err(JsValue::from_str("WebGL2 context is null"));
        }

        log!("I made it");

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

        Ok(PixelRenderer {
            context,
            program,
            width,
            height,
            fb: Framebuffer::new(width, height),
        })
    }

    #[wasm_bindgen]
    pub fn raytrace(&mut self, scene_data: String, args: JsValue) -> Result<(), JsValue> {
        let args: RayTracerArgs = serde_wasm_bindgen::from_value(args)?;

        let scene = parse_scene(
            &scene_data,
            Some(args.width),
            Some(args.height),
            args.aspect_ratio,
            args.recursion_depth,
            args.disable_shadows,
            args.render_normals,
        )
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // log!("{:#?}", scene);

        let rays_per_pixel = args
            .rays_per_pixel
            .unwrap_or(public_consts::DEFAULT_RAYS_PER_PIXEL);
        let sqrt_rays_per_pixel = (rays_per_pixel as f64).sqrt() as u16;

        // error if rays_per_pixel is not a perfect square
        if sqrt_rays_per_pixel * sqrt_rays_per_pixel != rays_per_pixel {
            return Err(JsValue::from_str("rays_per_pixel must be a perfect square"));
        }

        let antialias_method = match args.antialias_method {
            Some(s) => raytracer_lib::AntialiasMethod::from_str(s.as_str()).unwrap(),
            _ => raytracer_lib::AntialiasMethod::Normal,
        };

        render_mut(
            &mut self.fb,
            &scene,
            sqrt_rays_per_pixel,
            antialias_method,
            None,
            None,
            // Some(&wasm_log),
        );

        Ok(())
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
                self.fb.pixels.as_ptr() as *const f32,
                self.fb.pixels.len() * 3,
            );

            let pixels_array = Float32Array::view(pixels);
            self.context
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    WebGl2RenderingContext::RGB32F as i32,
                    self.width as i32,
                    self.height as i32,
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

#[wasm_bindgen]
pub fn test_webgl() -> Result<(), JsValue> {
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
