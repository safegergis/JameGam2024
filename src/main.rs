mod collision;
mod enemy;
mod pixel_grid_snap;
mod player;
mod utils;

use bevy::prelude::*;
use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use pixel_grid_snap::PixelSnapPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(PixelSnapPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}
