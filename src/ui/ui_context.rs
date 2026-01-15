use crate::ui::{frame::Frame, geometry};
pub struct UiContext {
    recs: Vec<super::geometry::Rect>,
}

impl UiContext {
    pub fn new() -> UiContext {
        let recs: Vec<super::geometry::Rect> = Vec::new();
        UiContext { recs: recs }
    }

    pub fn build_frame(&mut self, window_size: (u32, u32)) -> Frame {
        let frame = Frame::new(self.recs.clone(), window_size);
        frame
    }

    pub fn add_rec(&mut self, x: f32, y: f32, w: f32, h: f32, color: super::colors::Color) {
        let new_rec = geometry::Rect::new(x, y, w, h, color);
        self.recs.push(new_rec);
    }
}
