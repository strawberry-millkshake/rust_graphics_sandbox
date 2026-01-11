use glfw::{Context, WindowEvent, fail_on_errors};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new() -> Window {
        let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

        let (mut window, events) = glfw
            .create_window(
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                "beee booo",
                glfw::WindowMode::Windowed,
            )
            .unwrap();

        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.make_current();

        Window {
            glfw: glfw,
            window: window,
            events: events,
        }
    }
}
