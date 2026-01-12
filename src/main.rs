mod graphics;
mod platform;
mod ui;

use crate::ui::{Frame, geometry};

async fn run() {
    let mut window = platform::Window::new();
    let mut graphics = graphics::Context::new(&mut window.window).await;
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
        let mut frame = Frame::new();

        let white = [1.0, 1.0, 1.0, 1.0];

        let block = geometry::Rect::new(x_pos, y_pos, 50.0, 50.0, white);
        frame.add_rec(block);

        graphics.render(frame, width, height);
    }
}

fn main() {
    pollster::block_on(run());
}