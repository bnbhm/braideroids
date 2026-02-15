use std::f32::consts::{PI, TAU};

mod collisions;

use collisions::collision;

use macroquad::{
    audio::{load_sound, play_sound, PlaySoundParams},
    prelude::*,
};

enum GameMode {
    Menu,
    Play(GameLevel),
}

struct GameAssets {
    spritesheet: Texture2D,
    blob_light: Texture2D,
    blob_dark: Texture2D,
}

struct GameObjects {
    ship: Ship,
    asteroids: Vec<Asteroid>,
    bullets: Vec<Bullet>,
    smokes: Vec<Smoke>,
}

enum GameLevel {
    Lvl1,
    Lvl2,
    Lvl3,
}

struct Level {
    level_initialised: bool,
}

impl Level {
    fn level_init(&self) {
        todo!();
    }

    fn level_loop(&self) {
        todo!();
    }
}

/* impl Level for GameLevel {
    fn level_init(&self) {
        match self {
            GameLevel::Lvl1 => {
                lvl1_init();
            }
            GameLevel::Lvl2 => {
                lvl2_init();
            }
        }
    }

    fn level_loop(&self) {}
} */

#[macroquad::main("Braideroids : Asteroids = Braid;")]
async fn main() {
    let music = load_sound("assets/catelectrician.wav").await.unwrap();

    let spritesheet = load_texture("assets/all.png").await.unwrap();
    let blob_light = load_texture("assets/blob_light.png").await.unwrap();
    let blob_dark = load_texture("assets/blob_dark.png").await.unwrap();

    let game_assets = GameAssets {
        spritesheet,
        blob_light,
        blob_dark,
    };

    let ship: Ship = Default::default();
    let bullets = Vec::<Bullet>::new();
    let asteroids = Vec::<Asteroid>::new();
    let smokes = Vec::<Smoke>::new();
    let mut game_objects = GameObjects {
        ship,
        asteroids,
        bullets,
        smokes,
    };

    let mut game_last_tick = get_time() as f32;
    let mut game_mode = GameMode::Play(GameLevel::Lvl3);
    play_sound(
        &music,
        PlaySoundParams {
            looped: true,
            volume: 0.4,
        },
    );

    loop {
        match game_mode {
            GameMode::Menu => {
                menu_run(&mut game_last_tick, &game_assets, &mut game_objects);
                if is_key_pressed(KeyCode::Enter) {
                    game_mode = GameMode::Play(GameLevel::Lvl1);
                    lvl1_init(&mut game_last_tick, &mut game_objects);
                }
            }
            GameMode::Play(ref current_level) => match current_level {
                GameLevel::Lvl1 => {
                    lvl1_run(&mut game_last_tick, &mut game_objects, &game_assets);

                    if is_key_pressed(KeyCode::Enter) {
                        game_mode = GameMode::Menu;
                    }

                    if game_objects.asteroids.len() == 0 {
                        game_mode = GameMode::Play(GameLevel::Lvl2);
                        lvl2_init(&mut game_last_tick, &mut game_objects);
                    }
                }
                GameLevel::Lvl2 => {
                    lvl2_run(&mut game_last_tick, &mut game_objects, &game_assets);

                    if is_key_pressed(KeyCode::Enter) {
                        game_mode = GameMode::Menu;
                    }

                    if game_objects.asteroids.len() == 0 {
                        game_mode = GameMode::Play(GameLevel::Lvl3);
                        lvl3_init(&mut game_last_tick, &mut game_objects);
                    }
                }
                GameLevel::Lvl3 => {
                    lvl3_run(
                        &mut game_last_tick,
                        &mut game_mode,
                        &mut game_objects,
                        &game_assets,
                    );

                    if is_key_pressed(KeyCode::Enter) {
                        game_mode = GameMode::Menu;
                    }

                    if game_objects.asteroids.len() == 0 {
                        game_mode = GameMode::Play(GameLevel::Lvl3);
                        lvl3_init(&mut game_last_tick, &mut game_objects);
                    }
                }
            },
        };
        next_frame().await;
    }
}

fn menu_init() {}

fn menu_run(game_last_tick: &mut f32, game_assets: &GameAssets, game_objects: &mut GameObjects) {
    draw_texture(&game_assets.blob_light, 0.0, 0.0, WHITE);
    game_objects.ship.draw(Some(&game_assets.spritesheet));
    game_objects
        .asteroids
        .iter()
        .for_each(|asteroid| asteroid.draw(None));
    game_objects.bullets.iter().for_each(|bullet| {
        bullet.draw(Some(&game_assets.spritesheet));
    });
    game_objects
        .smokes
        .iter()
        .for_each(|smoke| smoke.draw(Some(&game_assets.spritesheet)));
    /* draw_texture_ex(
        &spritesheet,
        120.0,
        200.0,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(242.0, 365.0, 252.0, 252.0)),
            rotation: 0.0,
            ..Default::default()
        },
    );
    draw_texture_ex(
        &spritesheet,
        500.0,
        200.0,
        RED,
        DrawTextureParams {
            source: Some(Rect::new(242.0, 365.0, 252.0, 252.0)),
            rotation: 0.0,
            ..Default::default()
        },
    ); */
    draw_text("Braideroids", 50.0, 100.0, 60.0, BLACK);
    draw_text(
        "Press enter to Start/Pause the game at any [time].",
        50.0,
        130.0,
        24.0,
        BLACK,
    );
}

fn lvl1_run(game_last_tick: &mut f32, game_objects: &mut GameObjects, game_assets: &GameAssets) {
    let current_tick = get_time() as f32;
    let dt = current_tick - *game_last_tick;
    *game_last_tick = current_tick;

    game_objects.update(dt);

    //clear_background(BLACK);
    draw_texture(&game_assets.blob_dark, 0.0, 0.0, WHITE);
    draw_text("Level 1", 100.0, 100.0, 50.0, GRAY);
    game_objects.draw(Some(&game_assets.spritesheet));
}

fn lvl1_init(game_last_tick: &mut f32, game_objects: &mut GameObjects) {
    game_objects.asteroids = vec![Asteroid {
        body: Body {
            lin_pos: Vec2 { x: 700.0, y: 500.0 },
            ..Default::default()
        },
        sides: 3,
        size: 70.0,
    }];
    game_objects.bullets = vec![];
    game_objects.smokes = vec![];
    game_objects.ship.body = Body {
        lin_pos: Vec2 { x: 200.0, y: 500.0 },
        ..Default::default()
    };
    *game_last_tick = get_time() as f32;
}

fn lvl2_run(game_last_tick: &mut f32, game_objects: &mut GameObjects, game_assets: &GameAssets) {
    let current_tick = get_time() as f32;
    let dt = current_tick - *game_last_tick;
    *game_last_tick = current_tick;

    game_objects.update(dt);

    draw_texture(&game_assets.blob_dark, 0.0, 0.0, WHITE);
    game_objects.draw(Some(&game_assets.spritesheet));
    draw_text("Level 2", 100.0, 100.0, 50.0, GRAY);
}

fn lvl3_init(game_last_tick: &mut f32, game_objects: &mut GameObjects) {
    game_objects.asteroids = vec![Asteroid {
        body: Body {
            lin_pos: Vec2 { x: 700.0, y: 500.0 },
            ..Default::default()
        },
        sides: 3,
        size: 70.0,
    }];
    game_objects.bullets = vec![];
    game_objects.smokes = vec![];
    game_objects.ship.body = Body {
        lin_pos: Vec2 { x: 200.0, y: 500.0 },
        ..Default::default()
    };
    *game_last_tick = get_time() as f32;
}

fn lvl3_run(
    game_last_tick: &mut f32,
    game_mode: &mut GameMode,
    game_objects: &mut GameObjects,
    game_assets: &GameAssets,
) {
    let current_tick = get_time() as f32;
    let dt = current_tick - *game_last_tick;
    *game_last_tick = current_tick;

    let obstacle = Rect {
        x: 400.0,
        y: 425.0,
        w: 20.0,
        h: 150.0,
    };

    game_objects
        .bullets
        .retain(|bullet| !collision(bullet, &obstacle));
    //if collision(&game_objects.ship, &obstacle){game_objects.ship.lin_vel = Vec2::new(0.0,0.0);}
    game_objects.update(dt);

    game_objects.draw(Some(&game_assets.spritesheet));

    draw_rectangle(obstacle.x, obstacle.y, obstacle.w, obstacle.h, WHITE);
    draw_rectangle(
        obstacle.x + obstacle.w / 2.0,
        obstacle.y + obstacle.h / 2.0,
        obstacle.w / 4.0,
        obstacle.h / 4.0,
        BLACK,
    );
}

impl Shape for Rect {
    fn shape(&self) -> Vec<Vec2> {
        let mut vertices = vec![];
        let left_top = Vec2::new(self.x, self.y);
        vertices.push(left_top);
        vertices.push(left_top + Vec2 { x: self.w, y: 0.0 });
        vertices.push(left_top + Vec2 { x: 0.0, y: self.h });
        vertices.push(
            left_top
                + Vec2 {
                    x: self.w,
                    y: self.h,
                },
        );

        return vertices;
    }
}

fn lvl2_init(game_last_tick: &mut f32, game_objects: &mut GameObjects) {
    *game_last_tick = get_time() as f32;
    game_objects.asteroids = vec![Asteroid {
        body: Body {
            lin_pos: Vec2 { x: 700.0, y: 500.0 },
            lin_vel: Vec2 { x: 70.0, y: 0.0 },
            ang_vel: 2.0,
            ..Default::default()
        },
        sides: 3,
        size: 150.0,
    }];
    game_objects.bullets = vec![];
    game_objects.smokes = vec![];
    game_objects.ship.body = Body {
        lin_pos: Vec2 { x: 200.0, y: 500.0 },
        ..Default::default()
    };
}

struct Smoke {
    body: Body,
    size: f32,
    timer: f32,
}

impl Update for Smoke {
    fn update(&mut self, dt: f32) -> () {
        self.timer += dt;
    }
}

impl Draw for Smoke {
    fn draw(&self, spritesheet: Option<&macroquad::texture::Texture2D>) -> () {
        let scale = 1.0 + self.timer / 12.0;
        match spritesheet {
            Some(spritesheet) => draw_texture_ex(
                &spritesheet,
                self.body.lin_pos.x - scale * self.size / 2.0,
                self.body.lin_pos.y - scale * self.size / 2.0,
                Color::new(1.0, 1.0, 1.0, 1.0 - self.timer / 3.0),
                DrawTextureParams {
                    source: Some(Rect::new(220.0, 640.0, 240.0, 240.0)),
                    dest_size: Some(scale * Vec2::new(self.size, self.size)),
                    ..Default::default()
                },
            ),
            None => {
                draw_circle_lines(
                    self.body.lin_pos.x,
                    self.body.lin_pos.y,
                    5.0,
                    LINE_THICKNESS,
                    LINE_COLOR,
                );
            }
        }
    }
}

fn is_out_of_screen(body: &Body) -> bool {
    return body.lin_pos.x <= 0.0
        || body.lin_pos.y <= 0.0
        || body.lin_pos.x >= screen_width()
        || body.lin_pos.y >= screen_height();
}

// CONSTANTS
const LINE_THICKNESS: f32 = 2.0;
const LINE_COLOR: Color = DARKBLUE;

trait Draw {
    fn draw(&self, spritesheet: Option<&Texture2D>) -> ();
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
                lin_pos: Vec2 { x: 535.0, y: 55.0 },
                lin_vel: Vec2 { x: 0.0, y: 0.0 },
                lin_acc: Vec2 { x: 0.0, y: 0.0 },
                ang_pos: 3.0 / 4.0 * TAU,
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

impl Default for Body {
    fn default() -> Self {
        Body {
            lin_pos: Vec2 { x: 0.0, y: 0.0 },
            lin_vel: Vec2 { x: 0.0, y: 0.0 },
            lin_acc: Vec2 { x: 0.0, y: 0.0 },
            ang_pos: TAU / 2.0,
            ang_vel: 0.0,
            ang_acc: 0.0,
        }
    }
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
                if self.ang_vel.abs() > 15.0 {
                    15.0 * self.ang_vel / self.ang_vel.abs()
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
    fn draw(&self, _spritesheet: Option<&Texture2D>) -> () {
        /* match spritesheet {
            Some(spritesheet) => draw_texture_ex(
                &spritesheet,
                self.body.lin_pos.x - 25.0,
                self.body.lin_pos.y - 50.0,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(1450.0, 850.0, 170.0, 167.0)),
                    rotation: self.body.ang_pos + TAU / 2.0,
                    dest_size: Some(Vec2 { x: 100.0, y: 100.0 }),

                    ..Default::default()
                },
            ),
            None => {}
        } */
        let vertices = self.shape();
        debug_assert!(vertices.len() == 3);
        draw_triangle_lines(vertices[0], vertices[1], vertices[2], 5.0, ORANGE);
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
        let ang_boost: f32 = if input_left {
                -50.0
            } else if input_right {
                50.0
            } else {
                0.0
            }
            // [AutoBreak]
             + if !input_left && !input_right {
                    -15.0*self.body.ang_vel
                }
                else{0.0};

        self.body.lin_acc = lin_boost;
        self.body.ang_acc = ang_boost;

        // clamp
        if self.body.lin_vel.length() > 1000.0 {
            self.body.lin_vel = 1000.0 * self.body.lin_vel / self.body.lin_vel.length();
        }
        if self.body.ang_vel.abs() > 3.0 {
            self.body.ang_vel = 3.0 * self.body.ang_vel / self.body.ang_vel.abs();
        }
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
    fn draw(&self, texture: Option<&Texture2D>) -> () {
        match texture {
            Some(texture) => draw_texture(texture, self.body.lin_pos.x, self.body.lin_pos.y, WHITE),
            None => {
                draw_poly_lines(
                    self.body.lin_pos.x,
                    self.body.lin_pos.y,
                    self.sides,
                    self.size,
                    self.body.ang_pos.to_degrees(),
                    LINE_THICKNESS,
                    WHITE,
                );
            }
        }
    }
}

struct Bullet {
    body: Body,
}

impl Draw for Bullet {
    fn draw(&self, spritesheet: Option<&macroquad::texture::Texture2D>) -> () {
        match spritesheet {
            Some(spritesheet) => draw_texture_ex(
                &spritesheet,
                self.body.lin_pos.x,
                self.body.lin_pos.y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(1230.0, 810.0, 25.0, 25.0)),
                    rotation: self.body.ang_pos,
                    ..Default::default()
                },
            ),
            None => {
                draw_circle_lines(
                    self.body.lin_pos.x,
                    self.body.lin_pos.y,
                    5.0,
                    LINE_THICKNESS,
                    LINE_COLOR,
                );
            }
        }
    }
}

impl Draw for GameObjects {
    fn draw(&self, spritesheet: Option<&Texture2D>) -> () {
        self.bullets
            .iter()
            .for_each(|bullet| bullet.draw(spritesheet));
        self.asteroids
            .iter()
            .for_each(|asteroid| asteroid.draw(None));
        self.smokes.iter().for_each(|smoke| smoke.draw(spritesheet));
        self.ship.draw(spritesheet);
    }
}

impl Update for Bullet {
    fn update(&mut self, dt: f32) {
        self.body.ang_acc = 0.0;
        self.body.lin_acc = Vec2 { x: 0.0, y: 0.0 };
        self.body.update(dt);
    }
}

impl Update for GameObjects {
    fn update(&mut self, dt: f32) {
        self.ship.update(dt);
        self.bullets.iter_mut().for_each(|bullet| bullet.update(dt));
        self.asteroids
            .iter_mut()
            .for_each(|asteroid| asteroid.update(dt));
        self.smokes.iter_mut().for_each(|smoke| smoke.update(dt));

        if is_key_pressed(KeyCode::F) {
            self.bullets.push(Bullet {
                body: Body {
                    lin_pos: self.ship.shape()[0],
                    lin_vel: -1000.0
                        * Vec2 {
                            x: self.ship.body.ang_pos.cos(),
                            y: self.ship.body.ang_pos.sin(),
                        }
                        + 0.2 * self.ship.body.lin_vel,
                    lin_acc: Vec2 { x: 0.0, y: 0.0 },
                    ang_pos: self.ship.body.ang_pos + TAU / 2.0,
                    ang_vel: 0.0,
                    ang_acc: 0.0,
                },
            })
        }

        let mut new_asteroids: Vec<Asteroid> = vec![];
        self.bullets
            .retain(|bullet| !is_out_of_screen(&bullet.body));
        self.asteroids.retain(|asteroid| {
            let mut asteroid_collided = false;
            let rotation_theta = 0.25 * TAU;

            for bullet in &self.bullets {
                let collided = collision(bullet, asteroid);
                if collided {
                    self.smokes.push(Smoke {
                        body: Body {
                            lin_pos: asteroid.body.lin_pos,
                            ..Default::default()
                        },
                        timer: 0.0,
                        size: 2.0 * asteroid.size,
                    });
                    if asteroid.sides > 3 {
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
                                                y: rotation_theta.cos(),
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
                };
                asteroid_collided = collided;
                if collided {
                    break;
                }
            }
            !asteroid_collided
        });
        self.smokes.retain(|smoke| smoke.timer < 3.0);
        new_asteroids
            .iter()
            .for_each(|new_asteroid| self.asteroids.push(new_asteroid.clone()));
        for asteroid in &self.asteroids {
            if collision(&self.ship, asteroid) {
                //*game_mode = GameMode::Menu;
            }
        }
    }
}

trait Shape {
    fn shape(&self) -> Vec<Vec2>;
}

impl Shape for Ship {
    fn shape(&self) -> Vec<Vec2> {
        let radius = 30.0;
        let v1 = self.body.lin_pos
            - radius
                * Vec2 {
                    x: self.body.ang_pos.cos(),
                    y: self.body.ang_pos.sin(),
                };
        let v2 = self.body.lin_pos
            - 2.0 * radius / 3.0
                * Vec2 {
                    x: (self.body.ang_pos + 2.0 * PI / 3.0).cos(),
                    y: (self.body.ang_pos + 2.0 * PI / 3.0).sin(),
                };
        let v3 = self.body.lin_pos
            - 2.0 * radius / 3.0
                * Vec2 {
                    x: (self.body.ang_pos + 4.0 * PI / 3.0).cos(),
                    y: (self.body.ang_pos + 4.0 * PI / 3.0).sin(),
                };
        return vec![v1, v2, v3];
    }
}

impl Shape for Asteroid {
    fn shape(&self) -> Vec<Vec2> {
        let center = self.body.lin_pos;
        let radius = self.size;
        let rotation = self.body.ang_pos;

        let mut vertices = vec![];
        let theta = TAU / self.sides as f32;
        for it in 0..self.sides {
            vertices.push(
                center
                    + radius
                        * Vec2::new(
                            (it as f32 * theta + rotation).cos(),
                            (it as f32 * theta + rotation).sin(),
                        ),
            );
        }
        return vertices;
    }
}

impl Shape for Bullet {
    fn shape(&self) -> Vec<Vec2> {
        let mut vertices = Vec::<Vec2>::new();
        let width = 10.0;
        let height = 5.0;

        let left_top = self.body.lin_pos
            - Vec2 {
                x: width / 2.0,
                y: height / 2.0,
            };
        vertices.push(left_top);
        vertices.push(left_top + Vec2 { x: width, y: 0.0 });
        vertices.push(left_top + Vec2 { x: 0.0, y: height });
        vertices.push(
            left_top
                + Vec2 {
                    x: width,
                    y: height,
                },
        );
        vertices
    }
}
