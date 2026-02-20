use crate::{draw_text, draw_texture, get_time, Vec2, GRAY, WHITE};
use crate::{Asteroid, Body, GameAssets, GameObjects};
use crate::{Draw, Update};

pub fn run(game_last_tick: &mut f32, game_objects: &mut GameObjects, game_assets: &GameAssets) {
    let current_tick = get_time() as f32;
    let dt = current_tick - *game_last_tick;
    *game_last_tick = current_tick;

    game_objects.update(dt);

    //clear_background(BLACK);
    draw_texture(&game_assets.blob_dark, 0.0, 0.0, WHITE);
    draw_text("Level 1", 100.0, 100.0, 50.0, GRAY);
    game_objects.draw(Some(&game_assets.spritesheet));
}

pub fn init(game_last_tick: &mut f32, game_objects: &mut GameObjects) {
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
