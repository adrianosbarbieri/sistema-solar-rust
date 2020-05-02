extern crate glfw;

use glfw::*;

mod camera;
mod mesh;
mod shader;
mod texture;
mod transform;
mod window;

use cgmath::*;
use std::f32;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = window::create_window(&mut glfw, 1600, 900, "Asd");
    let mut obj = mesh::create_mesh_from_file("untitled.obj");
    let mesh = mesh::init_mesh(&mut obj);
    let shader1 = shader::Shader::new("shader");
    let shader2 = shader::Shader::new("shader2");
    let jupiter = texture::Texture::new("jupiter.jpg");
    let sun = texture::Texture::new("sun.jpg");
    let space = texture::Texture::new("space.jpg");
    let mut camera = camera::Camera::new(
        cgmath::Point3::<f32> {
            x: 0.0,
            y: 0.0,
            z: -300.0,
        },
        90.0,
        1600.0 / 900.0,
        0.01,
        4000.0,
    );

    let mut last_x: f32 = 1600.0 / 2.0;
    let mut last_y: f32 = 900.0 / 2.0;
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;
    let mut first_mouse = true;

    /* mouse callback */
    let mut mouse_callback = |xpos: f32, ypos: f32| -> cgmath::Vector3<f32> {
        if first_mouse == true {
            last_x = xpos;
            last_y = ypos;
            first_mouse = false;
        }
        let mut xoffset = xpos - last_x;
        let mut yoffset = last_y - ypos;
        last_x = xpos;
        last_y = ypos;
        let sensitivity = 0.0005;
        xoffset += sensitivity;
        yoffset += sensitivity;
        yaw += xoffset;
        pitch += yoffset;
        if pitch > 89.0 {
            pitch = 89.0;
        }
        if pitch < -89.0 {
            pitch = -89.0;
        }
        let mut front: cgmath::Vector3<f32> = cgmath::vec3(0.0, 0.0, 0.0);
        front.x = yaw.to_radians().cos() * pitch.to_radians().cos();
        front.y = pitch.to_radians().sin();
        front.z = yaw.to_radians().sin() * pitch.to_radians().cos();
        front.normalize()
    };

    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_cursor_pos_polling(true);

    let mut transform = transform::Transform {
        pos: cgmath::vec3(0.0, 0.0, 0.0),
        rot: cgmath::vec3(0.0, 0.0, 0.0),
        scale: cgmath::vec3(1.0, 1.0, 1.0),
        scale_val: 1.0,
    };
    let mut counter: f64 = 0.0;
    let mut last_counter: f64;
    unsafe {
        while !window.should_close() {
            last_counter = counter;
            counter = glfw.get_time();
            let diff = counter - last_counter;
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            glfw.poll_events();

            if window.get_key(Key::W) != Action::Release {
                camera.position = camera.position + (200.0 * diff as f32 * camera.forward);
            }

            if window.get_key(Key::S) != Action::Release {
                camera.position = camera.position - (200.0 * diff as f32 * camera.forward);
            }

            if window.get_key(Key::A) != Action::Release {
                camera.position =
                    camera.position - 200.0 * diff as f32 * (camera.forward.cross(camera.up));
            }

            if window.get_key(Key::D) != Action::Release {
                camera.position =
                    camera.position + 200.0 * diff as f32 * (camera.forward.cross(camera.up));
            }
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    }
                    glfw::WindowEvent::CursorPos(pos_x, pos_y) => {
                        camera.forward = mouse_callback(pos_x as f32, pos_y as f32);
                    }
                    _ => {}
                }
            }

            gl::Disable(gl::DEPTH_TEST);
            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::BACK);
            transform.pos = cgmath::vec3(50.0, 0.0, 0.0);
            transform.scale = cgmath::vec3(500.0, 500.0, 500.0);
            transform.scale_val = 500.0;
            transform.rot = cgmath::vec3(90.0, 0.0, 0.0);
            shader1.bind();
            shader1.update(&transform, &camera);
            space.bind();
            mesh.draw();
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BACK);

            transform.pos = cgmath::vec3(0.0, 0.0, 0.0);
            transform.scale = cgmath::vec3(1.0, 1.0, 1.0);
            transform.scale_val = 10.0;
            transform.rot = transform.rot + cgmath::vec3(1.0, 1.0, 0.0);
            shader2.bind();
            shader2.update(&transform, &camera);
            sun.bind();
            mesh.draw();

            window.swap_buffers();
        }
    }
    sun.destroy();
    jupiter.destroy();
    space.destroy();
    shader2.destroy();
    shader1.destroy();
    mesh.destroy();
}
