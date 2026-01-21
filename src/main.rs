mod graphics;
mod platform;
mod ui;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

async fn run() {
    let mut platform = platform::Context::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut graphics = graphics::Context::new(&mut platform.window.pwindow).await;
    let mut ui = ui::UiContext::new();

    let red_select_block = ui.new_rec(10.0, 10.0, 20.0, 20.0, ui::colors::RED);
    let green_select_block = ui.new_rec(10.0, 60.0, 20.0, 20.0, ui::colors::GREEN);
    let blue_select_block = ui.new_rec(10.0, 110.0, 20.0, 20.0, ui::colors::BLUE);
    let black_select_block = ui.new_rec(10.0, 160.0, 20.0, 20.0, ui::colors::BLACK);
    let pointer = ui.new_rec(400.0, 400.0, 20.0, 20.0, ui::colors::BLACK);

    while platform.is_open() {
        platform.update();

        let current_color = ui.get_object_color(pointer);
        ui.set_object_position(pointer, platform.mouse_x - 10.0, platform.mouse_y - 10.0);

        if platform.mouse_is_clicked {
            if ui.object_collides_with_position(
                red_select_block,
                platform.mouse_x,
                platform.mouse_y,
            ) {
                ui.set_object_color(pointer, ui::colors::RED);
            } else if ui.object_collides_with_position(
                green_select_block,
                platform.mouse_x,
                platform.mouse_y,
            ) {
                ui.set_object_color(pointer, ui::colors::GREEN);
            } else if ui.object_collides_with_position(
                blue_select_block,
                platform.mouse_x,
                platform.mouse_y,
            ) {
                ui.set_object_color(pointer, ui::colors::BLUE);
            } else if ui.object_collides_with_position(
                black_select_block,
                platform.mouse_x,
                platform.mouse_y,
            ) {
                ui.set_object_color(pointer, ui::colors::BLACK);
            } else {
                ui.new_rec(
                    platform.mouse_x,
                    platform.mouse_y,
                    20.0,
                    20.0,
                    current_color,
                );
            }
        }

        let frame = ui.build_frame(platform.get_size());
        graphics.render(frame);
    }
}

fn main() {
    pollster::block_on(run());
}
