mod background;
mod camera;
mod collision;
mod enemy;
mod mainmenu;
mod pickup;
mod player;
mod ui;
mod utils;

use background::BackgroundPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use mainmenu::MainMenuPlugin;
use pickup::PickupPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;
use bevy_hanabi::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    Playing,
    Paused,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum AppState {
    MainMenu,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(CameraPlugin)
        .insert_state(AppState::MainMenu)
        .add_plugins(MainMenuPlugin {
            state: AppState::MainMenu,
        })
        .add_plugins(EnemyPlugin {
            state: AppState::InGame,
        })
        .add_plugins(PlayerPlugin {
            state: AppState::InGame,
        })
        .add_plugins(CollisionPlugin {
            state: AppState::InGame,
        })
        .add_plugins(PickupPlugin {
            state: AppState::InGame,
        })
        .add_plugins(BackgroundPlugin)
        .add_plugins(UiPlugin {
            state: AppState::InGame,
        })
        .add_plugins(HanabiPlugin)
        .run();
}
