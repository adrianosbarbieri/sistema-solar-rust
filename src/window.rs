use glfw::Action;
use glfw::Key;
use glfw::WindowHint;

pub fn handle_window_events(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

pub fn create_window(
    glfw_ctx: &mut glfw::Glfw,
    width: u32,
    height: u32,
    title: &str,
) -> (
    glfw::Window,
    std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
) {
    glfw_ctx.window_hint(WindowHint::ContextVersionMajor(3));
    glfw_ctx.window_hint(WindowHint::ContextVersionMinor(1));
    glfw_ctx.window_hint(WindowHint::Resizable(false));
    let (mut window, events) = glfw_ctx
        .create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create the window");
    window.set_key_polling(true);
    glfw_ctx.make_context_current(Some(&window));
    glfw_ctx.set_swap_interval(glfw::SwapInterval::Sync(1));
    (window, events)
}
