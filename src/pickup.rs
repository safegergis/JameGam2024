use crate::player::Player;
use bevy::prelude::*;
use rand::Rng;
pub struct PickupPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PickupPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state.clone()), spawn_pickup);
        app.insert_resource(PickupTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
        app.add_systems(Update, spawn_pickup.run_if(in_state(self.state.clone())));
        app.add_systems(Update, pickup_hover.run_if(in_state(self.state.clone())));
        app.add_systems(OnExit(self.state.clone()), clean_up_pickups);
    }
}
#[derive(Component)]
struct Pickup;
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
    let player_transform = player.single();
    let mut rng = rand::thread_rng();

    if pickup_timer.0.tick(time.delta()).just_finished() {
        // Spawn initial 1-2 pickups
        let num_pickups = rng.gen_range(1..=2);
        for _ in 0..num_pickups {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let distance = rng.gen_range(50.0..150.0);

            let offset = Vec2::new(angle.cos() * distance, angle.sin() * distance);
            let pickup_pos = player_transform.translation.truncate() + offset;

            commands.spawn((
                Sprite::from_image(asset_server.load("candycane.png")),
                Transform::from_translation(pickup_pos.extend(1.0)),
                Pickup,
            ));
            commands.spawn((
                Sprite::from_image(asset_server.load("shadow.png")),
                Transform::from_translation(pickup_pos.extend(1.0) - Vec3::new(0.0, 8.0, 0.0)),
                PickupShadow,
            ));
            println!("Spawned pickup");
        }
    }
}
fn pickup_hover(mut query: Query<&mut Transform, With<Pickup>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let time = time.elapsed_secs();
        transform.translation.y += (time * 2.0).sin() * 0.1;
    }
}
fn clean_up_pickups(mut commands: Commands, pickup_query: Query<Entity, With<Pickup>>) {
    for pickup in pickup_query.iter() {
        commands.entity(pickup).despawn_recursive();
    }
}
