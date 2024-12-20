use std::f32::consts::PI;

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub angle: f32
}

impl Point {
    pub fn rotate(&mut self, delta: f32) {
        self.angle += delta;
        while self.angle < 0f32 {
            self.angle += 2f32 * PI;
        }
        while self.angle >= 2f32 * PI {
            self.angle -= 2f32 * PI;
        }
    }
}