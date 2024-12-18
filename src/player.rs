use crate::pixel_grid_snap::OuterCamera;
use crate::pixel_grid_snap::{RES_HEIGHT, RES_WIDTH};
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
pub struct PlayerPlugin;

pub const SPRITE_SIZE: f32 = 16.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (player_movement, fire_projectile, projectile_movement),
        );
    }
}

#[derive(Component)]
struct Player {
    velocity: Vec2,
    acceleration_rate: f32,
    max_velocity: f32,
}
#[derive(Component)]
struct Projectile {
    velocity: f32,
    direction: Vec2,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player {
            velocity: Vec2::ZERO,
            acceleration_rate: 500.0,
            max_velocity: 100.0,
        },
        Sprite::from_image(asset_server.load("elf.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut transform, mut player)) = query.get_single_mut() else {
        return;
    };

    // Calculate acceleration based on input
    let mut acceleration_vector = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        acceleration_vector.y += player.acceleration_rate;
    }
    if keys.pressed(KeyCode::KeyS) {
        acceleration_vector.y -= player.acceleration_rate;
    }
    if keys.pressed(KeyCode::KeyA) {
        acceleration_vector.x -= player.acceleration_rate;
    }
    if keys.pressed(KeyCode::KeyD) {
        acceleration_vector.x += player.acceleration_rate;
    }

    // Apply acceleration to velocity
    player.velocity += acceleration_vector * time.delta_secs();

    // Clamp speed to max_velocity
    let current_speed = player.velocity.length();
    if current_speed > player.max_velocity {
        player.velocity = player.velocity.normalize() * player.max_velocity;
    }

    println!(
        "velocity: {:?} translation: {:?}, speed: {:?}, acceleration: {:?}",
        player.velocity, transform.translation, current_speed, acceleration_vector
    );

    // Move player based on velocity
    transform.translation += (player.velocity * time.delta_secs()).extend(0.0);

    // Clamp position to screen bounds (accounting for sprite size)
    let min_x = -(RES_WIDTH as f32) / 2.0 + SPRITE_SIZE / 2.0;
    let max_x = RES_WIDTH as f32 / 2.0 - SPRITE_SIZE / 2.0;
    let min_y = -(RES_HEIGHT as f32) / 2.0 - SPRITE_SIZE / 2.0;
    let max_y = RES_HEIGHT as f32 / 2.0 + SPRITE_SIZE / 2.0;

    // Check for collisions and bounce
    if transform.translation.x <= min_x {
        transform.translation.x = min_x;
        player.velocity.x = player.velocity.x.abs(); // Bounce right
    } else if transform.translation.x >= max_x {
        transform.translation.x = max_x;
        player.velocity.x = -player.velocity.x.abs(); // Bounce left
    }

    if transform.translation.y <= min_y {
        transform.translation.y = min_y;
        player.velocity.y = player.velocity.y.abs(); // Bounce up
    } else if transform.translation.y >= max_y {
        transform.translation.y = max_y;
        player.velocity.y = -player.velocity.y.abs(); // Bounce down
    }
}

fn projectile_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Projectile)>) {
    for (mut transform, projectile) in query.iter_mut() {
        transform.translation +=
            (projectile.direction * projectile.velocity * time.delta_secs()).extend(0.0);
    }
}
fn fire_projectile(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<OuterCamera>>,
    q_player: Query<&Transform, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        println!("World coords: {}/{}", world_position.x, world_position.y);
    }

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        let player_transform = q_player.single();
        let player_position = player_transform.translation.truncate();
        let projectile_direction = (world_position - player_position).normalize();
        if mouse_button.just_pressed(MouseButton::Left) {
            commands.spawn((
                Projectile {
                    velocity: 100.0,
                    direction: projectile_direction,
                },
                Transform::from_translation(player_position.extend(0.0)),
                Sprite::from_image(asset_server.load("projectile.png")),
            ));
        }
    }
}
