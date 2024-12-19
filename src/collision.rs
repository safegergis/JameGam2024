use crate::enemy::Enemy;
use crate::enemy::EnemyHealth;
use crate::player::Projectile;
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, projectiles_collision);
        app.add_systems(FixedUpdate, enemy_collision);
    }
}

fn projectiles_collision(
    mut commands: Commands,
    projectiles_q: Query<(Entity, &Transform), With<Projectile>>,
    mut enemies_q: Query<(&mut EnemyHealth, &Transform), With<Enemy>>,
) {
    for (entity, tf1) in projectiles_q.iter() {
        for (mut health, tf2) in enemies_q.iter_mut() {
            let pos1 = tf1.translation.truncate();
            let pos2 = tf2.translation.truncate();
            let dist = pos1.distance(pos2);
            if dist < 16.0 {
                health.health -= 34;
                println!("enemy destroyed");
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
fn enemy_collision(mut q: Query<&mut Transform, With<Enemy>>) {
    let mut combinations = q.iter_combinations_mut();
    while let Some([mut tf1, mut tf2]) = combinations.fetch_next() {
        let pos1 = tf1.translation.truncate();
        let pos2 = tf2.translation.truncate();

        let collision_dist = 16.0;
        let dist = pos1.distance(pos2);

        if dist < collision_dist {
            // Calculate push direction and amount
            let push_dir = (pos1 - pos2).normalize();
            let push_amount = (collision_dist - dist) / 2.0;

            // Push both enemies apart equally
            tf1.translation += (push_dir * push_amount).extend(0.0);
            tf2.translation += (-push_dir * push_amount).extend(0.0);
        }
    }
}
