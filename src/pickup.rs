use crate::player::Player;
use crate::utils::YSort;
use crate::GameState;
use bevy::prelude::*;
use rand::Rng;
pub struct PickupPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PickupPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(PickupTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(
            Update,
            (spawn_pickup, pickup_hover)
                .run_if(in_state(self.state.clone()))
                .run_if(in_state(GameState::Playing)),
        );
        app.add_systems(OnExit(self.state.clone()), clean_up_pickups);
    }
}
#[derive(Component)]
pub struct Pickup;
#[derive(Component)]
struct PickupShadow;
#[derive(Resource)]
struct PickupTimer(Timer);
fn spawn_pickup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
    mut pickup_timer: ResMut<PickupTimer>,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };
    let mut rng = rand::thread_rng();

    if pickup_timer.0.tick(time.delta()).just_finished() {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance = rng.gen_range(100.0..200.0);

        let offset = Vec2::new(angle.cos() * distance, angle.sin() * distance);
        let pickup_pos = player_transform.translation.truncate() + offset;

        let pickup_entity = commands
            .spawn((
                Sprite::from_image(asset_server.load("candycane.png")),
                Transform::from_translation(pickup_pos.extend(1.0)),
                Pickup,
            ))
            .id();
        let shadow_entity = commands
            .spawn((
                Sprite::from_image(asset_server.load("shadow.png")),
                Transform {
                    translation: Vec3::new(-1.0, -7.0, 0.0),
                    rotation: Quat::from_rotation_z(0.0),
                    scale: Vec3::new(1.5, 1.5, 1.0),
                },
                PickupShadow,
                YSort { z: -100.0 },
            ))
            .id();
        commands.entity(pickup_entity).add_child(shadow_entity);
        println!("Spawned pickup");
    }
}
fn pickup_hover(
    mut q_pickup: Query<&mut Transform, (With<Pickup>, Without<PickupShadow>)>,
    mut q_shadow: Query<&mut Transform, With<PickupShadow>>,
    time: Res<Time>,
) {
    let time = time.elapsed_secs();
    for mut shadow_transform in q_shadow.iter_mut() {
        shadow_transform.translation.y -= (time * 4.0).sin() * 0.1;
    }
    for mut transform in q_pickup.iter_mut() {
        transform.translation.y += (time * 4.0).sin() * 0.1;
    }
}
fn clean_up_pickups(mut commands: Commands, pickup_query: Query<Entity, With<Pickup>>) {
    for pickup in pickup_query.iter() {
        commands.entity(pickup).despawn_recursive();
    }
}
