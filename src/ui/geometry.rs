use crate::ui;

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: ui::colors::Color,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32, color: ui::colors::Color) -> Rect {
        Rect {
            x,
            y,
            w,
            h,
            color,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Image {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
