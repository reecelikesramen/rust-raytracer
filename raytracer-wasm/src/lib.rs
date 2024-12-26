use js_sys::{Float32Array, JsString, Promise};
use rayon::prelude::*;
use raytracer_lib::{public_consts, render_pixel, Framebuffer, Real, SceneDescription, SceneGraph};
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;
use web_sys::{
    OffscreenCanvas, WebGl2RenderingContext, WebGlContextAttributes, WebGlProgram, WebGlShader,
    WebGlTexture,
};

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

// js string macro
macro_rules! js {
    ( $( $t:tt )* ) => {
        JsValue::from_str(&format!( $( $t )* ))
    }
}

struct RayTracer {
    pub complete: bool,
    next_pixel: (u32, u32),
    scene: SceneGraph,
    fb: Arc<Framebuffer>,
    pub sqrt_rays_per_pixel: u16,
    antialias_method: raytracer_lib::AntialiasMethod,
    context: Option<WebGl2RenderingContext>,
    texture: Option<WebGlTexture>,
    background_texture: Option<WebGlTexture>,
}

#[wasm_bindgen]
pub struct RayTracerApp {
    scene_desc: Option<SceneDescription>,
    raytracer: Option<RayTracer>,
}

#[wasm_bindgen]
impl RayTracerApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            scene_desc: None,
            raytracer: None,
        }
    }

    #[wasm_bindgen]
    pub fn parse_scene(&mut self, scene_json: String) -> Result<JsValue, JsValue> {
        let scene_desc = SceneDescription::from_json(&scene_json).map_err(err_to_js)?;

        // Convert data_needed to a JS array to return to JavaScript
        let paths = js_sys::Array::new();
        for path in &scene_desc.data_needed {
            paths.push(&JsValue::from_str(path));
        }

        self.scene_desc = Some(scene_desc);
        Ok(paths.into())
    }

    #[wasm_bindgen]
    pub fn is_ready(&self) -> bool {
        self.raytracer.is_some()
    }

    #[wasm_bindgen]
    pub fn is_complete(&self) -> bool {
        match &self.raytracer {
            Some(raytracer) => raytracer.complete,
            None => false,
        }
    }

    #[wasm_bindgen]
    pub fn get_needed_resources(&self) -> Vec<JsString> {
        match &self.scene_desc {
            Some(scene_desc) => scene_desc
                .data_needed
                .iter()
                .map(|path| JsString::from(path.clone()))
                .collect(),
            None => Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn initialize(
        &mut self,
        canvas_id: String,
        raytracer_args: JsValue,
        scene_data_js: JsValue,
    ) -> Result<(), JsValue> {
        let scene_desc = self
            .scene_desc
            .take()
            .ok_or(js!("Scene description not parsed yet"))?;

        let document = web_sys::window()
            .ok_or(js!("Failed to get window"))?
            .document()
            .ok_or(js!("Failed to get document"))?;

        let canvas = document
            .get_element_by_id(&canvas_id)
            .ok_or(js!("Failed to get canvas element"))?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| js!("Failed to cast element to canvas"))?;

        // Parse the raytrace args from JSON
        let args: RayTracerArgs = serde_wasm_bindgen::from_value(raytracer_args)
            .map_err(|e| js!("Failed to parse raytracer args: {:?}", e))?;

        #[cfg(debug_assertions)]
        log!("{:#?}", scene_desc);

        let rays_per_pixel = args
            .rays_per_pixel
            .unwrap_or(public_consts::DEFAULT_RAYS_PER_PIXEL);
        let sqrt_rays_per_pixel = (rays_per_pixel as f64).sqrt() as u16;

        // error if rays_per_pixel is not a perfect square
        if sqrt_rays_per_pixel * sqrt_rays_per_pixel != rays_per_pixel {
            return Err(js!("rays_per_pixel must be a perfect square"));
        }

        let antialias_method = match args.antialias_method {
            Some(ref s) => raytracer_lib::AntialiasMethod::from_str(s)
                .map_err(|e| js!("Failed to parse antialias method: {:?}", e))?,
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
            context_attributes.set_premultiplied_alpha(false);
            context_attributes.set_antialias(true);

            context_attributes.set_power_preference(web_sys::WebGlPowerPreference::HighPerformance);

            let context = if let Some(context) =
                canvas.get_context_with_context_options("webgl2", &context_attributes)?
            {
                context.dyn_into::<WebGl2RenderingContext>()?
            } else {
                return Err(js!("Failed to get WebGL2 context"));
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
                vec4 color = texture(tex, texCoord);
                fragColor = color;
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

            // Handle transparency
            context.enable(WebGl2RenderingContext::BLEND);
            context.blend_func(
                WebGl2RenderingContext::SRC_ALPHA,
                WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
            );

            context
        };

        // Get data from JS object
        let mut scene_data: HashMap<String, Vec<u8>> = HashMap::new();
        let js_obj = js_sys::Object::from(scene_data_js);

        // Get all the entries from the JS object
        for relative_path in &scene_desc.data_needed {
            if let Ok(Some(array)) =
                js_sys::Reflect::get(&js_obj, &JsValue::from_str(relative_path))
                    .map(|v| v.dyn_into::<js_sys::Uint8Array>().ok())
            {
                let mut bytes = vec![0; array.length() as usize];
                array.copy_to(&mut bytes);
                scene_data.insert(relative_path.clone(), bytes);
            } else {
                return Err(JsValue::from_str(&format!(
                    "Missing or invalid data for path: {}",
                    relative_path
                )));
            }
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

        self.raytracer = Some(RayTracer {
            complete: false,
            next_pixel: (0, 0),
            scene: scene_graph,
            fb: Arc::new(Framebuffer::new(args.width, args.height)),
            sqrt_rays_per_pixel,
            antialias_method,
            context: Some(context),
            texture: None,
            background_texture: None,
        });

        Ok(())
    }

    #[wasm_bindgen]
    pub fn raytrace_next_pixels(&mut self, num_pixels: u32) -> Promise {
        let raytracer = match self.raytracer.as_mut() {
            Some(rt) => rt,
            None => return Promise::reject(&js!("Raytracer not initialized")),
        };

        let mut count = 0;
        let (mut i, mut j) = raytracer.next_pixel;
        let mut pixels: Vec<(u32, u32)> = Vec::with_capacity(num_pixels as usize);

        while i < raytracer.fb.width && count < num_pixels {
            while j < raytracer.fb.height && count < num_pixels {
                pixels.push((i, j));

                count += 1;
                j += 1;
            }

            if j >= raytracer.fb.height {
                j = 0;
                i += 1;
            }
        }

        pixels.into_par_iter().for_each(|(i, j)| {
            render_pixel(
                raytracer.fb.clone(),
                &raytracer.scene,
                raytracer.sqrt_rays_per_pixel,
                raytracer.antialias_method,
                i,
                j,
            );
        });

        raytracer.next_pixel = (i, j);

        // Check if we've completed the entire image
        if i >= raytracer.fb.width {
            raytracer.complete = true;
        }

        Promise::resolve(&JsValue::undefined())
    }

    #[wasm_bindgen]
    pub fn rescan(&mut self) -> Result<(), JsValue> {
        let raytracer = match self.raytracer.as_mut() {
            Some(rt) => rt,
            None => return Err(js!("Raytracer not initialized")),
        };

        raytracer.next_pixel = (0, 0);
        raytracer.complete = false;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn set_dimensions(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        let raytracer = match self.raytracer.as_mut() {
            Some(rt) => rt,
            None => return Err(js!("Raytracer not initialized")),
        };

        let context = match raytracer.context.as_ref() {
            Some(ctx) => ctx,
            None => return Err(js!("WebGL context not initialized")),
        };

        raytracer.fb = Arc::new(Framebuffer::new(width, height));

        // Delete background old texture
        if let Some(tex) = raytracer.background_texture.take() {
            context.delete_texture(Some(&tex));
        }

        // Move previous foreground texture to background texture
        raytracer.background_texture = raytracer.texture.take();

        // Create new foreground texture
        raytracer.texture = Some(context.create_texture().expect("Failed to create texture"));

        // Bind foreground texture
        context.bind_texture(
            WebGl2RenderingContext::TEXTURE_2D,
            raytracer.texture.as_ref(),
        );

        // Set texture parameters
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );

        self.rescan()?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn render_to_canvas(&self) -> Result<(), JsValue> {
        let raytracer = match self.raytracer.as_ref() {
            Some(rt) => rt,
            None => return Err(js!("Raytracer not initialized")),
        };

        let context = match raytracer.context.as_ref() {
            Some(ctx) => ctx,
            None => return Err(js!("WebGL context not initialized")),
        };

        // Clear canvas
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        // Exit if no texture
        if raytracer.texture.is_none() {
            return Ok(());
        }

        // If background texture, bind and draw
        if let Some(tex) = raytracer.background_texture.as_ref() {
            context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(tex));

            // Draw full-screen quad
            context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
        }

        // bind foreground texture
        let texture_ref = raytracer.texture.as_ref();
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture_ref);

        let pixels = framebuffer_to_rgba32f(&raytracer.fb);

        // Upload pixels to texture
        unsafe {
            // View over pixels
            let pixels_view = Float32Array::view(std::slice::from_raw_parts(
                pixels.as_ptr() as *const f32,
                pixels.len() * 4,
            ));

            // Bind texture and upload pixels
            context
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    WebGl2RenderingContext::RGBA32F as i32,
                    raytracer.fb.width as i32,
                    raytracer.fb.height as i32,
                    0,
                    WebGl2RenderingContext::RGBA,
                    WebGl2RenderingContext::FLOAT,
                    Some(&pixels_view),
                )?;
        }

        // Draw full-screen quad
        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);

        Ok(())
    }
}

#[wasm_bindgen]
pub fn full_render(
    scene_json: String,
    raytracer_args: JsValue,
    scene_data_js: JsValue,
    offscreen_canvas: OffscreenCanvas,
) -> Result<(), JsValue> {
    // Parse the raytrace args from JSON
    let args: RayTracerArgs = serde_wasm_bindgen::from_value(raytracer_args)
        .map_err(|e| js!("Failed to parse raytracer args: {:?}", e))?;

    let scene_desc = SceneDescription::from_json(&scene_json).map_err(err_to_js)?;

    #[cfg(debug_assertions)]
    log!("{:#?}", scene_desc);

    let rays_per_pixel = args
        .rays_per_pixel
        .unwrap_or(public_consts::DEFAULT_RAYS_PER_PIXEL);
    let sqrt_rays_per_pixel = (rays_per_pixel as f64).sqrt() as u16;

    // error if rays_per_pixel is not a perfect square
    if sqrt_rays_per_pixel * sqrt_rays_per_pixel != rays_per_pixel {
        return Err(js!("rays_per_pixel must be a perfect square"));
    }

    let antialias_method = match args.antialias_method {
        Some(ref s) => raytracer_lib::AntialiasMethod::from_str(s)
            .map_err(|e| js!("Failed to parse antialias method: {:?}", e))?,
        _ => raytracer_lib::AntialiasMethod::Normal,
    };

    // Get data from JS object
    let mut scene_data: HashMap<String, Vec<u8>> = HashMap::new();
    let js_obj = js_sys::Object::from(scene_data_js);

    // Get all the entries from the JS object
    for relative_path in &scene_desc.data_needed {
        if let Ok(Some(array)) = js_sys::Reflect::get(&js_obj, &JsValue::from_str(relative_path))
            .map(|v| v.dyn_into::<js_sys::Uint8Array>().ok())
        {
            let mut bytes = vec![0; array.length() as usize];
            array.copy_to(&mut bytes);
            scene_data.insert(relative_path.clone(), bytes);
        } else {
            return Err(JsValue::from_str(&format!(
                "Missing or invalid data for path: {}",
                relative_path
            )));
        }
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

    let raytracer = RayTracer {
        complete: false,
        next_pixel: (0, 0),
        scene: scene_graph,
        fb: Arc::new(Framebuffer::new(args.width, args.height)),
        sqrt_rays_per_pixel,
        antialias_method,
        context: None,
        texture: None,
        background_texture: None,
    };

    let fb = raytracer.fb.clone();
    let scene = &raytracer.scene;
    let sqrt_rays_per_pixel = raytracer.sqrt_rays_per_pixel;
    let antialias_method = raytracer.antialias_method;
    let width = raytracer.fb.width;
    let height = raytracer.fb.height;
    (0..width).into_par_iter().for_each(move |i| {
        for j in 0..height {
            render_pixel(
                fb.clone(),
                scene,
                sqrt_rays_per_pixel,
                antialias_method,
                i,
                j,
            );
        }
    });

    // Set up WebGL2
    offscreen_canvas.set_width(args.width);
    offscreen_canvas.set_height(args.height);
    let context = {
        #[cfg(debug_assertions)]
        test_webgl2()?;

        let context_attributes = WebGlContextAttributes::new();
        context_attributes.set_alpha(true);
        context_attributes.set_premultiplied_alpha(false);
        context_attributes.set_antialias(true);

        context_attributes.set_power_preference(web_sys::WebGlPowerPreference::HighPerformance);

        let context = if let Some(context) =
            offscreen_canvas.get_context_with_context_options("webgl2", &context_attributes)?
        {
            context.dyn_into::<WebGl2RenderingContext>()?
        } else {
            return Err(js!("Failed to get WebGL2 context"));
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
            vec4 color = texture(tex, texCoord);
            fragColor = color;
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

        // Handle transparency
        context.enable(WebGl2RenderingContext::BLEND);
        context.blend_func(
            WebGl2RenderingContext::SRC_ALPHA,
            WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        context
    };

    // Clear canvas
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    // Create and bind texture
    let texture = context.create_texture();
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

    // Set texture parameters
    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MIN_FILTER,
        WebGl2RenderingContext::NEAREST as i32,
    );
    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MAG_FILTER,
        WebGl2RenderingContext::NEAREST as i32,
    );

    // bind foreground texture
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

    let pixels = framebuffer_to_rgba32f(&raytracer.fb);

    // Upload pixels to texture
    unsafe {
        // View over pixels
        let pixels_view = Float32Array::view(std::slice::from_raw_parts(
            pixels.as_ptr() as *const f32,
            pixels.len() * 4,
        ));

        // Bind texture and upload pixels
        context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA32F as i32,
                raytracer.fb.width as i32,
                raytracer.fb.height as i32,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::FLOAT,
                Some(&pixels_view),
            )?;
    }

    // Draw full-screen quad
    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);

    Ok(())
}

fn framebuffer_to_rgba32f(fb: &Framebuffer) -> Vec<[f32; 4]> {
    // process frame buffer data into pixels
    let pixels_guard = fb.pixels.read().expect("Failed to lock pixels");
    let samples_guard = fb.samples.read().expect("Failed to lock samples");

    pixels_guard
        .iter()
        .zip(samples_guard.iter())
        .map(|(p, &s)| {
            if s > 0 {
                let s = s as f32;
                [
                    (p[0] / s).clamp(0., 1.).powf(1.0 / 2.2),
                    (p[1] / s).clamp(0., 1.).powf(1.0 / 2.2),
                    (p[2] / s).clamp(0., 1.).powf(1.0 / 2.2),
                    1.0,
                ]
            } else {
                [0.0, 0.0, 0.0, 0.0]
            }
        })
        .collect()
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
