use super::rect::Rectangle;
use super::hashgrid::{Box, XY};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Entity {
    pub id: i32,
    pub bounds: Rectangle,
    pub vel: (f64, f64),
    pub keys: HashMap<char, bool>,
}

impl Entity {
    pub fn update_pos(&mut self) {
        let (vx, vy) = self.vel;
        let (x, y) = self.bounds.get_center();
        let size = self.bounds.get_width();
        self.bounds = Rectangle::center_rect(x + vx, y + vy, size, size);
    }

    pub fn stay_in_bounds(&mut self, width: f64, height: f64) {
        let (mut x, mut y) = self.bounds.get_center();
        let size = self.bounds.get_width();
        let half_size = size * 0.5;

        if x - half_size < 0.0 {
            x = half_size;
        } else if x + half_size > width {
            x = width - half_size;
        }

        if y - half_size < 0.0 {
            y = half_size;
        } else if y + half_size > height {
            y = height - half_size;
        }

        self.bounds = Rectangle::center_rect(x, y, size, size);
    }

    pub fn get_bounding_box(&self) -> Box {
        Box::new(self.id, self.pos.x, self.pos.y, self.size)
    }
}
