use std::f32::consts::PI;

use macroquad::prelude::*;

#[macroquad::main("Braideroids : Asteroids = Braid;")]
async fn main() {
    let mut player: Player = Default::default();
    let mut asteroid = Asteroid {
        body: Body {
            pos: Vec2 { x: 100.0, y: 100.0 },
            vel: Vec2 { x: 30.0, y: 10.0 },
            rot: PI,
            ome: 10.0,
        },
        sides: 6,
        size: 100.0,
    };
    let mut asteroid2 = Asteroid {
        body: Body {
            pos: Vec2 { x: 500.0, y: 100.0 },
            vel: Vec2 { x: 30.0, y: 10.0 },
            rot: PI,
            ome: -3.0,
        },
        sides: 4,
        size: 50.0,
    };
    let mut last_tick = get_time();
    loop {
        clear_background(GRAY);

        let boost = if is_key_down(KeyCode::Up) {
            -1000.0
                * Vec2 {
                    x: player.body.rot.cos(),
                    y: player.body.rot.sin(),
                }
        } else {
            Vec2 { x: 0.0, y: 0.0 }
        };

        let turn = 5.0
            * (if is_key_down(KeyCode::Left) {
                // dbg!("Left");
                -1.0
            } else {
                0.0
            } + if is_key_down(KeyCode::Right) {
                // dbg!("Right");
                1.0
            } else {
                0.0
            });

        let current_tick = get_time();
        let dt = current_tick - last_tick;
        last_tick = current_tick;

        player.body.update(boost, turn, dt as f32);
        asteroid
            .body
            .update(Vec2 { x: 0.0, y: 0.0 }, 0.0, dt as f32);

        asteroid2
            .body
            .update(Vec2 { x: 0.0, y: 0.0 }, 0.0, dt as f32);

        player.draw();
        asteroid.draw();

        asteroid2.draw();

        next_frame().await;
    }
}

// CONSTANTS
const LINE_THICKNESS: f32 = 2.0;
const LINE_COLOR: Color = DARKBLUE;

struct Player {
    body: Body,
}

impl Player {
    fn get_shape(&self) -> (Vec2, Vec2, Vec2) {
        let radius = 50.0;
        let v1 = self.body.pos
            - radius
                * Vec2 {
                    x: self.body.rot.cos(),
                    y: self.body.rot.sin(),
                };
        let v2 = self.body.pos
            - radius / 2.0
                * Vec2 {
                    x: (self.body.rot + 2.0 * PI / 3.0).cos(),
                    y: (self.body.rot + 2.0 * PI / 3.0).sin(),
                };
        let v3 = self.body.pos
            - radius / 2.0
                * Vec2 {
                    x: (self.body.rot + 4.0 * PI / 3.0).cos(),
                    y: (self.body.rot + 4.0 * PI / 3.0).sin(),
                };
        return (v1, v2, v3);
    }
}

impl Default for Player {
    fn default() -> Player {
        Player {
            body: Body {
                pos: Vec2 {
                    x: screen_width() / 2.0,
                    y: screen_height() / 2.0,
                },
                vel: Vec2 { x: 0.0, y: -100.0 },
                rot: PI / 2.0,
                ome: 0.0,
            },
        }
    }
}

struct Asteroid {
    body: Body,
    sides: u8,
    size: f32,
}

impl Asteroid {}

struct Body {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    ome: f32,
}

impl Body {
    fn update(&mut self, boost: Vec2, turn: f32, dt: f32) -> () {
        let lin_fric = if self.vel.length() > 100.0 {
            -1.0 * self.vel
        } else {
            Vec2 { x: 0.0, y: 0.0 }
        };
        let acc = boost + lin_fric;
        self.vel += acc * dt;
        self.pos += self.vel * dt;

        let ang_fric = if self.ome.abs() > 3.0 {
            -1.0 * self.ome
        } else {
            0.0
        };
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

trait Draw {
    fn draw(&self) -> ();
}

impl Draw for Player {
    fn draw(&self) -> () {
        let (v1, v2, v3) = self.get_shape();
        draw_triangle_lines(v1, v2, v3, LINE_THICKNESS, LINE_COLOR);
    }
}

impl Draw for Asteroid {
    fn draw(&self) -> () {
        draw_poly_lines(
            self.body.pos.x,
            self.body.pos.y,
            self.sides,
            self.size,
            self.body.rot.to_degrees(),
            LINE_THICKNESS,
            LINE_COLOR,
        );
    }
}
