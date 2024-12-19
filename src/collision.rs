use crate::enemy::Enemy;
use crate::enemy::EnemyHealth;
use crate::player::Player;
use crate::player::Projectile;
use crate::player::Shield;
use bevy::prelude::*;

const KNOCKBACK_STRENGTH: f32 = 7.0;
const FRICTION: f32 = 0.5;

pub struct CollisionPlugin;

#[derive(Component)]
struct Knockback {
    direction: Vec2,
    strength: f32,
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, projectiles_collision);
        app.add_systems(FixedUpdate, enemy_collision);
        app.add_systems(FixedUpdate, knockback_system);
        app.add_systems(FixedUpdate, shield_collision);
    }
}
fn knockback_system(
    mut q: Query<(&mut Transform, &mut Knockback, Entity)>,
    mut commands: Commands,
) {
    for (mut tf, mut knockback, entity) in q.iter_mut() {
        let velocity = knockback.direction * knockback.strength;
        tf.translation += velocity.extend(0.0);

        knockback.strength = (knockback.strength - FRICTION).max(0.0);

        if knockback.strength <= 0.0 {
            commands.entity(entity).remove::<Knockback>();
        }
    }
}
fn projectiles_collision(
    mut commands: Commands,
    projectiles_q: Query<(Entity, &Transform), (With<Projectile>, Without<Enemy>)>,
    mut enemies_q: Query<(&mut EnemyHealth, &Transform, Entity), With<Enemy>>,
) {
    for (projectile_entity, projectile_tf) in projectiles_q.iter() {
        for (mut health, enemy_tf, enemy_entity) in enemies_q.iter_mut() {
            let pos1 = projectile_tf.translation.truncate();
            let pos2 = enemy_tf.translation.truncate();
            let dist = pos1.distance(pos2);
            if dist < 16.0 {
                health.health -= 34;
                println!("enemy destroyed");
                // Apply knockback to enemy
                let knockback_direction = (pos2 - pos1).normalize();
                commands.entity(enemy_entity).insert(Knockback {
                    direction: knockback_direction,
                    strength: KNOCKBACK_STRENGTH,
                });
                commands.entity(projectile_entity).despawn_recursive();
            }
        }
    }
}
fn shield_collision(
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
    q_shield: Query<(&GlobalTransform, &Shield), With<Shield>>,
    mut q_enemy: Query<(&Transform, &mut EnemyHealth, Entity), (With<Enemy>, Without<Shield>)>,
) {
    let player_tf = q_player.single();
    for (shield_tf, shield) in q_shield.iter() {
        for (mut enemy_tf, mut enemy_health, enemy_entity) in q_enemy.iter_mut() {
            let pos1 = shield_tf.translation().truncate();
            let pos2 = enemy_tf.translation.truncate();
            let dist = pos1.distance(pos2);
            if dist < 16.0 {
                enemy_health.health -= shield.damage as i32;
                let knockback_direction = (pos2 - player_tf.translation.truncate()).normalize();
                commands.entity(enemy_entity).insert(Knockback {
                    direction: knockback_direction,
                    strength: KNOCKBACK_STRENGTH,
                });
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
