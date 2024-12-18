use crate::utils::YSort;
use bevy::prelude::*;
use rand::Rng;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyTimer(Timer::from_seconds(0.01, TimerMode::Repeating)));
        app.add_systems(Update, (chase_player, spawn_enemy, wiggle, y_sort));
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
        let num_offset = rand::thread_rng().gen_range(-1.0..1.0);

        let snowman_holder = commands
            .spawn((
                Visibility::Visible,
                Transform::from_xyz(boundary_pt.x, boundary_pt.y, 2.0),
                YSort { z: 32.0 },
                Enemy { speed: 25.0 },
            ))
            .id();

        let snowman_sprite = commands
            .spawn((
                Sprite::from_image(asset_server.load("Snowman2.png")),
                Wiggle {
                    rotate_speed: 18.0,
                    rotate_amount: 0.02,
                    scale_speed: 18.0,
                    scale_amount: 0.15,
                    offset: num_offset,
                },
            ))
            .id();

        let snowman_shadow = commands
            .spawn((
                Sprite::from_image(asset_server.load("Shadow.png")),
                YSort { z: -100.0 },
            ))
            .id();
        commands.entity(snowman_holder).add_child(snowman_shadow);
        commands.entity(snowman_holder).add_child(snowman_sprite);
    }
}

#[derive(Component)]
struct Wiggle {
    rotate_speed: f32,
    rotate_amount: f32,
    scale_speed: f32,
    scale_amount: f32,
    offset: f32,
}

fn wiggle(time: Res<Time>, mut q: Query<(&mut Transform, &Wiggle)>) {
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
struct Enemy {
    speed: f32,
}

fn chase_player(time: Res<Time>, mut q: Query<(&mut Transform, &Enemy)>) {
    for (mut tf, enemy) in q.iter_mut() {
        let dt = time.delta_secs() * enemy.speed as f32;
        let mut dir = (Vec3::new(0.0, 0.0, 0.0) - tf.translation).normalize();
        dir.z = 0.0;
        tf.translation += dir * dt;
    }
}

fn y_sort(mut q: Query<(&mut Transform, &YSort)>) {
    for (mut tf, ysort) in q.iter_mut() {
        tf.translation.z = ysort.z - (1.0f32 / (1.0f32 + (2.0f32.powf(-0.01 * tf.translation.y))));
    }
}
