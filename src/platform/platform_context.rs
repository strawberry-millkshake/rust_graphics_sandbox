use crate::platform;

pub struct Context {
    pub window: platform::Window,
    pub window_width: u32,
    pub window_height: u32,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

impl Context {
    pub fn new(window_width: u32, window_height: u32) -> Context {
        let window = platform::Window::new(window_width, window_height);

        Context {
            window: window,
            window_width: window_width,
            window_height: window_height,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }

    pub fn is_open(&mut self) -> bool {
        !self.window.pwindow.should_close()
    }

    pub fn update(&mut self) {
        let events = self.window.get_events();
        events.iter().for_each(|x| self.event_match(x));
    }

    pub fn get_size(&mut self) -> (u32, u32) {
        (self.window_width, self.window_height)
    }

    pub fn set_mouse_pos(&mut self, x_pos: f32, y_pos: f32) {
        self.mouse_x = x_pos;
        self.mouse_y = y_pos;
    }

    fn event_match(&mut self, (_time, event): &(f64, glfw::WindowEvent)) {
        match event {
            glfw::WindowEvent::CursorPos(cur_x_pos, cur_y_pos) => {
                self.set_mouse_pos(*cur_x_pos as f32, *cur_y_pos as f32);
            }

            glfw::WindowEvent::MouseButton(glfw::MouseButton::Button1, glfw::Action::Press, _) => {}
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {}

            _ => {}
        }
    }
}
