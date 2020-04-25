mod mesh;
mod window;

fn main() {
    let _obj = mesh::create_mesh_from_file("./ball.obj");
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = window::create_window(&mut glfw, 800, 600, "Asd");
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            window::handle_window_events(&mut window, event)
        }
    }
}
