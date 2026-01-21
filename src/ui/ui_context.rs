use crate::ui::frame::Frame;
use std::collections::HashMap;

pub struct UiContext {
    pub elements: HashMap<u32, super::geometry::Rect>,
}

impl UiContext {
    pub fn new<'a>() -> UiContext {
        let elements: HashMap<u32, super::geometry::Rect> = HashMap::new();
        UiContext { elements: elements }
    }

    pub fn build_frame(&mut self, window_size: (u32, u32)) -> Frame {
        let draw_list: Vec<super::geometry::Rect> = self.elements.iter().map(|x| *(x.1)).collect();
        let frame = Frame::new(draw_list, window_size);
        frame
    }

    pub fn new_rec(&mut self, x: f32, y: f32, w: f32, h: f32, color: super::colors::Color) -> u32 {
        let new_rec = super::geometry::Rect::new(x, y, w, h, color);
        let id = self.elements.len() as u32;
        self.elements.insert(id, new_rec.clone());
        id
    }

    pub fn set_object_position(&mut self, object_id: u32, x: f32, y: f32) {
        let object: &mut super::geometry::Rect = self.elements.get_mut(&object_id).unwrap();
        object.x = x;
        object.y = y;
    }

    pub fn _get_object_position(&mut self, object_id: u32) -> (f32, f32, f32, f32) {
        let object: &super::geometry::Rect = self.elements.get(&object_id).unwrap();
        (object.x, object.y, object.w, object.h)
    }

    pub fn object_collides_with_position(&mut self, object_id: u32, x: f32, y: f32) -> bool {
        let object: &super::geometry::Rect = self.elements.get(&object_id).unwrap();
        if object.x < x && x < object.x + object.w && object.y < y && y < object.y + object.h {
            return true;
        }
        false
    }

    pub fn set_object_color(&mut self, object_id: u32, color: super::colors::Color) {
        let object: &mut super::geometry::Rect = self.elements.get_mut(&object_id).unwrap();
        object.color = color;
    }

    pub fn get_object_color(&mut self, object_id: u32) -> super::colors::Color {
        let object: &super::geometry::Rect = self.elements.get(&object_id).unwrap();
        object.color
    }
}
