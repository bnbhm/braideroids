use std::f32::consts::PI;

use macroquad::prelude::*;

#[macroquad::main("Braideroids : Asteroids = Braid;")]
async fn main() {
    let mut player: Player = Default::default();
    let mut last_tick = get_time();
    loop {
        clear_background(GRAY);

        let boost = if is_key_down(KeyCode::Up) {
            -1000.0
                * Vec2 {
                    x: player.dir.cos(),
                    y: player.dir.sin(),
                }
        } else {
            Vec2 { x: 0.0, y: 0.0 }
        };

        let turn = if is_key_down(KeyCode::Left) {
            -0.05
        } else {
            0.0
        } + if is_key_down(KeyCode::Right) {
            0.05
        } else {
            0.0
        };

        let current_tick = get_time();
        let dt = current_tick - last_tick;
        last_tick = current_tick;

        player.update(turn, boost, dt);

        player.draw();

        next_frame().await;
    }
}

struct Player {
    pos: Vec2,
    dir: f32,
    vel: Vec2,
}

impl Player {
    fn get_shape(&self) -> (Vec2, Vec2, Vec2) {
        let radius = 50.0;
        let v1 = self.pos
            - radius
                * Vec2 {
                    x: self.dir.cos(),
                    y: self.dir.sin(),
                };
        let v2 = self.pos
            - radius / 2.0
                * Vec2 {
                    x: (self.dir + 2.0 * PI / 3.0).cos(),
                    y: (self.dir + 2.0 * PI / 3.0).sin(),
                };
        let v3 = self.pos
            - radius / 2.0
                * Vec2 {
                    x: (self.dir + 4.0 * PI / 3.0).cos(),
                    y: (self.dir + 4.0 * PI / 3.0).sin(),
                };
        return (v1, v2, v3);
    }

    fn draw(&self) -> () {
        let (v1, v2, v3) = self.get_shape();
        draw_triangle(v1, v2, v3, DARKGRAY);
    }

    fn update(&mut self, turn: f32, boost: Vec2, dt: f64) -> () {
        let acc = boost - self.vel; // friction

        self.pos += self.vel * dt as f32;
        self.vel += acc * dt as f32;
        self.dir += turn;
    }
}

impl Default for Player {
    fn default() -> Player {
        Player {
            pos: Vec2 {
                x: screen_width() / 2.0,
                y: screen_height() / 2.0,
            },
            dir: PI / 2.0,
            vel: Vec2 { x: 0.0, y: -100.0 },
        }
    }
}
