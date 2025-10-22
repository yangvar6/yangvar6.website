use js_sys::Date;
use leptos::html::Canvas;
use leptos::prelude::*;
use leptos::*;
use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext as WebGl, WebGlProgram, WebGlShader};

// Assume shader files are included as strings (replace with actual shader code)
const MAZE_SHADER_VERT: &'static str = include_str!("../assets/maze_shader.vert");
const MAZE_SHADER_FRAG: &'static str = include_str!("../assets/maze_shader.frag");

#[component]
pub fn BackgroundCanvas() -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();

    // Initialize WebGL context
    Effect::new(move |_| {
        // skip for now
        return;

        let Some(canvas) = canvas_ref.get() else {
            leptos::logging::error!("Canvas not found");
            return;
        };

        let gl = canvas
            .get_context("webgl2")
            .ok()
            .flatten()
            .and_then(|ctx| ctx.dyn_into::<WebGl>().ok())
            .expect("Failed to create WebGL2 context!");

        // Configure canvas (equivalent to gl={{ antialias: false, depth: false }})
        gl.disable(WebGl::DEPTH_TEST);

        // Set up orthographic projection (equivalent to camera={{ position: [0, 0, 100], zoom:
        // 100 }})
        let width = canvas.width() as f32;
        let height = canvas.height() as f32;

        // Create shader program
        let Ok(vert_shader) = compile_shader(&gl, WebGl::VERTEX_SHADER, MAZE_SHADER_VERT) else {
            std::hint::cold_path();
            leptos::logging::error!("Failed to compile vertex shader");
            return;
        };
        let Ok(frag_shader) = compile_shader(&gl, WebGl::FRAGMENT_SHADER, MAZE_SHADER_FRAG) else {
            std::hint::cold_path();
            leptos::logging::error!("Failed to compile fragment shader");
            return;
        };

        let program =
            link_program(&gl, &vert_shader, &frag_shader).expect("Failed to link shader program");
        gl.use_program(Some(&program));

        // Set up uniforms
        let u_time = gl.get_uniform_location(&program, "uTime");
        let u_resolution = gl.get_uniform_location(&program, "uResolution");

        let vertices: &[f32] = &[
            -15.0, -15.0, 0.0, // Bottom-left
            15.0, -15.0, 0.0, // Bottom-right
            15.0, 15.0, 0.0, // Top-right
            -15.0, 15.0, 0.0, // Top-left
        ];

        let vertex_buffer = gl.create_buffer().expect("Failed to create vertex buffer");
        gl.bind_buffer(WebGl::ARRAY_BUFFER, Some(&vertex_buffer));
        let vertices_array = js_sys::Float32Array::new_with_length(vertices.len() as u32);
        vertices_array.copy_from(vertices);
        gl.buffer_data_with_array_buffer_view(
            WebGl::ARRAY_BUFFER,
            &vertices_array,
            WebGl::STATIC_DRAW,
        );

        let indices: &[u16] = &[0, 1, 2, 0, 2, 3];

        let index_buffer = gl.create_buffer().expect("Failed to create index buffer");
        gl.bind_buffer(WebGl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        let indices_array = js_sys::Uint16Array::new_with_length(indices.len() as u32);
        indices_array.copy_from(indices);
        gl.buffer_data_with_array_buffer_view(
            WebGl::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            WebGl::STATIC_DRAW,
        );

        let position = gl.get_attrib_location(&program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 3, WebGl::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        // Animation loop (equivalent to useFrame)
        let time = Rc::new(RefCell::new(0.0));
        let gl = Rc::new(gl);
        let program = Rc::new(program);
        let canvas = Rc::new(canvas);

        let mut render_loop = MaybeUninit::<Closure<dyn FnMut()>>::uninit();
        let render_loop_ref = unsafe { render_loop.as_mut_ptr().as_ref().unwrap_unchecked() };
        render_loop.write(Closure::new(move || {
            let now = Date::now() / 1000.0;
            *time.borrow_mut() = now as f32;

            let gl = gl.clone();
            let program = program.clone();
            let canvas = canvas.clone();

            gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(WebGl::COLOR_BUFFER_BIT);

            // Update uniforms
            if let Some(loc) = u_time.as_ref() {
                gl.uniform1f(Some(loc), *time.borrow());
            }
            if let Some(loc) = u_resolution.as_ref() {
                gl.uniform2f(Some(loc), width, height);
            }

            // Draw the plane
            gl.draw_elements_with_i32(WebGl::TRIANGLES, 6, WebGl::UNSIGNED_SHORT, 0);

            // Request next frame
            web_sys::window()
                .unwrap()
                .request_animation_frame(render_loop_ref.as_ref().unchecked_ref())
                .unwrap();
        }));

        // let render_loop: Closure<dyn FnMut()> = );

        // Start animation loop
        let closure: &js_sys::Function =
            unsafe { render_loop.assume_init_ref() }.as_ref().unchecked_ref();
        web_sys::window().unwrap().request_animation_frame(closure).unwrap();
        // closure.forget(); // Prevent deallocation
    });

    view! {
        <canvas
            node_ref=canvas_ref
            class="w-full h-full"
            width="800"
            height="600"
        />
    }
}

// Helper functions for WebGL setup
fn compile_shader(gl: &WebGl, shader_type: u32, source: &str) -> Result<WebGlShader, Box<str>> {
    let shader = gl.create_shader(shader_type).ok_or("Unable to create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    let Some(_compiled) = gl.get_shader_parameter(&shader, WebGl::COMPILE_STATUS).as_bool() else {
        std::hint::cold_path();
        return Err(gl
            .get_shader_info_log(&shader)
            .map(String::into_boxed_str)
            .unwrap_or(Box::from("Unknown shader error")));
    };

    Ok(shader)
}

fn link_program(
    gl: &WebGl,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, Box<str>> {
    let program = gl.create_program().ok_or("Unable to create program")?;
    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    let Some(_linked) = gl.get_program_parameter(&program, WebGl::LINK_STATUS).as_bool() else {
        std::hint::cold_path();
        return Err(gl
            .get_program_info_log(&program)
            .map(String::into_boxed_str)
            .unwrap_or(Box::from("Unknown program error")));
    };

    Ok(program)
}
