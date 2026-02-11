use std::f32::consts::{PI, TAU};

use macroquad::{miniquad::gl::glTexImage2D, prelude::*};

enum GameMode {
    Play,
    Menu,
}

#[macroquad::main("Braideroids : Asteroids = Braid;")]
async fn main() {
    let ship: &mut Ship = &mut Default::default();
    let mut bullets = Vec::<Bullet>::new();
    let mut asteroids = vec![
        Asteroid {
            body: Body {
                lin_pos: Vec2 { x: 0.0, y: 300.0 },
                lin_vel: Vec2 {
                    x: -100.0,
                    y: -100.0,
                },
                lin_acc: Vec2 { x: 0.0, y: 0.0 },
                ang_pos: PI,
                ang_vel: 3.0,
                ang_acc: 0.0,
            },
            sides: 6,
            size: 100.0,
        },
        Asteroid {
            body: Body {
                lin_pos: Vec2 { x: 200.0, y: 300.0 },
                lin_vel: Vec2 {
                    x: -200.0,
                    y: 100.0,
                },
                lin_acc: Vec2 { x: 0.0, y: 0.0 },
                ang_pos: PI,
                ang_vel: 4.0,
                ang_acc: 0.0,
            },
            sides: 4,
            size: 150.0,
        },
    ];
    let player_image = load_texture("assets/panda.png").await.unwrap();
    let bullet_image = load_texture("assets/gun.png").await.unwrap();
    let mut last_tick = get_time();
    let mut game_over = false;
    let mut game_mode = GameMode::Menu;
    while !game_over {
        let current_tick = get_time();
        let dt = current_tick - last_tick;
        last_tick = current_tick;
        match game_mode {
            GameMode::Menu => {
                if is_key_pressed(KeyCode::V) {
                    game_over = true;
                }
                if is_key_pressed(KeyCode::Enter) {
                    game_mode = GameMode::Play;
                }
                clear_background(WHITE);
                ship.draw();
                asteroids.iter().for_each(|asteroid| asteroid.draw());
                bullets.iter().for_each(|bullet| {
                    bullet.draw();
                });
            }
            GameMode::Play => {
                if is_key_pressed(KeyCode::Enter) {
                    game_mode = GameMode::Menu;
                }
                clear_background(BLACK);

                if is_key_pressed(KeyCode::F) {
                    bullets.push(Bullet {
                        body: Body {
                            lin_pos: ship.shape()[0],
                            lin_vel: -1000.0
                                * Vec2 {
                                    x: ship.body.ang_pos.cos(),
                                    y: ship.body.ang_pos.sin(),
                                }
                                + ship.body.lin_vel,
                            lin_acc: Vec2 { x: 0.0, y: 0.0 },
                            ang_pos: 0.0,
                            ang_vel: 0.0,
                            ang_acc: 0.0,
                        },
                        timer: 0.0,
                    })
                }

                let mut new_asteroids: Vec<Asteroid> = vec![];
                for asteroid in &asteroids {
                    if collision(ship, &asteroid) {
                        game_over = false;
                    }
                }
                asteroids.retain(|asteroid| {
                    let mut asteroid_collided = false;
                    let rotation_theta = 0.25 * TAU;

                    bullets.retain(|bullet| {
                        let collided = collision(bullet, asteroid);
                        asteroid_collided = collided;
                        if collided && asteroid.sides > 3 {
                            vec![
                                Asteroid {
                                    body: Body {
                                        lin_pos: asteroid.body.lin_pos,
                                        lin_vel: asteroid.body.lin_vel
                                            - Mat2 {
                                                x_axis: Vec2 {
                                                    x: rotation_theta.cos(),
                                                    y: -rotation_theta.sin(),
                                                },
                                                y_axis: Vec2 {
                                                    x: rotation_theta.sin(),
                                                    y: -rotation_theta.cos(),
                                                },
                                            } * 0.2
                                                * bullet.body.lin_vel,
                                        lin_acc: Vec2 { x: 0.0, y: 0.0 },
                                        ang_pos: asteroid.body.ang_pos,
                                        ang_vel: asteroid.body.ang_vel,
                                        ang_acc: 0.0,
                                    },
                                    sides: asteroid.sides - 1,
                                    size: 2.0 * asteroid.size / 3.0,
                                },
                                Asteroid {
                                    body: Body {
                                        lin_pos: asteroid.body.lin_pos,
                                        lin_vel: asteroid.body.lin_vel
                                            + Mat2 {
                                                x_axis: Vec2 {
                                                    x: rotation_theta.cos(),
                                                    y: -rotation_theta.sin(),
                                                },
                                                y_axis: Vec2 {
                                                    x: rotation_theta.sin(),
                                                    y: -rotation_theta.cos(),
                                                },
                                            } * 0.2
                                                * bullet.body.lin_vel,
                                        lin_acc: Vec2 { x: 0.0, y: 0.0 },
                                        ang_pos: asteroid.body.ang_pos,
                                        ang_vel: asteroid.body.ang_vel,
                                        ang_acc: 0.0,
                                    },
                                    sides: asteroid.sides - 1,
                                    size: 2.0 * asteroid.size / 3.0,
                                },
                            ]
                            .iter()
                            .for_each(|new_asteroid| new_asteroids.push(new_asteroid.clone()))
                        }
                        !collided && (bullet.timer <= 1.0)
                    });
                    !asteroid_collided
                });
                new_asteroids
                    .iter()
                    .for_each(|new_asteroid| asteroids.push(new_asteroid.clone()));

                ship.update(dt as f32);
                asteroids
                    .iter_mut()
                    .for_each(|asteroid| asteroid.update(dt as f32));
                bullets
                    .iter_mut()
                    .for_each(|bullet| bullet.update(dt as f32));

                ship.draw();
                asteroids.iter().for_each(|asteroid| asteroid.draw());
                bullets.iter().for_each(|bullet| {
                    bullet.draw();
                });
            }
        };
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

impl Ship {}

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

#[derive(Clone)]
struct Asteroid {
    body: Body,
    sides: u8,
    size: f32,
}

impl Asteroid {}

#[derive(Clone)]
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
                -1.0 * if self.lin_vel.length() > 1000.0 {
                    1.0 * self.lin_vel
                } else {
                    Vec2 { x: 0.0, y: 0.0 }
                },
                -10.0
                    * if self.ang_vel.abs() > 10.0 {
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
        let vertices = self.shape();
        debug_assert!(vertices.len() == 3);
        draw_triangle_lines(
            vertices[0],
            vertices[1],
            vertices[2],
            LINE_THICKNESS,
            LINE_COLOR,
        );
    }
}

impl Update for Ship {
    fn update(&mut self, dt: f32) -> () {
        let input_left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::J);
        let input_right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::L);

        let lin_boost: Vec2 = if is_key_down(KeyCode::Up) || is_key_down(KeyCode::I) {
            -4000.0
                * Vec2 {
                    x: self.body.ang_pos.cos(),
                    y: self.body.ang_pos.sin(),
                }
        } else {
            // [AutoBreak]
            -10.0 * self.body.lin_vel
        };
        let value = 50.0;
        let ang_boost: f32 = if input_left {
                -value
            } else if input_right {
                value
            } else {
                0.0
            }
            // [AutoBreak]
             + if input_left || input_right {
                    if self.body.ang_vel.abs() > 7.0 {
                        -value/2.0*self.body.ang_vel
                    } else{
                        0.0
                    }
                } else {
                    -value*self.body.ang_vel
                };

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

struct Bullet {
    body: Body,
    timer: f32,
}

impl Draw for Bullet {
    fn draw(&self) -> () {
        draw_circle_lines(
            self.body.lin_pos.x,
            self.body.lin_pos.y,
            5.0,
            LINE_THICKNESS,
            LINE_COLOR,
        );
    }
}

impl Update for Bullet {
    fn update(&mut self, dt: f32) {
        self.body.ang_acc = 0.0;
        self.body.lin_acc = Vec2 { x: 0.0, y: 0.0 };
        self.body.update(dt);
        self.timer += dt;
    }
}

trait Shape {
    fn shape(&self) -> Vec<Vec2>;
}

impl Shape for Ship {
    fn shape(&self) -> Vec<Vec2> {
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
        return vec![v1, v2, v3];
    }
}

impl Shape for Asteroid {
    fn shape(&self) -> Vec<Vec2> {
        let mut vertices: Vec<Vec2> = Vec::with_capacity(self.sides as usize);
        for it in 0..self.sides as usize {
            let angle = self.body.ang_pos + it as f32 * TAU / self.sides as f32;
            vertices.push(
                self.body.lin_pos
                    + self.size
                        * Vec2 {
                            x: angle.cos(),
                            y: angle.sin(),
                        },
            );
        }
        vertices
    }
}
impl Shape for Bullet {
    fn shape(&self) -> Vec<Vec2> {
        let mut vertices = Vec::<Vec2>::new();
        vertices.push(self.body.lin_pos);
        vertices
    }
}

fn dbg_draw_vertices(vertices: Vec<Vec2>) {
    vertices.iter().for_each(|vertice| {
        draw_circle(vertice.x, vertice.y, 10.0, BLUE);
    })
}

fn collision(object: &impl Shape, asteroid: &Asteroid) -> bool {
    let dist_collision =
        |lin_pos1: Vec2, lin_pos2: Vec2| -> bool { (lin_pos1 - lin_pos2).length() < asteroid.size };
    for object_vertice in object.shape() {
        if dist_collision(object_vertice, asteroid.body.lin_pos) {
            return true;
        }
    }

    false
}

enum Entity {
    Asteroid,
    Bullet,
    Ship,
}
