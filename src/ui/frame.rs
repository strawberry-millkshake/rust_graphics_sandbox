use crate::ui::geometry;

pub struct Frame {
    pub rec_vec: Vec<geometry::Rect>,
    pub width: u32,
    pub height: u32,
}

impl Frame {
    pub fn new(rec_vec: Vec<super::geometry::Rect>, (width, height): (u32, u32)) -> Frame {
        Frame {
            rec_vec: rec_vec,
            width: width,
            height: height,
        }
    }
}
