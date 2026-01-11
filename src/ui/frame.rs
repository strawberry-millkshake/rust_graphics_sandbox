use crate::ui::geometry;

pub struct UiFrame {
    pub rec_vec: Vec<geometry::Rect>,
}

impl UiFrame {
    pub fn new() -> UiFrame {
        let rec_vec = Vec::new();

        UiFrame { rec_vec: rec_vec }
    }

    pub fn add_rec(&mut self, new_rect: geometry::Rect) {
        self.rec_vec.push(new_rect);
    }
}
