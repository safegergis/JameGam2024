use crate::enemy;
use crate::enemy::Enemy;
use crate::enemy::EnemyHealth;
use crate::enemy::EnemyXp;
use crate::player::Player;
use crate::player::PlayerXp;
use crate::player::Projectile;
use crate::player::Shield;
use bevy::prelude::*;

const FLASH_DURATION: f32 = 0.05;
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
        app.add_systems(FixedUpdate, xp_collision);
        app.add_systems(Update, flashing);
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
fn xp_collision(
    mut commands: Commands,
    xp_q: Query<(&Transform, Entity, &EnemyXp), (Without<Player>)>,
    mut player_q: Query<(&Transform, &mut PlayerXp), With<Player>>,
) {
    let (player_tf, mut player_xp) = player_q.single_mut();
    for (xp_tf, xp_entity, xp) in xp_q.iter() {
        let pos1 = xp_tf.translation.truncate();
        let pos2 = player_tf.translation.truncate();
        let dist = pos1.distance(pos2);
        if dist < 16.0 {
            player_xp.xp += xp.xp;
            println!("Player XP: {}", player_xp.xp);
            commands.entity(xp_entity).despawn_recursive();
        }
    }
}
fn projectiles_collision(
    mut commands: Commands,
    projectiles_q: Query<(Entity, &Transform), (With<Projectile>, Without<Enemy>)>,
    mut enemies_q: Query<(&mut EnemyHealth, &Transform, Entity, &Children), With<Enemy>>,
) {
    for (projectile_entity, projectile_tf) in projectiles_q.iter() {
        for (mut health, enemy_tf, enemy_entity, enemy_children) in enemies_q.iter_mut() {
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
                commands.entity(enemy_children[1]).insert(FlashingTimer { time_left: FLASH_DURATION, });
            }
        }
    }
}
fn shield_collision(
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
    q_shield: Query<(&GlobalTransform, &Shield), With<Shield>>,
    mut q_enemy: Query<(&Transform, &mut EnemyHealth, Entity, &Children), (With<Enemy>, Without<Shield>)>,
) {
    let player_tf = q_player.single();
    for (shield_tf, shield) in q_shield.iter() {
        for (enemy_tf, mut enemy_health, enemy_entity, enemy_children) in q_enemy.iter_mut() {
            let pos1 = shield_tf.translation().truncate();
            let pos2 = enemy_tf.translation.truncate();
            let dist = pos1.distance(pos2);
            if dist < 16.0 {
                enemy_health.health -= shield.damage as i32;
                let knockback_direction = (pos2 - player_tf.translation.truncate()).normalize();
                commands.entity(enemy_children[1]).insert(FlashingTimer { time_left: FLASH_DURATION, });
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


#[derive(Component)]
struct FlashingTimer { time_left: f32 }

fn flashing (
    mut commands: Commands, 
    mut flashing_query: Query<(&mut FlashingTimer, Entity, &mut Sprite)>, 
    time: Res<Time>, 
) {
    for (mut timer, timer_e, mut timer_sprite) in flashing_query.iter_mut() { 
        print!("asd\n");
        timer_sprite.color = Color::srgba(8., 8., 8., 1.); // bright white color 
        
        timer.time_left -= time.delta_secs();
        
        if timer.time_left <= 0.0 {
            timer_sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0); // resets the color back to normal
            commands.entity(timer_e).remove::<FlashingTimer>(); // removes the FlashingTimer component from the entity
        }
} }
