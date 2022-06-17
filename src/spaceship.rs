use macroquad::prelude::*;
use crate::bullet::Bullet;
use crate::wrap_around;

pub const SHIP_HEIGHT: f32 = 25.;
pub const SHIP_BASE: f32 = 22.;
pub struct Ship {
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            position: Vec2::new(screen_width() / 2., screen_height() / 2.),
            rotation: 0.,
            velocity: Vec2::new(0., 0.),
        }
    }

    pub fn draw(&self) {
        let rotation = self.rotation.to_radians();
        let v1 = Vec2::new(
            self.position.x + rotation.sin() * SHIP_HEIGHT / 2.,
            self.position.y - rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            self.position.x - rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            self.position.y - rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            self.position.x + rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            self.position.y + rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        draw_triangle_lines(v1, v2, v3, 2., BLACK);
    }

    pub fn accelerate(&mut self) {
        let rotation_radians = self.rotation.to_radians();
        let acceleration = Vec2::new(rotation_radians.sin(), -rotation_radians.cos()) / 3.;
        self.advance(acceleration);
    }

    pub fn retardate(&mut self) {
        self.advance(-self.velocity / 10.);
        //self.update_position(Vec2::new(0., 0.));
    }

    pub fn advance(&mut self, acceleration: Vec2) {
        self.velocity += acceleration;
        if self.velocity.length() > 5. {
            self.velocity = self.velocity.normalize() * 5.;
        }
        self.position += self.velocity;
        self.position = wrap_around(&self.position);
    }

    pub fn rotate_right(&mut self) {
        self.rotation += 5.;
    }

    pub fn rotate_left(&mut self) {
        self.rotation -= 5.;
    }

    pub fn fire_bullet(&self, frame_t: f64) -> Bullet {
        let rotation_radians = self.rotation.to_radians();
        let rotation_vector = Vec2::new(rotation_radians.sin(), -rotation_radians.cos());
        Bullet {
            position: self.position + rotation_vector * SHIP_HEIGHT / 2.,
            velocity: rotation_vector * 7.,
            shot_at: frame_t,
            collided: false,
        }
    }
}