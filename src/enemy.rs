use bevy::ecs::{bundle, component};
use bevy::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyTimer(Timer::from_seconds(0.01, TimerMode::Repeating)));
        app.add_systems(Update, (chase_player, wiggle, y_sort));
    }
}

#[derive(Resource)]
struct EnemyTimer(Timer);

fn spawn_enemy(
    time: Res<Time>,
    mut timer: ResMut<EnemyTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let circle = Circle::new(200.0);
    let boundary_pt = circle.sample_boundary(&mut rand::thread_rng());
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Sprite::from_image(asset_server.load("snowman.png")),
            Transform::from_xyz(boundary_pt.x, boundary_pt.y, 2.0),
            YSort { z: 32.0 },
            ChasePlayer { speed: 25.0 },
            Wiggle {
                speed: 15.0,
                amount: 2.0,
                offset: time.elapsed_secs(),
            },
        ));
    }
}

#[derive(Component)]
struct Wiggle {
    speed: f32,
    amount: f32,
    offset: f32,
}

fn wiggle(time: Res<Time>, mut q: Query<(&mut Transform, &Wiggle)>) {
    for (mut tf, wiggle) in q.iter_mut() {
        let sin_amount = f32::sin(time.elapsed_secs() * wiggle.speed);
        let dt = time.delta_secs() * wiggle.amount as f32;
        tf.rotate_z(sin_amount * dt);
    }
}

#[derive(Component)]
struct ChasePlayer {
    speed: f32,
}

fn chase_player(time: Res<Time>, mut q: Query<(&mut Transform, &ChasePlayer)>) {
    for (mut tf, chaseplayer) in q.iter_mut() {
        let dt = time.delta_secs() * chaseplayer.speed as f32;
        let mut dir = (Vec3::new(0.0, 0.0, 0.0) - tf.translation).normalize();
        dir.z = 0.0;
        tf.translation += dir * dt;
    }
}

#[derive(Component)]
struct YSort {
    z: f32,
}

fn y_sort(mut q: Query<(&mut Transform, &YSort)>) {
    for (mut tf, ysort) in q.iter_mut() {
        tf.translation.z = ysort.z - (1.0f32 / (1.0f32 + (2.0f32.powf(-0.01 * tf.translation.y))));
    }
}

#[derive(Component)]
struct Enemy {
    x: f32,
    y: f32,
}
