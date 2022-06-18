use macroquad::prelude::*;
use crate::wrap_around;

#[derive(Default)]
pub struct Asteroid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub size: f32,
    pub sides: u8,
    pub collided: bool,
}

impl Asteroid {
    pub fn new(sides: u8) -> Asteroid {
        let screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
        Asteroid {
            position: screen_center
                + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
                * screen_width().min(screen_height())
                / 2.,
            velocity: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
            rotation: 0.,
            rotation_speed: rand::gen_range(-2., 2.),
            size: screen_width().min(screen_height()) / 10.,
            sides: sides,
            collided: false,
        }
    }
    
    pub fn draw(&self) {
        draw_poly_lines(
            self.position.x,
            self.position.y,
            self.sides,
            self.size,
            self.rotation,
            2.,
            BLACK,
        )
    }
    
    pub fn advance(&mut self) {
        self.position += self.velocity;
        self.position = wrap_around(&self.position);
        self.rotation += self.rotation_speed;
    }
}