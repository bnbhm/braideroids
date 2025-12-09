use std::f32::consts::PI;

use macroquad::prelude::*;

#[macroquad::main("Braideroids : Asteroids = Braid;")]
async fn main() {
    let mut ship: Ship = Default::default();
    let asteroids = &mut [
        &mut Asteroid {
            body: Body {
                lin_pos: Vec2 { x: 100.0, y: 100.0 },
                lin_vel: Vec2 { x: 100.0, y: 001.0 },
                lin_acc: Vec2 { x: 0.0, y: 0.0 },
                ang_pos: PI,
                ang_vel: 2.0,
                ang_acc: 0.0,
            },
            sides: 6,
            size: 100.0,
        },
        &mut Asteroid {
            body: Body {
                lin_pos: Vec2 { x: 200.0, y: 300.0 },
                lin_vel: Vec2 {
                    x: -200.0,
                    y: 1000.0,
                },
                lin_acc: Vec2 { x: 0.0, y: 0.0 },
                ang_pos: PI,
                ang_vel: 15.0,
                ang_acc: 0.0,
            },
            sides: 4,
            size: 50.0,
        },
    ];
    let mut last_tick = get_time();
    loop {
        clear_background(LIGHTGRAY);

        let current_tick = get_time();
        let dt = current_tick - last_tick;
        last_tick = current_tick;

        ship.update(dt as f32);
        asteroids
            .iter_mut()
            .for_each(|asteroid| asteroid.update(dt as f32));

        ship.draw();
        asteroids.iter().for_each(|asteroid| asteroid.draw());

        next_frame().await;
    }
}

// CONSTANTS
const LINE_THICKNESS: f32 = 2.0;
const LINE_COLOR: Color = DARKBLUE;

trait Draw {
    fn draw(&self) -> ();
}

trait Update {
    fn update(&mut self, dt: f32) -> ();
}

struct Ship {
    body: Body,
}

impl Ship {
    fn get_shape(&self) -> (Vec2, Vec2, Vec2) {
        let radius = 50.0;
        let v1 = self.body.lin_pos
            - radius
                * Vec2 {
                    x: self.body.ang_pos.cos(),
                    y: self.body.ang_pos.sin(),
                };
        let v2 = self.body.lin_pos
            - radius / 2.0
                * Vec2 {
                    x: (self.body.ang_pos + 2.0 * PI / 3.0).cos(),
                    y: (self.body.ang_pos + 2.0 * PI / 3.0).sin(),
                };
        let v3 = self.body.lin_pos
            - radius / 2.0
                * Vec2 {
                    x: (self.body.ang_pos + 4.0 * PI / 3.0).cos(),
                    y: (self.body.ang_pos + 4.0 * PI / 3.0).sin(),
                };
        return (v1, v2, v3);
    }
}

impl Default for Ship {
    fn default() -> Ship {
        Ship {
            body: Body {
                lin_pos: Vec2 {
                    x: screen_width() / 2.0,
                    y: screen_height() / 2.0,
                },
                lin_vel: Vec2 { x: 0.0, y: 0.0 },
                lin_acc: Vec2 { x: 0.0, y: 0.0 },
                ang_pos: PI / 2.0,
                ang_vel: 0.0,
                ang_acc: 0.0,
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
    lin_pos: Vec2,
    lin_vel: Vec2,
    lin_acc: Vec2,

    ang_pos: f32,
    ang_vel: f32,
    ang_acc: f32,
}

impl Update for Body {
    fn update(&mut self, dt: f32) -> () {
        let (lin_fric, ang_fric) = {
            // [DragClamp]
            (
                -1.0 * if self.lin_vel.length() > 150.0 {
                    1.0 * self.lin_vel
                } else {
                    Vec2 { x: 0.0, y: 0.0 }
                },
                -10.0
                    * if self.ang_vel.abs() > 5.0 {
                        1.0 * self.ang_vel
                    } else {
                        0.0
                    },
            )
        };

        self.lin_acc += lin_fric;
        self.lin_vel += self.lin_acc * dt;
        self.lin_pos += self.lin_vel * dt;

        self.ang_acc += ang_fric;
        self.ang_vel += self.ang_acc * dt;
        self.ang_pos += self.ang_vel * dt;

        {
            // warping
            let x = &mut self.lin_pos.x;
            let y = &mut self.lin_pos.y;
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

impl Draw for Ship {
    fn draw(&self) -> () {
        let (v1, v2, v3) = self.get_shape();
        draw_triangle_lines(v1, v2, v3, LINE_THICKNESS, LINE_COLOR);
    }
}

impl Update for Ship {
    fn update(&mut self, dt: f32) -> () {
        let (lin_boost, ang_boost): (Vec2, f32) = (
            if is_key_down(KeyCode::Up) {
                -800.0
                    * Vec2 {
                        x: self.body.ang_pos.cos(),
                        y: self.body.ang_pos.sin(),
                    }
            } else {
                // [AutoBreak]
                -1.0 * self.body.lin_vel
            },
            if is_key_down(KeyCode::Left) {
                -10.0
            } else {
                0.0
            } + if is_key_down(KeyCode::Right) {
                10.0
            } else {
                0.0
            }
            // [AutoBreak]
             + 10.0
                * if !is_key_down(KeyCode::Left) && !is_key_down(KeyCode::Right) {
                    -self.body.ang_vel
                } else {
                    self.body.ang_vel
                },
        );

        self.body.lin_acc = lin_boost;
        self.body.ang_acc = ang_boost;

        self.body.update(dt);
    }
}

impl Update for Asteroid {
    fn update(&mut self, dt: f32) -> () {
        self.body.ang_acc = 0.0;
        self.body.lin_acc = Vec2 { x: 0.0, y: 0.0 };
        self.body.update(dt);
    }
}

impl Draw for Asteroid {
    fn draw(&self) -> () {
        draw_poly_lines(
            self.body.lin_pos.x,
            self.body.lin_pos.y,
            self.sides,
            self.size,
            self.body.ang_pos.to_degrees(),
            LINE_THICKNESS,
            LINE_COLOR,
        );
    }
}
