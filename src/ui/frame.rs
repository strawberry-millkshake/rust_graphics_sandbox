use crate::ui::geometry;

pub struct Frame {
    pub rec_vec: Vec<geometry::Rect>,
}

impl Frame {
    pub fn new() -> Frame {
        let rec_vec = Vec::new();

        Frame { rec_vec: rec_vec }
    }

    pub fn add_rec(&mut self, new_rect: geometry::Rect) {
        self.rec_vec.push(new_rect);
    }
}
