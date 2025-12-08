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
                    x: player.rot.cos(),
                    y: player.rot.sin(),
                }
        } else {
            Vec2 { x: 0.0, y: 0.0 }
        };

        let turn = 5.0
            * (if is_key_down(KeyCode::Left) {
                dbg!("Left");
                -1.0
            } else {
                0.0
            } + if is_key_down(KeyCode::Right) {
                dbg!("Right");
                1.0
            } else {
                0.0
            });

        let current_tick = get_time();
        let dt = current_tick - last_tick;
        last_tick = current_tick;

        player.update(boost, turn, dt as f32);

        player.draw();

        next_frame().await;
    }
}

struct Player {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    ome: f32,
}

impl Player {
    fn get_shape(&self) -> (Vec2, Vec2, Vec2) {
        let radius = 50.0;
        let v1 = self.pos
            - radius
                * Vec2 {
                    x: self.rot.cos(),
                    y: self.rot.sin(),
                };
        let v2 = self.pos
            - radius / 2.0
                * Vec2 {
                    x: (self.rot + 2.0 * PI / 3.0).cos(),
                    y: (self.rot + 2.0 * PI / 3.0).sin(),
                };
        let v3 = self.pos
            - radius / 2.0
                * Vec2 {
                    x: (self.rot + 4.0 * PI / 3.0).cos(),
                    y: (self.rot + 4.0 * PI / 3.0).sin(),
                };
        return (v1, v2, v3);
    }

    fn draw(&self) -> () {
        let (v1, v2, v3) = self.get_shape();
        draw_triangle_lines(v1, v2, v3, 1.0, DARKGRAY);
    }

    fn update(&mut self, boost: Vec2, turn: f32, dt: f32) -> () {
        let lin_fric = -1.0 * self.vel;
        let acc = boost + lin_fric;
        self.vel += acc * dt;
        self.pos += self.vel * dt;

        let ang_fric = -1.0 * self.ome;
        let ang_acc = turn + ang_fric;
        self.ome += ang_acc * dt;
        self.rot += self.ome * dt;

        // warping
        {
            let x = &mut self.pos.x;
            let y = &mut self.pos.y;
            if *x < 0.0 {
                *x = screen_width();
            } else if *x > screen_width() {
                *x = 0.0;
            };
            if *y < 0.0 {
                *y = screen_height();
            } else if *y > screen_height() {
                *y = 0.0;
            };
        }
    }
}

impl Default for Player {
    fn default() -> Player {
        Player {
            pos: Vec2 {
                x: screen_width() / 2.0,
                y: screen_height() / 2.0,
            },
            vel: Vec2 { x: 0.0, y: -100.0 },
            rot: PI / 2.0,
            ome: 0.0,
        }
    }
}
