use glfw::{Context, WindowEvent};

pub struct Window {
    pub glfw: glfw::Glfw,
    pub pwindow: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(window_width: u32, window_height: u32) -> Window {
        let mut glfw = glfw::init_no_callbacks().unwrap();

        glfw.window_hint(glfw::WindowHint::Resizable(true));

        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (mut window, events) = glfw
            .create_window(
                window_width,
                window_height,
                "beee booo",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to make glfw window - STRWB");

        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.make_current();
        window.set_framebuffer_size_polling(true);

        Window {
            glfw: glfw,
            pwindow: window,
            events: events,
        }
    }

    pub fn get_events(&mut self) -> Vec<(f64, glfw::WindowEvent)> {
        self.glfw.poll_events();
        glfw::flush_messages(&self.events).collect()
    }
}
