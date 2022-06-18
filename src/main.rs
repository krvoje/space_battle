use macroquad::prelude::*;
use crate::asteroid::Asteroid;
use crate::bullet::Bullet;
use rand::gen_range;
use crate::spaceship::{Ship, SHIP_HEIGHT};

mod asteroid;
mod spaceship;
mod bullet;

fn wrap_around(v: &Vec2) -> Vec2 {
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > screen_width() {
        vr.x = 0.;
    }
    if vr.x < 0. {
        vr.x = screen_width()
    }
    if vr.y > screen_height() {
        vr.y = 0.;
    }
    if vr.y < 0. {
        vr.y = screen_height()
    }
    vr
}

#[derive(Default)]
struct Game {
    ship1: Ship,
    ship2: Ship,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    sides: u8,
    is_gameover: bool
}

impl Game {
    fn new() -> Game {
        let mut game = Game::default();
        game.reset(3);
        game
    }

    fn reset(&mut self, sides: u8) {
        self.ship1 = Ship::new();
        self.ship2 = Ship::new();
        self.bullets.clear();
        self.asteroids.clear();
        self.sides = sides;
        self.is_gameover = false;
        for _ in 0..self.sides {
            self.add_asteroid();
        }
    }

    fn next_level(&mut self) {
        self.reset(self.sides + 1)
    }

    fn add_asteroid(&mut self) {
        self.asteroids.push(Asteroid::new(gen_range(3, self.sides)))
    }
}

#[macroquad::main("Asteroids")]
async fn main() {
    let mut game = Game::new();
    let mut last_shot = get_time();

    loop {
        if game.is_gameover {
            gameover(&mut game).await;
            continue;
        }
        let frame_t = get_time();

        if is_key_down(KeyCode::Up) {
            game.ship1.accelerate();
        } else {
            game.ship1.retardate();
        }

        if is_key_down(KeyCode::W) {
            game.ship2.accelerate();
        } else {
            game.ship2.retardate();
        }

        if is_key_pressed(KeyCode::Down) && frame_t - last_shot > 0.1 {
            game.bullets.push(game.ship1.fire_bullet(frame_t));
            last_shot = frame_t;
        }
        if is_key_pressed(KeyCode::S) && frame_t - last_shot > 0.1 {
            game.bullets.push(game.ship2.fire_bullet(frame_t));
            last_shot = frame_t;
        }

        if is_key_down(KeyCode::Right) {
            game.ship1.rotate_right();
        } else if is_key_down(KeyCode::Left) {
            game.ship1.rotate_left();
        }

        if is_key_down(KeyCode::D) {
            game.ship2.rotate_right();
        } else if is_key_down(KeyCode::A) {
            game.ship2.rotate_left();
        }

        for bullet in game.bullets.iter_mut() {
            bullet.advance()
        }
        for asteroid in game.asteroids.iter_mut() {
            asteroid.advance();
        }

        game.bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);

        let mut new_asteroids = Vec::new();
        for asteroid in game.asteroids.iter_mut() {
            if (asteroid.position - game.ship1.position).length() < asteroid.size + SHIP_HEIGHT / 3. {
                game.is_gameover = true;
                break;
            }
            if (asteroid.position - game.ship2.position).length() < asteroid.size + SHIP_HEIGHT / 3. {
                game.is_gameover = true;
                break;
            }
            for bullet in game.bullets.iter_mut() {
                if (asteroid.position - bullet.position).length() < asteroid.size {
                    asteroid.collided = true;
                    bullet.collided = true;
                    if asteroid.sides > 3 {
                        new_asteroids.push(Asteroid {
                            position: asteroid.position,
                            velocity: Vec2::new(bullet.velocity.y, -bullet.velocity.x).normalize()
                                * rand::gen_range(1., 3.),
                            rotation: rand::gen_range(0., 360.),
                            rotation_speed: rand::gen_range(-2., 2.),
                            size: asteroid.size * 0.8,
                            sides: asteroid.sides - 1,
                            collided: false,
                        });
                        new_asteroids.push(Asteroid {
                            position: asteroid.position,
                            velocity: Vec2::new(-bullet.velocity.y, bullet.velocity.x).normalize()
                                * rand::gen_range(1., 3.),
                            rotation: rand::gen_range(0., 360.),
                            rotation_speed: rand::gen_range(-2., 2.),
                            size: asteroid.size * 0.8,
                            sides: asteroid.sides - 1,
                            collided: false,
                        })
                    }
                    break;
                }
            }
        }

        game.bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
        game.asteroids.retain(|asteroid| !asteroid.collided);
        game.asteroids.append(&mut new_asteroids);

        if game.asteroids.len() == 0 {
            game.is_gameover = true;
        }

        if game.is_gameover {
            continue;
        }

        clear_background(LIGHTGRAY);
        for bullet in game.bullets.iter() {
            bullet.draw();
        }

        for asteroid in game.asteroids.iter() {
            asteroid.draw();
        }

        game.ship1.draw();
        game.ship2.draw();
        next_frame().await
    }
}

async fn gameover(game: &mut Game) {
    clear_background(LIGHTGRAY);
    let is_win = game.asteroids.is_empty();

    let text = if is_win {
        "You Win!. Press [enter] to play again."
    } else {
        "Game Over. Press [enter] to play again."
    };
    let font_size = 30.;

    let text_size = measure_text(text, None, font_size as _, 1.0);
    draw_text(
        text,
        screen_width() / 2. - text_size.width / 2.,
        screen_height() / 2. - text_size.height / 2.,
        font_size,
        DARKGRAY,
    );

    if is_win {
        game.next_level();
    } else if is_key_down(KeyCode::Enter){
        game.reset(2);
    }

    next_frame().await;

}