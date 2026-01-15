mod graphics;
mod platform;
mod ui;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

async fn run() {
    let mut platform = platform::Context::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut graphics = graphics::Context::new(&mut platform.window.pwindow).await; //FIX!!!!! WE NEED AN INTERFACE!!!

    let mut ui_context = ui::UiContext::new();

    while platform.is_open() {
        platform.update();

        ui_context.add_rec(
            platform.mouse_x,
            platform.mouse_y,
            50.0,
            50.0,
            ui::colors::WHITE,
        );

        let frame = ui_context.build_frame(platform.get_size());
        graphics.render(frame);
    }
}

fn main() {
    pollster::block_on(run());
}
