use macroquad::prelude::*;

pub struct Bullet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub shot_at: f64,
    pub collided: bool,
}

impl Bullet {
    pub fn advance(&mut self) {
        self.position += self.velocity;
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 2., BLACK);
    }
}