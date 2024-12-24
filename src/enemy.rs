use crate::camera::InGameCamera;
use crate::collision::Blink;
use crate::player::Player;
use crate::player::PlayerHealth;
use crate::player::PlayerSnowball;
use crate::player::PlayerStats;
use crate::utils::YSort;
use crate::collision::FlashingTimer;

use bevy::prelude::*;
use bevy::sprite;
use rand::Rng;
pub struct EnemyPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for EnemyPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyTimer(Timer::from_seconds(0.05, TimerMode::Repeating)));
        app.add_systems(
            Update,
            (chase_player, spawn_enemy, wiggle, y_sort, kill_dead_enemies, unfreeze, extinguish)
                .run_if(in_state(self.state.clone())),
        );
        app.add_systems(OnExit(self.state.clone()), clean_up_enemies);
    }
}

#[derive(Resource)]
struct EnemyTimer(Timer);

#[derive(Component)]
pub struct EnemyHealth {
    pub health: f32,
}
#[derive(Component)]
pub struct EnemyXp {
    pub xp: u32,
}
#[derive(Component)]
pub struct ChasePlayer {
    pub speed: f32,
}
#[derive(Component)]
pub struct Frozen {
    pub duration: f32,
}
#[derive(Component)]
pub struct OnFire {
    pub duration: f32,
}

fn spawn_enemy(
    q_camera: Query<(&Camera, &GlobalTransform), With<InGameCamera>>,
    time: Res<Time>,
    mut timer: ResMut<EnemyTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let circle = Circle::new(350.0);
        let boundary_pt = circle.sample_boundary(&mut rand::thread_rng());
        let (_camera, camera_transform) = q_camera.single();

        let num_offset = rand::thread_rng().gen_range(-1.0..1.0);
        let snowman_holder = commands
            .spawn((
                Visibility::Visible,
                Transform::from_xyz(
                    boundary_pt.x + camera_transform.translation().x,
                    boundary_pt.y + camera_transform.translation().y,
                    2.0,
                ),
                YSort { z: 32.0 },
                Enemy,
                ChasePlayer { speed: 25.0 },
                EnemyHealth { health: 100. },
            ))
            .id();

        let snowman_sprite = commands
            .spawn((
                Sprite::from_image(asset_server.load("Snowman.png")),
                Wiggle {
                    rotate_speed: 18.0,
                    rotate_amount: 0.0125,
                    scale_speed: 18.0,
                    scale_amount: 0.125,
                    offset: num_offset,
                },
            ))
            .id();

        let snowman_shadow = commands
            .spawn((
                Transform::from_xyz(0.0, -8.0, 0.0),
                Sprite::from_image(asset_server.load("Shadow.png")),
                YSort { z: -100.0 },
                Wiggle {
                    rotate_speed: 0.0,
                    rotate_amount: 0.0,
                    scale_speed: 18.0,
                    scale_amount: 0.085,
                    offset: num_offset,
                },
            ))
            .id();
        commands.entity(snowman_holder).add_child(snowman_shadow);
        commands.entity(snowman_holder).add_child(snowman_sprite);
    }
}

#[derive(Component)]
pub struct Wiggle {
    pub rotate_speed: f32,
    pub rotate_amount: f32,
    pub scale_speed: f32,
    pub scale_amount: f32,
    pub offset: f32,
}

fn wiggle(time: Res<Time>, mut q: Query<(&mut Transform, &Wiggle), Without<Frozen>>) {
    for (mut tf, wiggle) in q.iter_mut() {
        let rotate_sin = f32::sin(wiggle.offset + time.elapsed_secs() * wiggle.rotate_speed);
        let scale_sin = f32::sin(wiggle.offset + time.elapsed_secs() * wiggle.scale_speed);
        tf.rotate_z(rotate_sin * wiggle.rotate_amount);
        tf.scale = Vec3::new(
            1.0 + scale_sin * wiggle.scale_amount,
            1.0 - scale_sin * wiggle.scale_amount,
            1.0,
        );
    }
}

#[derive(Component)]
pub struct Enemy;
fn kill_dead_enemies(
    mut commands: Commands,
    enemy_q: Query<(&EnemyHealth, &Transform, Entity), (With<Enemy>, Without<EnemyXp>)>,
    mut q_player: Query<&mut PlayerHealth, With<Player>>,
    mut q_snowball: Query<&mut Transform, (With<PlayerSnowball>, Without<Enemy>)>,
    asset_server: Res<AssetServer>,
) {
    let mut player_health = q_player.single_mut();
    let snowball_transform = q_snowball.single_mut();
    for (health, transform, entity) in enemy_q.iter() {
        if health.health <= 0. {
            commands.entity(entity).despawn_recursive();
            commands.spawn((
                EnemyXp { xp: 10 },
                Sprite::from_image(asset_server.load("xp.png")),
                Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
                ChasePlayer { speed: 100.0 },
            ));
            player_health.hp += 0.2;
        }
    }
}

fn chase_player(
    time: Res<Time>,
    q_player: Query<(&GlobalTransform, &Player)>,
    mut q: Query<(&mut Transform, &ChasePlayer), Without<Frozen>>,
) {
    let (player, _player_transform) = q_player.single();
    //println!("PlayerPositon coords: {}/{}", player.translation().x, player.translation().y)
    for (mut tf, chase_player) in q.iter_mut() {
        let dt = time.delta_secs() * chase_player.speed as f32;
        let dir = (player.translation().truncate() - tf.translation.truncate())
            .normalize()
            .extend(0.0);
        tf.translation += dir * dt;
    }
}

fn unfreeze(
    mut commands: Commands,
    mut q: Query<(&mut Frozen, Entity)>,
    time: Res<Time>,
) {
    for (mut frozen, frozen_entity) in q.iter_mut() {
        let dt = time.delta_secs();
        frozen.duration -= dt;
        if(frozen.duration <= 0.0)
        {
            commands.entity(frozen_entity).remove::<Frozen>();
        }
    }
}

fn extinguish(
    mut commands: Commands,
    mut q: Query<(&mut OnFire, &mut EnemyHealth, &Children, Entity)>,
    time: Res<Time>,
    stats: Res<PlayerStats>,
) {
    for (mut on_fire, mut enemy_health, children, on_fire_entity) in q.iter_mut() {
        let dt = time.delta_secs();
        on_fire.duration -= dt;
        enemy_health.health -= stats.fire_dps * dt;
        if(on_fire.duration <= 0.0)
        {
            commands.entity(on_fire_entity).remove::<OnFire>();
            commands.entity(children[1]).remove::<Blink>();
            commands.entity(children[1]).insert(FlashingTimer {
                time_left: 0.0,
                color: Color::srgba(1., 1., 1., 1.),
            });
        }
    }
}

fn y_sort(mut q: Query<(&mut Transform, &YSort)>) {
    for (mut tf, ysort) in q.iter_mut() {
        tf.translation.z = ysort.z - (1.0f32 / (1.0f32 + (2.0f32.powf(-0.01 * tf.translation.y))));
    }
}

fn clean_up_enemies(mut commands: Commands, enemy_q: Query<Entity, With<Enemy>>) {
    for entity in enemy_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
