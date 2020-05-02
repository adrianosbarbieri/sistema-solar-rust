extern crate gl;
extern crate glfw;
use glfw::*;

pub fn create_window(
    glfw_ctx: &mut Glfw,
    width: u32,
    height: u32,
    title: &str,
) -> (Window, std::sync::mpsc::Receiver<(f64, WindowEvent)>) {
    glfw_ctx.window_hint(WindowHint::ContextVersion(3, 1));
    glfw_ctx.window_hint(WindowHint::Resizable(false));
    let (mut window, events) = glfw_ctx
        .create_window(width, height, title, WindowMode::Windowed)
        .expect("Failed to create the window");
    window.make_current();
    window.set_key_polling(true);
    glfw_ctx.make_context_current(Some(&window));
    glfw_ctx.set_swap_interval(SwapInterval::Sync(1));
    gl::load_with(|s| window.get_proc_address(s) as *const _);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
    }
    (window, events)
}
