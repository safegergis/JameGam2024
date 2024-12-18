mod pixel_grid_snap;
mod enemy;

use bevy::prelude::*;
use enemy::EnemyPlugin;
use pixel_grid_snap::PixelSnapPlugin;

fn main() {
    App::new()
    .add_plugins(PixelSnapPlugin)
    .add_plugins(EnemyPlugin)
    .run();
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}