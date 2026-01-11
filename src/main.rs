// use glfw::{Context};

mod graphics;
mod window;

async fn run() {
    let mut window = window::window::Window::new();
    let mut graphics = graphics::graphics::Graphics::new(&mut window.window).await;
    let mut x_pos: f32 = 0.0;
    let mut y_pos: f32 = 0.0;

    while !window.window.should_close() {
        window.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&window.events) {
            match event {
                glfw::WindowEvent::CursorPos(cur_x_pos, cur_y_pos) => {
                    x_pos = cur_x_pos as f32;
                    y_pos = cur_y_pos as f32;
                }
                glfw::WindowEvent::MouseButton(
                    glfw::MouseButton::Button1,
                    glfw::Action::Press,
                    _,
                ) => {}
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.window.set_should_close(true);
                }

                _ => {}
            }
        }

        let (height, width) = window.window.get_size();
        graphics.render(height, width, x_pos, y_pos);
    }
}

fn main() {
    pollster::block_on(run());
}