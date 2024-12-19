use crate::camera::{InGameCamera, OuterCamera, Rotate};
use crate::utils::YSort;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(PostStartup, spawn_shield);
        app.add_systems(
            Update,
            (
                player_movement,
                fire_projectile,
                projectile_movement,
                animate_sprite,
                camera_follow,
            ),
        );
        app.add_systems(FixedUpdate, shield_movement);
    }
}
const LERP_FACTOR: f32 = 4.0;

#[derive(Component)]
pub struct Player {
    velocity: Vec2,
    acceleration_rate: f32,
    max_velocity: f32,
}
#[derive(Component)]
pub struct Projectile {
    velocity: f32,
    direction: Vec2,
}
#[derive(Component)]
struct ShieldCircle {
    number: u32,
}
#[derive(Component)]
pub struct Shield {
    pub damage: u32,
}
#[derive(Component)]
pub struct PlayerXp {
    pub xp: u32,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("elf.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 2 };
    let player = commands
        .spawn((
            Player {
                velocity: Vec2::ZERO,
                acceleration_rate: 500.0,
                max_velocity: 100.0,
            },
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Transform::from_xyz(0.0, 0.0, 0.0),
            YSort { z: 32.0 },
            ShieldCircle { number: 3 },
            PlayerXp { xp: 0 },
        ))
        .id();

    let texture = asset_server.load("Snowball484819.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 19, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 18 };
    let snowball_sprite = commands
        .spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
            Transform::from_xyz(0.0, -20.0, 0.0),
            YSort { z: -32.0 },
        ))
        .id();

    let snowmball_shadow = commands
        .spawn((
            Transform::from_xyz(0.0, -22.0, 0.0),
            Sprite::from_image(asset_server.load("Shadow.png")),
            YSort { z: -100.0 },
        ))
        .id();

    commands.entity(player).add_child(snowball_sprite);
    commands.entity(player).add_child(snowmball_shadow);
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

    // Move player based on velocity
    transform.translation += (player.velocity * time.delta_secs()).extend(0.0);
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<InGameCamera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let target = player_transform.translation;
    let current = camera_transform.translation;
    let lerp_factor = LERP_FACTOR * time.delta_secs();

    camera_transform.translation = current.lerp(target, lerp_factor);
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
    q_incamera: Query<&GlobalTransform, With<InGameCamera>>,
    q_player: Query<&Transform, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let camera_in_transform = q_incamera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        let new_world_position = world_position + camera_in_transform.translation().truncate();
        let player_transform = q_player.single();
        let player_position = player_transform.translation.truncate();
        let projectile_direction = (new_world_position - player_position).normalize();
        if mouse_button.just_pressed(MouseButton::Left) {
            commands.spawn((
                Projectile {
                    velocity: 550.0,
                    direction: projectile_direction,
                },
                Transform::from_translation(player_position.extend(0.0)),
                Sprite::from_image(asset_server.load("candycane_shuriken.png")),
                Rotate { speed: -30.0 },
            ));
        }
    }
}
const SHIELD_OFFSET: f32 = 50.0;
fn spawn_shield(
    mut commands: Commands,
    q_player: Query<(Entity, &Transform, &ShieldCircle), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let (player_entity, player_transform, shield_circle) = q_player.single();
    for i in 0..shield_circle.number {
        let child = commands
            .spawn((
                Shield { damage: 10 },
                Transform::from_translation({
                    let angle =
                        (i as f32) * 2.0 * std::f32::consts::PI / (shield_circle.number as f32);
                    let x = SHIELD_OFFSET * angle.cos();
                    let y = SHIELD_OFFSET * angle.sin();
                    player_transform.translation + Vec3::new(x, y, 0.0)
                }),
                Sprite::from_image(asset_server.load("chestnut.png")),
            ))
            .id();
        commands.entity(player_entity).add_child(child);
    }
}
fn shield_movement(mut shield_query: Query<&mut Transform, (With<Shield>, Without<Player>)>) {
    for mut transform in shield_query.iter_mut() {
        let rotation = Quat::from_rotation_z(0.05);
        transform.translation = rotation * transform.translation;
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
