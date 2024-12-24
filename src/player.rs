use crate::camera::{InGameCamera, OuterCamera, Rotate};
use crate::utils::YSort;
use crate::AppState;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_hanabi::prelude::*;

pub struct PlayerPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PlayerPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state.clone()), spawn_player);
        app.add_systems(
            PostStartup,
            spawn_shield.run_if(in_state(self.state.clone())),
        );
        app.add_systems(
            Update,
            (
                player_movement,
                fire_projectile,
                projectile_movement,
                animate_sprite,
                camera_follow,
                scale_snowball_to_health,
                kill_player,
            )
                .run_if(in_state(self.state.clone())),
        );
        app.add_systems(
            FixedUpdate,
            shield_movement.run_if(in_state(self.state.clone())),
        );
    }
}
const LERP_FACTOR: f32 = 4.0;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub acceleration_rate: f32,
    max_velocity: f32,
}
#[derive(Component)]
pub struct Projectile {
    velocity: f32,
    direction: Vec2,
    pub pierce_amount: i32,
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
#[derive(Component)]
pub struct PlayerHealth {
    pub hp: f32,
}
#[derive(Component)]
pub struct PlayerSnowball;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut effects: ResMut<Assets<EffectAsset>>,
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
            PlayerHealth { hp: 10.0 },
        ))
        .id();

    let texture = asset_server.load("Snowball48488.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 6 };
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
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Transform::from_xyz(0.0, -20.0, 0.0),
            YSort { z: -32.0 },
            PlayerSnowball,
        ))
        .id();

    let snowmball_shadow = commands
        .spawn((
            Transform::from_xyz(0.0, -2.0, 0.0),
            Sprite::from_image(asset_server.load("Shadow.png")),
            YSort { z: -100.0 },
        ))
        .id();

    commands.entity(player).add_child(snowball_sprite);
    commands.entity(snowball_sprite).add_child(snowmball_shadow);





    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec3::splat(0.6).extend(1.0));
    gradient.add_key(1.0, Vec3::splat(0.6).extend(1.0));
  
    // Create a new expression module
    let mut module = Module::default();
  
    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(5.),
        dimension: ShapeDimension::Surface,
    };
  
    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    // let init_vel = SetVelocitySphereModifier {
    //     center: module.lit(Vec3::ZERO),
    //     speed: module.lit(6.),
    // };
  
    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(10.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME, lifetime);
  
    // Every frame, add a gravity-like acceleration downward
    //let accel = module.lit(Vec3::new(0., -3., 0.));
    //let update_accel = AccelModifier::new(accel);
  
    // Create the effect asset
    let effect = EffectAsset::new(
      // Maximum number of particles alive at a time
      32768,
      // Spawn at a rate of 5 particles per second
      Spawner::rate(100.0.into()),
      // Move the expression module into the asset
      module
    )
    .with_name("MyEffect")
    .init(init_pos)
    //.init(init_vel)
    .init(init_lifetime)
    //.update(update_accel)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier { gradient })
    .render(SetSizeModifier { size: Vec3::splat(8.).into() });
  
    // Insert into the asset system
    let effect_handle = effects.add(effect);

    let snowball_particle = commands.spawn((
        Name::new("firework"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle).with_z_layer_2d(Some(-100.)),
            transform: Transform::from_xyz(0.0, -6., -100.0),
            ..Default::default()
        },
    )).id();

    commands.entity(snowmball_shadow).add_child(snowball_particle);
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
                    pierce_amount: 0,
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
fn scale_snowball_to_health(
    mut q_player: Query<(&PlayerHealth, &mut Player)>,
    mut q_player_snowball: Query<&mut Transform, With<PlayerSnowball>>,
) {
    let (player_health, mut player) = q_player.single_mut();
    let mut player_snowball_tf = q_player_snowball.single_mut();
    if player_health.hp > 10.0 {
        player_snowball_tf.scale = Vec3::new(player_health.hp / 10.0, player_health.hp / 10.0, 1.0);
        player.max_velocity = player_health.hp * 10.0;
    }
    println!(
        "Player health: {}, max velocity: {}",
        player_health.hp, player.max_velocity
    );
}
fn kill_player(mut commands: Commands, q_player: Query<(Entity, &PlayerHealth), With<Player>>) {
    let (player_entity, player_health) = q_player.single();
    if player_health.hp <= 0.0 {
        commands.entity(player_entity).despawn_recursive();
        commands.set_state(AppState::GameOver);
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
