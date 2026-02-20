use crate::Draw;
use crate::{draw_text, draw_texture, BLACK, WHITE};
use crate::{GameAssets, GameObjects};

pub fn init() {}

pub fn run(_game_last_tick: &mut f32, game_assets: &GameAssets, game_objects: &mut GameObjects) {
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
    draw_text("Braideroids", 50.0, 100.0, 60.0, BLACK);
    draw_text(
        "Press enter to Start/Pause the game at any [time].",
        50.0,
        130.0,
        24.0,
        BLACK,
    );
}
