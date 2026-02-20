mod collisions;
mod levels;
mod prelude;

use collisions::collision;
use levels::Level;
use levels::{level1, level2, level3, menu};
use prelude::{Asteroid, Body, Bullet, Ship, Smoke};
use prelude::{Draw, Shape, Update};
use prelude::{GameAssets, GameMode, GameObjects};

use macroquad::{
    audio::{load_sound, play_sound, PlaySoundParams},
    prelude::*,
};
use std::f32::consts::{PI, TAU};

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
    let mut game_mode = GameMode::Play(Level::Lvl3);
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
                menu::run(&mut game_last_tick, &game_assets, &mut game_objects);
                if is_key_pressed(KeyCode::Enter) {
                    game_mode = GameMode::Play(Level::Lvl1);
                    level1::init(&mut game_last_tick, &mut game_objects);
                }
            }
            GameMode::Play(ref current_level) => match current_level {
                Level::Lvl1 => {
                    level1::run(&mut game_last_tick, &mut game_objects, &game_assets);

                    if is_key_pressed(KeyCode::Enter) {
                        game_mode = GameMode::Menu;
                    }

                    if game_objects.asteroids.len() == 0 {
                        game_mode = GameMode::Play(Level::Lvl2);
                        level2::init(&mut game_last_tick, &mut game_objects);
                    }
                }
                Level::Lvl2 => {
                    level2::run(&mut game_last_tick, &mut game_objects, &game_assets);

                    if is_key_pressed(KeyCode::Enter) {
                        game_mode = GameMode::Menu;
                    }

                    if game_objects.asteroids.len() == 0 {
                        game_mode = GameMode::Play(Level::Lvl3);
                        level3::init(&mut game_last_tick, &mut game_objects);
                    }
                }
                Level::Lvl3 => {
                    level3::run(
                        &mut game_last_tick,
                        &mut game_mode,
                        &mut game_objects,
                        &game_assets,
                    );

                    if is_key_pressed(KeyCode::Enter) {
                        game_mode = GameMode::Menu;
                    }

                    if game_objects.asteroids.len() == 0 {
                        game_mode = GameMode::Menu;
                        menu::init();
                    }
                }
            },
        };
        next_frame().await;
    }
}
