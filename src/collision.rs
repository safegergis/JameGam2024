use crate::enemy::Enemy;
use crate::enemy::EnemyCount;
use crate::enemy::EnemyHealth;
use crate::enemy::EnemyXp;
use crate::enemy::Frozen;
use crate::enemy::OnFire;
use crate::enemy::Vunerable;
use crate::pickup::Pickup;
use crate::player::AnimationIndices;
use crate::player::AnimationTimer;
use crate::player::PlayerStats;
use crate::utils::YSort;
use rand::Rng;

use crate::player::Player;
use crate::player::PlayerHealth;
use crate::player::PlayerSnowball;
use crate::player::PlayerXp;
use crate::player::PoweredUp;
use crate::player::Projectile;
use crate::player::Shield;
use crate::GameState;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;

const IFRAME_DURATION: f32 = 0.1;
const FLASH_DURATION: f32 = 0.1;
const KNOCKBACK_STRENGTH: f32 = 4.0;
const FRICTION: f32 = 0.2;

pub struct CollisionPlugin<S: States> {
    pub state: S,
}

#[derive(Component)]
struct Knockback {
    direction: Vec2,
    strength: f32,
}

#[derive(Component)]
struct CheckIfFreeze;
#[derive(Component)]
struct CheckIfFire;

impl<S: States> Plugin for CollisionPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                (projectiles_collision, fire_check, freeze_check).chain(),
                enemy_collision,
                knockback_system,
                shield_collision,
                player_collision,
                xp_collision,
                pickup_colliisions,
            )
                .run_if(in_state(self.state.clone()))
                .run_if(in_state(GameState::Playing)),
        );
        app.add_systems(
            Update,
            (blinking, flashing, invincible, destroy_after)
                .run_if(in_state(self.state.clone()))
                .run_if(in_state(GameState::Playing)),
        );
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
    mut projectiles_q: Query<
        (Entity, &Transform, &mut Projectile, Option<&Vunerable>),
        Without<Enemy>,
    >,
    mut enemies_q: Query<
        (&mut EnemyHealth, &Transform, Entity, &Children),
        (With<Enemy>, Without<InvincibleTimer>),
    >,
    asset_server: Res<AssetServer>,
    stats: Res<PlayerStats>,
) {
    for (projectile_entity, projectile_tf, mut projectile, vunerable) in projectiles_q.iter_mut() {
        for (mut health, enemy_tf, enemy_entity, enemy_children) in enemies_q.iter_mut() {
            let pos1 = projectile_tf.translation.truncate();
            let pos2 = enemy_tf.translation.truncate();
            let dist = pos1.distance(pos2);
            if dist < 16.0 {
                let mut multiplier: f32 = 1.0;
                if let Some(_vunerable) = vunerable {
                    multiplier = _vunerable.multiplier;
                }
                health.health -= stats.damage * multiplier;
                //println!("enemy destroyed");
                // Apply knockback to enemy
                let knockback_direction = (pos2 - pos1).normalize();
                commands.entity(enemy_entity).insert(Knockback {
                    direction: knockback_direction,
                    strength: stats.knockback_strength,
                });

                if projectile.pierce_amount > 0 {
                    projectile.pierce_amount = projectile.pierce_amount - 1;
                } else {
                    commands.entity(projectile_entity).despawn_recursive();
                }

                commands.entity(enemy_children[1]).insert(FlashingTimer {
                    time_left: FLASH_DURATION,
                    color: Color::srgba(12., 12., 12., 1.),
                });
                commands.entity(enemy_entity).insert(InvincibleTimer {
                    time_left: IFRAME_DURATION,
                });
                commands.entity(enemy_entity).insert((
                    AudioPlayer::new(asset_server.load("sounds/hit1.ogg")),
                    PlaybackSettings {
                        mode: PlaybackMode::Remove,
                        ..default()
                    },
                ));
                let random_freeze_chance = rand::thread_rng().gen_range(1..100);
                if stats.freeze_chance >= random_freeze_chance {
                    commands.entity(enemy_entity).insert(CheckIfFreeze);
                }

                let random_burn_chance = rand::thread_rng().gen_range(1..100);
                if stats.fire_chance >= random_burn_chance {
                    commands.entity(enemy_entity).insert(CheckIfFire);
                }
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
    asset_server: Res<AssetServer>,
    player_stats: Res<PlayerStats>,
) {
    let Ok(player_tf) = q_player.get_single() else {
        return;
    };
    for (shield_tf, shield) in q_shield.iter() {
        for (enemy_tf, mut enemy_health, enemy_entity, enemy_children) in q_enemy.iter_mut() {
            let pos1 = shield_tf.translation().truncate();
            let pos2 = enemy_tf.translation.truncate();
            let dist = pos1.distance(pos2);
            if dist < 16.0 {
                enemy_health.health -= shield.damage;
                let knockback_direction = (pos2 - player_tf.translation.truncate()).normalize();
                commands.entity(enemy_children[1]).insert(FlashingTimer {
                    time_left: FLASH_DURATION,
                    color: Color::srgba(12., 12., 12., 1.),
                });
                commands.entity(enemy_entity).insert(InvincibleTimer {
                    time_left: IFRAME_DURATION,
                });
                commands.entity(enemy_entity).insert(Knockback {
                    direction: knockback_direction,
                    strength: player_stats.knockback_strength,
                });
                commands.entity(enemy_entity).insert((
                    AudioPlayer::new(asset_server.load("sounds/hit1.ogg")),
                    PlaybackSettings {
                        mode: PlaybackMode::Remove,
                        ..default()
                    },
                ));
                let random_freeze_chance = rand::thread_rng().gen_range(1..100);
                if player_stats.shield_apply_effects {
                    if player_stats.freeze_chance >= random_freeze_chance {
                        commands.entity(enemy_entity).insert(CheckIfFreeze);
                    }

                    let random_burn_chance = rand::thread_rng().gen_range(1..100);
                    if player_stats.fire_chance >= random_burn_chance {
                        commands.entity(enemy_entity).insert(CheckIfFire);
                    }
                }
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
    asset_server: Res<AssetServer>,
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
            commands.entity(player_entity).insert((
                AudioPlayer::new(asset_server.load("sounds/powerup.ogg")),
                PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    ..default()
                },
            ));
        }
    }
}
fn player_collision(
    mut commands: Commands,
    mut q_player: Query<
        (
            &mut PlayerHealth,
            Entity,
            &mut Player,
            Option<&InvincibleTimer>,
        ),
        Without<PlayerSnowball>,
    >,
    mut q_player_snowball: Query<&mut GlobalTransform, With<PlayerSnowball>>,
    q_player_poweredup: Query<Entity, (With<PoweredUp>, Without<PlayerSnowball>)>,
    mut q_enemy: Query<(&mut Transform, &mut EnemyHealth, Entity), With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut enemy_count: ResMut<EnemyCount>,
    player_stats: Res<PlayerStats>,

) {
    let Ok((mut player_health, player_entity, mut player, iframes)) = q_player.get_single_mut()
    else {
        return;
    };
    let player_snowball_tf = q_player_snowball.single_mut();
    for (mut enemy_tf, mut enemy_health, enemy_entity) in q_enemy.iter_mut() {
        let pos1 = player_snowball_tf.translation().truncate();
        let pos2 = enemy_tf.translation.truncate();
        let dist = pos1.distance(pos2);
        //let collision_radius = 12.0 * player_snowball_tf.scale().x;
        let collision_radius = 10.0 * player_snowball_tf.scale().x; //Made a little smaller to forgive players
        let max_collision_radius: f32 = 500.;
        let player_poweredup: bool = q_player_poweredup.get_single().is_ok();

        if(dist > max_collision_radius)
        {
            // enemy_tf.translation = ((pos1 * 2.) - pos2).extend(0.);
            // commands.entity(enemy_entity).insert(InvincibleTimer{
            //     time_left: 1.,
            // });
            
            enemy_count.enemy_count -= 1;
            commands.entity(enemy_entity).despawn_recursive();
            continue;
        }

        if !player_poweredup {
            if dist < collision_radius {
                let collision_direction = (pos2 - pos1).normalize();
                if let Some(iframes) = iframes {
                } else {
                    player_health.hp -= 3.0;
                    player.velocity = player.velocity * 0.5;

                    commands.entity(player_entity).insert((
                        AudioPlayer::new(asset_server.load("sounds/hit2.ogg")),
                        PlaybackSettings {
                            mode: PlaybackMode::Remove,
                            ..default()
                        },
                    ));

                    commands.entity(player_entity).insert(FlashingTimer {
                        time_left: FLASH_DURATION,
                        color: Color::srgba(12., 12., 12., 1.),
                    });
                    commands.entity(player_entity).insert(InvincibleTimer {
                        time_left: IFRAME_DURATION,
                    });
                }
                commands.entity(enemy_entity).insert(Knockback {
                    direction: collision_direction,
                    strength: player_stats.knockback_strength,
                });
            }
        } else {
            if dist < collision_radius {
                let collision_direction = (pos2 - pos1).normalize();
                enemy_health.health -= player_stats.snowball_damage_multiplier * 25.;
                commands.entity(enemy_entity).insert(Knockback {
                    direction: collision_direction,
                    strength: player_stats.knockback_strength * 2.0,
                });
            }
        }
    }
}

#[derive(Component)]
pub struct FlashingTimer {
    pub time_left: f32,
    pub color: Color,
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

#[derive(Component)]
pub struct DestroyAfter {
    pub duration: f32,
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

fn destroy_after(
    mut commands: Commands,
    mut q: Query<(&mut DestroyAfter, Entity)>,
    time: Res<Time>,
) {
    for (mut destroy_after, destroy_entity) in q.iter_mut() {
        let dt = time.delta_secs();
        destroy_after.duration -= dt;
        if destroy_after.duration <= 0.0 {
            commands.entity(destroy_entity).despawn_recursive();
        }
    }
}

fn freeze_check(
    mut commands: Commands,
    mut q_entity: Query<(
        &CheckIfFreeze,
        &mut EnemyHealth,
        Entity,
        &Children,
        Option<&Frozen>,
        Option<&OnFire>,
        Option<&CheckIfFire>,
    )>,
    fire_query: Query<(&OnFire, Entity, Option<&DestroyAfter>)>,
    stats: Res<PlayerStats>,
    asset_server: Res<AssetServer>,
) {
    for (
        _check_freeze,
        mut enemy_health,
        enemy_entity,
        enemy_children,
        frozen,
        on_fire,
        fire_check,
    ) in q_entity.iter_mut()
    {
        if let Some(frozen) = frozen {
            commands.entity(enemy_entity).remove::<CheckIfFreeze>();
        } else {
            // Check if on fire
            if let Some(_on_fire) = on_fire {
                if let Some(_fire_check) = fire_check {
                    commands.entity(enemy_entity).remove::<CheckIfFire>();
                }

                if stats.flash_freeze {
                    enemy_health.health -= enemy_health.health * 0.15;
                }

                commands.entity(enemy_entity).remove::<OnFire>();
                commands.entity(enemy_children[1]).remove::<Blink>();
                commands.entity(enemy_children[1]).insert(FlashingTimer {
                    time_left: 0.0,
                    color: Color::srgba(1., 1., 1., 1.),
                });
                for (_child, child_entity, to_destroy) in fire_query.iter_many(enemy_children) {
                    commands.entity(child_entity).remove::<OnFire>();

                    if let Some(_to_destroy) = to_destroy {
                        commands.entity(child_entity).despawn_recursive();
                    }
                }
            }
            ////////////////////////////////////////////////

            commands.entity(enemy_entity).insert(Frozen {
                duration: stats.freeze_duration,
            });
            commands.entity(enemy_children[1]).insert(Frozen {
                duration: stats.freeze_duration,
            });

            let freeze_sprite = commands
                .spawn((
                    Sprite::from_image(asset_server.load("Freeze.png")),
                    YSort { z: 0.6 },
                    Frozen {
                        duration: stats.freeze_duration,
                    },
                    DestroyAfter {
                        duration: stats.freeze_duration,
                    },
                ))
                .id();

            commands.entity(enemy_entity).add_child(freeze_sprite);
        }
    }
}

fn fire_check(
    mut commands: Commands,
    mut q_entity: Query<(
        &CheckIfFire,
        Entity,
        &Children,
        Option<&OnFire>,
        Option<&Frozen>,
        Option<&CheckIfFreeze>,
    )>,
    frozen_query: Query<(&Frozen, Entity, Option<&DestroyAfter>)>,
    stats: Res<PlayerStats>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (_check_fire, enemy_entity, enemy_children, on_fire, frozen, freeze_check) in
        q_entity.iter_mut()
    {
        if let Some(on_fire) = on_fire {
            commands.entity(enemy_entity).remove::<CheckIfFire>();
        } else {
            //Check if frozen
            if let Some(_frozen) = frozen {
                for (_child, child_entity, to_destroy) in frozen_query.iter_many(enemy_children) {
                    commands.entity(child_entity).remove::<Frozen>();

                    if let Some(_to_destroy) = to_destroy {
                        commands.entity(child_entity).despawn_recursive();
                    }
                }

                commands.entity(enemy_entity).remove::<Frozen>();
                if let Some(_freeze_check) = freeze_check {
                    commands.entity(enemy_entity).remove::<CheckIfFreeze>();
                }

                if stats.freezer_burn {
                    commands.entity(enemy_entity).insert(Vunerable {
                        duration: stats.freezer_burn_duration,
                        multiplier: stats.freezer_burn_multiplier,
                    });
                }
            }
            //////////////////////////////////////////////

            commands.entity(enemy_entity).insert(OnFire {
                duration: stats.fire_duration,
            });
            commands.entity(enemy_children[1]).insert(Blink {
                color: Color::srgba(12., 12., 12., 1.),
                speed: 10.0,
            });

            let texture = asset_server.load("fire.png");
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(36), 2, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let animation_indices = AnimationIndices { first: 0, last: 1 };
            let fire_sprite = commands
                .spawn((
                    YSort { z: 0.6 },
                    OnFire {
                        duration: stats.fire_duration,
                    },
                    DestroyAfter {
                        duration: stats.fire_duration,
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
                ))
                .id();

            commands.entity(enemy_entity).add_child(fire_sprite);
        }
    }
}
