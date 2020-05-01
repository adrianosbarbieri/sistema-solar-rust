extern crate glfw;

use glfw::*;

mod camera;
mod mesh;
mod transform;
mod window;
mod shader;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = window::create_window(&mut glfw, 800, 600, "Asd");
    let camera = camera::new_camera(
        glm::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        glm::radians(90.0),
        800.0 / 600.0,
        0.01,
        4000.0,
    );

    let mut obj = mesh::create_mesh_from_file("untitled.obj");
    let mut mesh = mesh::init_mesh(&mut obj);
    while !window.should_close() {
        glfw.poll_events();
        mesh::draw(&mut mesh);
        for (_, event) in glfw::flush_messages(&events) {
            window::handle_window_events(&mut window, event);
        }
        window.swap_buffers();
    }
    mesh::delete_mesh(&mesh);
}
