use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::time::Duration;

use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;
use gl::types::{GLfloat, GLsizei};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod shader;

use shader::Shader;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const FLOAT_NUM: usize = 3;
const VERTEX_NUM: usize = 3;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);
        let (major, minor) = gl_attr.context_version();
        println!("OK: init OpenGL: version={}.{}", major, minor);
    }

    let window = video_subsystem
        .window("SDL", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");

    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;

    // set buffer
    #[rustfmt::skip]
    let buffer_array: [f32; BUF_LEN] = [
        -1.0, -1.0, 0.0,
        1.0, -1.0, 0.0,
        0.0, 1.0, 0.0,
    ];

    unsafe {
        // create vertex array and vertex buffer
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (buffer_array.len() * mem::size_of::<GLfloat>()) as isize,
            &buffer_array as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        // set attribute pointer
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );

        // unset
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        unsafe {
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // clear screen
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // init matrice for model, view and projection
            let model_matrix = Matrix4::identity();
            let view_matrix = Matrix4::look_at(
                Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 5.0,
                },
                Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            );
            let projection_matrix: Matrix4 = perspective(
                cgmath::Deg(45.0f32),
                WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                0.1,
                100.0,
            );

            // shader use matrices
            shader.use_program();
            shader.set_mat4(c_str!("aModel"), &model_matrix);
            shader.set_mat4(c_str!("aView"), &view_matrix);
            shader.set_mat4(c_str!("aProjection"), &projection_matrix);

            // render triangle with vertex array
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, VERTEX_NUM as i32);
            gl::BindVertexArray(0);

            window.gl_swap_window();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
