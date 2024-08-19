use nalgebra_glm::{Vec2, Rot3};

pub struct Camera {
    pub pos: Vec2,
    pub angle: f32,
}

impl Camera {
    pub fn move_forward(&mut self, speed: f32) {
        self.pos += Vec2::new(speed * self.angle.cos(), speed * self.angle.sin());
    }

    pub fn move_backward(&mut self, speed: f32) {
        self.pos -= Vec2::new(speed * self.angle.cos(), speed * self.angle.sin());
    }

    pub fn rotate(&mut self, delta: f32) {
        self.angle += delta;
    }
}
