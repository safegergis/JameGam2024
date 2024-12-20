mod camera;
mod collision;
mod enemy;
mod player;
mod ui;
mod utils;
mod background;

use bevy::prelude::*;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use background::BackgroundPlugin;
use ui::UiPlugin;
use bevy_hanabi::prelude::*;

fn main() {
    App::new()
        .add_plugins(CameraPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(HanabiPlugin)
        .run();
}
