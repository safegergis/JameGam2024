#![windows_subsystem = "windows"]

mod audio;
mod background;
mod camera;
mod collision;
mod enemy;
mod mainmenu;
mod pickup;
mod player;
mod ui;
mod utils;

use audio::AudioPlugin;
use background::BackgroundPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use mainmenu::MainMenuPlugin;
use pickup::PickupPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    Playing,
    Paused,
    Upgrade,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum AppState {
    MainMenu,
    InGame,
    GameOver,
    Settings,
    Credits,
    Tutorial,
}

#[derive(Resource)]
pub struct Volume {
    pub music: f32,
    pub sfx: f32,
}

fn main() {
    App::new()
        .insert_resource(Volume {
            music: 1.0,
            sfx: 1.0,
        })
        .add_plugins(CameraPlugin)
        .add_plugins(BackgroundPlugin)
        .insert_state(AppState::MainMenu)
        .insert_state(GameState::Playing)
        .add_plugins(AudioPlugin)
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
        .add_plugins(UiPlugin {
            state: AppState::InGame,
        })
        //.add_plugins(HanabiPlugin)
        .run();
}
