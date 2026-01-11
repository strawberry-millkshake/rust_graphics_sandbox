#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: [f32; 4],
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h,
            color: color,
        }
    }
}
