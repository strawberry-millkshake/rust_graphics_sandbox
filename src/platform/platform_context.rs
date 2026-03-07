use crate::platform;

pub struct Context {
    pub window: platform::Window,
    pub window_width: u32,
    pub window_height: u32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_is_clicked: bool,
    time_delta: f64,
    current_time: f64,
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
            mouse_is_clicked: false,
            time_delta: 0.0,
            current_time: 0.0,
        }
    }

    pub fn is_open(&mut self) -> bool {
        !self.window.pwindow.should_close()
    }

    pub fn update(&mut self) {
        let events = self.window.get_events();

        self.time_delta = self.window.glfw.get_time() - self.current_time;
        self.current_time = self.window.glfw.get_time();

        // if we ever needed to control the framerate - this is a pretty insane way of doing it
        //
        //
        // while self.time_delta < 1.0/60.0 {
        //     self.time_delta = self.window.glfw.get_time();
        // }

        events.iter().for_each(|x| self.event_match(x));
    }

    pub fn get_size(&mut self) -> (u32, u32) {
        (self.window_width, self.window_height)
    }

    pub fn set_mouse_pos(&mut self, x_pos: f32, y_pos: f32) {
        self.mouse_x = x_pos;
        self.mouse_y = y_pos;
    }

    pub fn set_mouse_click(&mut self, click_state: bool) {
        self.mouse_is_clicked = click_state;
    }

    fn event_match(&mut self, (_time, event): &(f64, glfw::WindowEvent)) {
        match event {
            glfw::WindowEvent::CursorPos(cur_x_pos, cur_y_pos) => {
                self.set_mouse_pos(*cur_x_pos as f32, *cur_y_pos as f32);
            }
            glfw::WindowEvent::MouseButton(glfw::MouseButton::Button1, glfw::Action::Press, _) => {
                self.set_mouse_click(true);
            }
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {}
            glfw::WindowEvent::MouseButton(
                glfw::MouseButton::Button1,
                glfw::Action::Release,
                _,
            ) => {
                self.set_mouse_click(false);
            }
            glfw::WindowEvent::FramebufferSize(cur_width, cur_height) => {
               self.window_width = *cur_width as u32;
               self.window_height = *cur_height as u32;
            }

            e => {
                println!("{:?}", e);
            }
        }
    }
}
