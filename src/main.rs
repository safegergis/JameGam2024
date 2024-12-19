mod camera;
mod collision;
mod enemy;
mod player;
mod utils;

use bevy::prelude::*;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(CameraPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}
