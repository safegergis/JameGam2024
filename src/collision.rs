use std::time::Duration;

use crate::enemy::Enemy;
use crate::enemy::EnemyHealth;
use crate::enemy::EnemyXp;
use crate::pickup::Pickup;

use crate::player::Player;
use crate::player::PlayerHealth;
use crate::player::PlayerSnowball;
use crate::player::PlayerXp;
use crate::player::PoweredUp;
use crate::player::Projectile;
use crate::player::Shield;

use bevy::prelude::*;

const IFRAME_DURATION: f32 = 0.05;
const FLASH_DURATION: f32 = 0.05;
const KNOCKBACK_STRENGTH: f32 = 4.0;
const FRICTION: f32 = 0.5;

pub struct CollisionPlugin<S: States> {
    pub state: S,
}

#[derive(Component)]
struct Knockback {
    direction: Vec2,
    strength: f32,
}

impl<S: States> Plugin for CollisionPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                projectiles_collision,
                enemy_collision,
                knockback_system,
                shield_collision,
                player_collision,
                xp_collision,
                pickup_colliisions,
            )
                .run_if(in_state(self.state.clone())),
        );
        app.add_systems(Update, (blinking, flashing, invincible).run_if(in_state(self.state.clone())));
        app.add_systems(OnExit(self.state.clone()), cleanup_xp);
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
    xp_q: Query<(&Transform, Entity, &EnemyXp), Without<Player>>,
    mut player_q: Query<(&Transform, &mut PlayerXp), With<Player>>,
) {
    let (player_tf, mut player_xp) = player_q.single_mut();
    for (xp_tf, xp_entity, xp) in xp_q.iter() {
        let pos1 = xp_tf.translation.truncate();
        let pos2 = player_tf.translation.truncate();
        let dist = pos1.distance(pos2);
        if dist < 16.0 {
            player_xp.xp += xp.xp;
            //println!("Player XP: {}", player_xp.xp);
            commands.entity(xp_entity).despawn_recursive();
        }
    }
}
fn projectiles_collision(
    mut commands: Commands,
    mut projectiles_q: Query<(Entity, &Transform, &mut Projectile), (Without<Enemy>)>,
    mut enemies_q: Query<(&mut EnemyHealth, &Transform, Entity, &Children), (With<Enemy>, Without<InvincibleTimer>)>,
) {
    for (projectile_entity, projectile_tf, mut projectile) in projectiles_q.iter_mut() {
        for (mut health, enemy_tf, enemy_entity, enemy_children) in enemies_q.iter_mut() {
            let pos1 = projectile_tf.translation.truncate();
            let pos2 = enemy_tf.translation.truncate();
            let dist = pos1.distance(pos2);
            if dist < 16.0 {
                health.health -= 34;
                //println!("enemy destroyed");
                // Apply knockback to enemy
                let knockback_direction = (pos2 - pos1).normalize();
                commands.entity(enemy_entity).insert(Knockback {
                    direction: knockback_direction,
                    strength: KNOCKBACK_STRENGTH,
                });

                if projectile.pierce_amount > 0
                {
                    projectile.pierce_amount = projectile.pierce_amount - 1;
                }else {               
                    commands.entity(projectile_entity).despawn_recursive();
                }
                
                commands.entity(enemy_children[1]).insert(FlashingTimer { 
                    time_left: FLASH_DURATION, 
                    color: Color::srgba(12., 12., 12., 1.),});
                commands.entity(enemy_entity).insert(InvincibleTimer { time_left: IFRAME_DURATION, });
            }
        }
    }
}
fn shield_collision(
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
    q_shield: Query<(&GlobalTransform, &Shield), With<Shield>>,
    mut q_enemy: Query<
        (&Transform, &mut EnemyHealth, Entity, &Children),
        (With<Enemy>, Without<Shield>, Without<InvincibleTimer>),
    >,
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
                commands.entity(enemy_children[1]).insert(FlashingTimer { 
                    time_left: FLASH_DURATION, 
                    color: Color::srgba(12., 12., 12., 1.),
                });
                commands.entity(enemy_entity).insert(InvincibleTimer { time_left: IFRAME_DURATION, });
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
fn pickup_colliisions(
    mut commands: Commands,
    q_pickup: Query<(&Transform, Entity), (With<Pickup>, Without<Player>)>,
    q_player: Query<(&Transform, Entity), With<Player>>,
) {
    let (player_tf, player_entity) = q_player.single();
    for (pickup_tf, pickup_entity) in q_pickup.iter() {
        let pos1 = pickup_tf.translation.truncate();
        let pos2 = player_tf.translation.truncate();
        let dist = pos1.distance(pos2);
        if dist < 16.0 {
            commands.entity(pickup_entity).despawn_recursive();
            commands.entity(player_entity).insert(PoweredUp {
                timer: Timer::from_seconds(5.0, TimerMode::Once),
            });
            commands.entity(player_entity).insert(Blink {
                color: Color::srgba(0., 1., 0., 1.),
                speed: 1.0,
            });
        }
    }
}
fn player_collision(
    mut commands: Commands,
    mut q_player: Query<(&mut PlayerHealth, Entity, &mut Player, Option<&InvincibleTimer>), Without<PlayerSnowball>>,
    mut q_player_snowball: Query<&mut GlobalTransform, With<PlayerSnowball>>,
    q_player_poweredup: Query<Entity, (With<PoweredUp>, Without<PlayerSnowball>)>,
    mut q_enemy: Query<(&Transform, &mut EnemyHealth, Entity), With<Enemy>>,
) {
    let (mut player_health, player_entity, mut player, iframes) = q_player.single_mut();
    let player_snowball_tf = q_player_snowball.single_mut();
    for (enemy_tf, mut enemy_health, enemy_entity) in q_enemy.iter_mut() {
        let pos1 = player_snowball_tf.translation().truncate();
        let pos2 = enemy_tf.translation.truncate();
        let dist = pos1.distance(pos2);
        let collision_radius = 12.0 * player_snowball_tf.scale().x;
        let player_poweredup: bool = q_player_poweredup.get_single().is_ok();
        if !player_poweredup {
            if dist < collision_radius {
                let collision_direction = (pos2 - pos1).normalize();
                if let Some(iframes) = iframes{

            }else {
                player_health.hp -= 3.0;
                player.velocity = player.velocity * 0.5;

                commands.entity(player_entity).insert(FlashingTimer { 
                    time_left: FLASH_DURATION, 
                    color: Color::srgba(12., 12., 12., 1.),
                });
                commands.entity(player_entity).insert(InvincibleTimer { time_left: IFRAME_DURATION, });
            }
            commands.entity(enemy_entity).insert(Knockback {
                direction: collision_direction,
                strength: KNOCKBACK_STRENGTH,
            });
            }
        } else {
            if dist < collision_radius {
                let collision_direction = (pos2 - pos1).normalize();
                enemy_health.health -= 25;
                commands.entity(enemy_entity).insert(Knockback {
                    direction: collision_direction,
                    strength: KNOCKBACK_STRENGTH,
                });
            }
        }
    }
}

#[derive(Component)]
struct FlashingTimer {
    time_left: f32,
    color: Color,
}

#[derive(Component)]
struct InvincibleTimer {
    time_left: f32,
}

fn flashing(
    mut commands: Commands,
    mut flashing_query: Query<(&mut FlashingTimer, Entity, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut timer, timer_e, mut timer_sprite) in flashing_query.iter_mut() {
        timer_sprite.color = timer.color; // bright white color

        timer.time_left -= time.delta_secs();

        if timer.time_left <= 0.0 {
            timer_sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0); // resets the color back to normal
            commands.entity(timer_e).remove::<FlashingTimer>(); // removes the FlashingTimer component from the entity
        }
    }
}
#[derive(Component)]
pub struct Blink {
    pub color: Color,
    pub speed: f32,
}

fn blinking(mut blinking_query: Query<(&Blink, &mut Sprite)>, time: Res<Time>) {
    for (blink, mut blink_sprite) in blinking_query.iter_mut() {
        let sin = 0.5 * ((time.elapsed_secs() * blink.speed * std::f32::consts::PI).sin() + 1.);
        let srgba: Srgba = blink.color.into();
        blink_sprite.color = Color::srgba(
            1. + srgba.red * sin,
            1. + srgba.green * sin,
            1. + srgba.blue * sin,
            1.,
        );
    }
}

fn invincible(
    mut commands: Commands,
    mut invincible_query: Query<(&mut InvincibleTimer, Entity)>,
    time: Res<Time>,
) {
    for (mut timer, timer_e) in invincible_query.iter_mut() {
        timer.time_left -= time.delta_secs();
        if timer.time_left <= 0.0 {
            commands.entity(timer_e).remove::<InvincibleTimer>(); // removes the FlashingTimer component from the entity
        }
    }
}

fn cleanup_xp(mut commands: Commands, xp_q: Query<Entity, With<EnemyXp>>) {
    for entity in xp_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
