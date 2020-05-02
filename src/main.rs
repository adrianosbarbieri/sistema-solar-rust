extern crate glfw;

use glfw::*;

mod camera;
mod mesh;
mod shader;
mod texture;
mod transform;
mod window;

use std::f32::consts::PI;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = window::create_window(&mut glfw, 800, 600, "Asd");
    let mut camera = camera::Camera::new(
        cgmath::Point3::<f32> {
            x: 0.0,
            y: 0.0,
            z: -300.0,
        },
        PI / 2.0,
        1600.0 / 900.0,
        0.01,
        4000.0,
    );
    let mut obj = mesh::create_mesh_from_file("untitled.obj");
    let mut mesh = mesh::init_mesh(&mut obj);
    let shader1 = shader::Shader::new("shader");
    let texture = texture::Texture::new("jupiter.jpg");
    let mut transform = transform::Transform {
        pos: cgmath::vec3(0.0, 0.0, 0.0),
        rot: cgmath::vec3(0.0, 0.0, 0.0),
        scale: cgmath::vec3(1.0, 1.0, 1.0),
        scale_val: 1.0,
    };

    unsafe {
        while !window.should_close() {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    }
                    glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                        camera.position = camera.position + (10.0 * camera.forward);
                    }
                    glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                        camera.position = camera.position - (10.0 * camera.forward);
                    }
                    glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                        camera.position = camera.position - (camera.forward);
                    }
                    glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                        camera.position = camera.position - (camera.forward);
                    }
                    _ => {}
                }
            }

            transform.pos.x = 0.0;
            transform.pos.y = 0.0;
            transform.pos.z = 0.0;
            transform.scale = cgmath::vec3(1.0, 1.0, 1.0);
            transform.scale_val = 10.0;
            transform.rot = transform.rot + cgmath::vec3(1.0, 1.0, 0.0);
            shader1.bind();
            shader1.update(&transform, &camera);
            texture.bind();
            mesh::draw(&mut mesh);

            window.swap_buffers();
            glfw.poll_events();
        }
    }
    texture.destroy();
    shader1.destroy();
    mesh::delete_mesh(&mesh);
}
