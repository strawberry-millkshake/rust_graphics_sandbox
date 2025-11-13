
fn glfw_window_with_events(){

    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            "hello world",
            glfw::WindowMode::Windowed,
        )
        .unwrap();

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_pos_polling(true);
    window.make_current();

    while !window.should_close() {

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    state.window.set_should_close(true);
                }
                glfw::WindowEvent::FramebufferSize(_width, _height, ) =>{
                    //event code here
                }
                glfw::WindowEvent::Pos(..) => {
                    state.update_surface();
                    state.resize(state.size);
                }
                _ => {}
            }
        }
    }
}