use std::f32::consts::PI;

pub struct Point {
    pub pos: (f32, f32),
    pub angle: f32,
    pub dir: (f32, f32),
}

impl Point {
    pub fn new(initial_pos: (f32, f32), initial_angle: f32) -> Self {
        Point {
            pos: initial_pos,
            angle: initial_angle,
            dir: (initial_angle * initial_angle.cos(), initial_angle * initial_angle.sin())
        }
    }

    pub fn rotate(&mut self, delta: f32) {
        self.angle += delta;
        while self.angle < 0f32 {
            self.angle += 2f32 * PI;
        }
        while self.angle >= 2f32 * PI {
            self.angle -= 2f32 * PI;
        }

        // updating the direction as well
        self.dir = (
            self.dir.0 * self.angle.cos(),
            self.dir.0 * self.angle.sin()
        );
    }
}