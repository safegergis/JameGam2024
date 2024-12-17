use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "JameGam2024".to_string(),
                resolution: (640.0, 360.0).into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}

