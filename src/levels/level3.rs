use crate::collision;
use crate::{draw_rectangle, get_time, Rect, Vec2, WHITE};
use crate::{Asteroid, Body, GameAssets, GameMode, GameObjects};
use crate::{Draw, Update};

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

pub fn run(
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
        .retain(|bullet| collision(bullet, &obstacle).is_none());
    game_objects.update(dt);

    game_objects.draw(Some(&game_assets.spritesheet));

    draw_rectangle(obstacle.x, obstacle.y, obstacle.w, obstacle.h, WHITE);
}
