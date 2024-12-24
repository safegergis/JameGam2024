use crate::player::PlayerStats;
use crate::AppState;
use crate::GameState;
use crate::Volume;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::seq::SliceRandom;
struct ImageIcons {
    projectile_icon: Handle<Image>,
    shield_icon: Handle<Image>,
    pierce_icon: Handle<Image>,
    freeze_icon: Handle<Image>,
    fire_icon: Handle<Image>,
    snowball_icon: Handle<Image>,
}
impl FromWorld for ImageIcons {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        Self {
            projectile_icon: asset_server.load("candycane_shuriken.png"),
            shield_icon: asset_server.load("chestnut.png"),
            pierce_icon: asset_server.load("candycane.png"),
            freeze_icon: asset_server.load("freeze.png"),
            fire_icon: asset_server.load("fire_icon.png"),
            snowball_icon: asset_server.load("snowball_icon.png"),
        }
    }
}

struct LogoImage {
    logo: Handle<Image>,
}
impl FromWorld for LogoImage {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        Self {
            logo: asset_server.load("logo.png"),
        }
    }
}
pub struct MainMenuPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for MainMenuPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.insert_resource(Volume {
            music: 1.0,
            sfx: 1.0,
        });
        app.add_systems(Startup, (load_fonts, load_upgrades));

        app.add_systems(Update, setup_main_menu.run_if(in_state(self.state.clone())));
        app.add_systems(Update, setup_game_over.run_if(in_state(AppState::GameOver)));
        app.add_systems(
            Update,
            setup_pause_menu
                .run_if(in_state(GameState::Paused))
                .run_if(in_state(AppState::InGame)),
        );
        app.add_systems(Update, listen_pause.run_if(in_state(AppState::InGame)));
        app.add_systems(OnEnter(GameState::Upgrade), generate_available_upgrades);
        app.add_systems(
            Update,
            upgrade_screen
                .run_if(in_state(GameState::Upgrade))
                .after(generate_available_upgrades),
        );
        app.add_systems(Update, setup_settings.run_if(in_state(AppState::Settings)));
        app.add_systems(Update, credits_screen.run_if(in_state(AppState::Credits)));
        app.add_systems(Update, setup_tutorial.run_if(in_state(AppState::Tutorial)));
    }
}
fn load_upgrades(
    mut commands: Commands,
    mut context: EguiContexts,
    icons: Local<ImageIcons>,
    player_stats: Res<PlayerStats>,
) {
    let rendered_projectile_icon = context.add_image(icons.projectile_icon.clone());
    let rendered_shield_icon = context.add_image(icons.shield_icon.clone());
    let rendered_pierce_icon = context.add_image(icons.pierce_icon.clone());
    let rendered_freeze_icon = context.add_image(icons.freeze_icon.clone());
    let rendered_fire_icon = context.add_image(icons.fire_icon.clone());
    let rendered_snowball_icon = context.add_image(icons.snowball_icon.clone());
    commands.insert_resource(SelectedUpgradeIndices::default());
    commands.insert_resource(UpgradeCards {
        upgrades: [
            UpgradeCard {
                name: "Chestnut Shield".to_string(),
                icon: rendered_shield_icon,
                description: "Adds an orbiting chestnut shield that protects you from enemies"
                    .to_string(),
                available: true,
                taken: None,
                prereq_met: true,
                upgrade_id: 0,
            },
            UpgradeCard {
                name: "Projectile Rate of Fire".to_string(),
                icon: rendered_projectile_icon,
                description: "Increases your attack speed".to_string(),
                available: true,
                taken: None,
                prereq_met: true,
                upgrade_id: 1,
            },
            UpgradeCard {
                name: "Projectile Speed".to_string(),
                icon: rendered_projectile_icon,
                description: "Increases your projectile speed".to_string(),
                available: true,
                taken: None,
                prereq_met: true,
                upgrade_id: 2,
            },
            UpgradeCard {
                name: "Damage".to_string(),
                icon: rendered_projectile_icon,
                description: "Increases your projectile damage".to_string(),
                available: true,
                taken: None,
                prereq_met: true,
                upgrade_id: 3,
            },
            UpgradeCard {
                name: "Acceleration".to_string(),
                icon: rendered_projectile_icon,
                description: "Increases your movement speed".to_string(),
                available: true,
                taken: None,
                prereq_met: true,
                upgrade_id: 4,
            },
            UpgradeCard {
                name: "Shield Damage".to_string(),
                icon: rendered_shield_icon,
                description: "Increases damage dealt by your orbiting shields".to_string(),
                available: true,
                taken: None,
                prereq_met: if player_stats.num_shields > 0 {
                    true
                } else {
                    false
                },
                upgrade_id: 5,
            },
            UpgradeCard {
                name: "Shield Rotation Speed".to_string(),
                icon: rendered_shield_icon,
                description: "Makes your shields rotate faster".to_string(),
                available: true,
                taken: None,
                prereq_met: if player_stats.num_shields > 0 {
                    true
                } else {
                    false
                },
                upgrade_id: 6,
            },
            //UpgradeCard {
            //    name: "Projectile Bounces".to_string(),
            //    icon: rendered_projectile_icon,
            //    description: "Your projectiles bounce one more time".to_string(),
            //    available: true,
            //    taken: None,
            //},
            UpgradeCard {
                name: "Freeze Chance".to_string(),
                icon: rendered_freeze_icon,
                description: "Increases chance to freeze enemies".to_string(),
                available: true,
                taken: None,
                prereq_met: true,
                upgrade_id: 7,
            },
            UpgradeCard {
                name: "Freeze Duration".to_string(),
                icon: rendered_freeze_icon,
                description: "Increases how long enemies stay frozen".to_string(),
                available: true,
                taken: None,
                prereq_met: if player_stats.freeze_chance > 0 {
                    true
                } else {
                    false
                },
                upgrade_id: 8,
            },
            UpgradeCard {
                name: "Fire Chance".to_string(),
                icon: rendered_fire_icon,
                description: "Increases chance to burn enemies".to_string(),
                available: true,
                taken: None,
                prereq_met: true,
                upgrade_id: 9,
            },
            UpgradeCard {
                name: "Fire Duration".to_string(),
                icon: rendered_fire_icon,
                description: "Increases how long enemies stay burning".to_string(),
                available: true,
                taken: None,
                prereq_met: if player_stats.fire_chance > 0 {
                    true
                } else {
                    false
                },
                upgrade_id: 10,
            },
            UpgradeCard {
                name: "Fire Damage".to_string(),
                icon: rendered_fire_icon,
                description: "Increases damage over time from burning".to_string(),
                available: true,
                taken: None,
                prereq_met: if player_stats.fire_chance > 0 {
                    true
                } else {
                    false
                },
                upgrade_id: 11,
            },
            UpgradeCard {
                name: "Flash Freeze".to_string(),
                icon: rendered_freeze_icon,
                description: "Freezing burning enemies deals percent damage".to_string(),
                available: true,
                taken: Some(false),
                prereq_met: if player_stats.freeze_chance > 0 && player_stats.fire_chance > 0 {
                    true
                } else {
                    false
                },
                upgrade_id: 12,
            },
            UpgradeCard {
                name: "Flash Freeze Damage".to_string(),
                icon: rendered_freeze_icon,
                description: "Increases percent damage from Flash Freeze".to_string(),
                available: true,
                taken: None,
                prereq_met: if player_stats.flash_freeze {
                    true
                } else {
                    false
                },
                upgrade_id: 13,
            },
            UpgradeCard {
                name: "Freezer Burn".to_string(),
                icon: rendered_freeze_icon,
                description: "Burning frozen enemies makes them vulnerable".to_string(),
                available: true,
                taken: Some(false),
                prereq_met: if player_stats.freeze_chance > 0 && player_stats.fire_chance > 0 {
                    true
                } else {
                    false
                },
                upgrade_id: 14,
            },
            UpgradeCard {
                name: "Freezer Burn Duration".to_string(),
                icon: rendered_freeze_icon,
                description: "Increases vulnerability duration from Freezer Burn".to_string(),
                available: true,
                taken: None,
                prereq_met: if player_stats.freezer_burn {
                    true
                } else {
                    false
                },
                upgrade_id: 15,
            },
            UpgradeCard {
                name: "Freezer Burn Multiplier".to_string(),
                icon: rendered_freeze_icon,
                description: "Increases damage multiplier from Freezer Burn".to_string(),
                available: true,
                taken: None,
                prereq_met: if player_stats.freezer_burn {
                    true
                } else {
                    false
                },
                upgrade_id: 16,
            },
            UpgradeCard {
                name: "Pierce".to_string(),
                icon: rendered_pierce_icon,
                description: "Your projectiles pierce through one more enemy".to_string(),
                available: true,
                taken: None,
                prereq_met: true,
                upgrade_id: 17,
            },
            UpgradeCard {
                name: "Shields apply effects".to_string(),
                icon: rendered_shield_icon,
                description: "Your shields apply effects (fire and freeze) to enemies".to_string(),
                available: true,
                taken: Some(false),
                prereq_met: if player_stats.num_shields > 0
                    && (player_stats.freeze_chance > 0 || player_stats.fire_chance > 0)
                {
                    true
                } else {
                    false
                },
                upgrade_id: 18,
            },
            UpgradeCard {
                name: "Sugar Rush Damage Multiplier".to_string(),
                icon: rendered_snowball_icon,
                description: "Increases damage of your snowball during sugar rushes (go full speed for max damage)!".to_string(),
                available: true,
                taken: None,
                prereq_met: true,
                upgrade_id: 19,
            },
            UpgradeCard {
                name: "Knockback Strength".to_string(),
                icon: rendered_snowball_icon,
                description: "Increases knockback strength".to_string(),
                available: true,
                taken: None,
                prereq_met: true,
                upgrade_id: 20,
            },
        ],
    });
}

fn load_fonts(mut context: EguiContexts) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "slkscr".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/slkscr.ttf")),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "slkscr".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "slkscr".to_owned());
    context.ctx_mut().set_fonts(fonts);
}
fn setup_tutorial(mut contexts: EguiContexts, mut app_state: ResMut<NextState<AppState>>) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, 250)))
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label(egui::RichText::new("How to Play").size(48.0).strong());
                ui.add_space(30.0);

                ui.label(
                    egui::RichText::new("• Enemies come in waves - defeat them to progress!")
                        .size(24.0),
                );
                ui.add_space(20.0);

                ui.label(
                    egui::RichText::new("• Your snowball grows as you defeat enemies").size(24.0),
                );
                ui.label(
                    egui::RichText::new("  Don't let it get too small or it's game over!")
                        .size(24.0),
                );
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("The bigger your snowball, the faster you build momentum!")
                        .size(24.0),
                );
                ui.add_space(20.0);

                ui.label(
                    egui::RichText::new("• Left click to fire candy cane shurikens").size(24.0),
                );
                ui.add_space(20.0);

                ui.label(
                    egui::RichText::new(
                        "• Collect candy canes on the ground to activate Sugar Rush",
                    )
                    .size(24.0),
                );
                ui.label(
                    egui::RichText::new("  Become invincible and plow through enemies!").size(24.0),
                );
                ui.add_space(20.0);

                ui.label(
                    egui::RichText::new("• Fill your XP bar to earn powerful upgrades").size(24.0),
                );
                ui.add_space(40.0);

                if ui
                    .add_sized(
                        [220.0, 60.0],
                        egui::Button::new(
                            egui::RichText::new("Got it!")
                                .size(32.0)
                                .color(egui::Color32::WHITE),
                        ),
                    )
                    .clicked()
                {
                    app_state.set(AppState::MainMenu);
                }
            });
        });
}
fn setup_main_menu(
    mut contexts: EguiContexts,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    logo_image: Local<LogoImage>,
) {
    let logo_image = contexts.add_image(logo_image.logo.clone());

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, 150)))
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.add(egui::Image::new(egui::load::SizedTexture::new(
                    logo_image,
                    [400.0, 200.0],
                )));
                ui.add_space(30.0);

                // Stylized play button
                let play_button = egui::Button::new(
                    egui::RichText::new("Play")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(240, 240, 255)),
                );
                if ui
                    .add_sized([220.0, 60.0], play_button)
                    .on_hover_text("Start your adventure!")
                    .clicked()
                {
                    app_state.set(AppState::InGame);
                    game_state.set(GameState::Playing);
                }

                ui.add_space(30.0);
                let tutorial_button = egui::Button::new(
                    egui::RichText::new("Tutorial")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(240, 240, 255)),
                );
                if ui
                    .add_sized([220.0, 60.0], tutorial_button)
                    .on_hover_text("Learn the ropes!")
                    .clicked()
                {
                    app_state.set(AppState::Tutorial);
                }

                ui.add_space(30.0);
                let settings_button = egui::Button::new(
                    egui::RichText::new("Settings")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(240, 240, 255)),
                );
                if ui
                    .add_sized([220.0, 60.0], settings_button)
                    .on_hover_text("Settings")
                    .clicked()
                {
                    app_state.set(AppState::Settings);
                }

                ui.add_space(30.0);
                let credits_button = egui::Button::new(
                    egui::RichText::new("Credits")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(240, 240, 255)),
                );
                if ui
                    .add_sized([220.0, 60.0], credits_button)
                    .on_hover_text("View credits")
                    .clicked()
                {
                    app_state.set(AppState::Credits);
                }

                ui.add_space(30.0);

                // Stylized quit button
                let quit_button = egui::Button::new(
                    egui::RichText::new("Quit")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(240, 240, 255)),
                );
                if ui
                    .add_sized([220.0, 60.0], quit_button)
                    .on_hover_text("Exit the game")
                    .clicked()
                {
                    std::process::exit(0);
                }
            });
        });
}
fn setup_settings(
    mut contexts: EguiContexts,
    mut app_state: ResMut<NextState<AppState>>,
    mut volume: ResMut<Volume>,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, 250)))
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                // Settings title
                let title = egui::RichText::new("Settings")
                    .size(48.0)
                    .color(egui::Color32::from_rgb(100, 150, 200))
                    .strong();
                ui.add(egui::Label::new(title));

                ui.add_space(40.0);

                // Music Volume Slider
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Music Volume")
                            .size(24.0)
                            .color(egui::Color32::WHITE),
                    );
                    ui.add(egui::Slider::new(&mut volume.music, 0.0..=1.0).text(""));
                });

                ui.add_space(20.0);

                // SFX Volume Slider
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("SFX Volume")
                            .size(24.0)
                            .color(egui::Color32::WHITE),
                    );
                    ui.add(egui::Slider::new(&mut volume.sfx, 0.0..=1.0).text(""));
                });

                ui.add_space(60.0);

                // Back button with enhanced styling
                if ui
                    .add_sized(
                        [220.0, 60.0],
                        egui::Button::new(
                            egui::RichText::new("Back")
                                .size(32.0)
                                .color(egui::Color32::from_rgb(240, 240, 255))
                                .strong(),
                        )
                        .fill(egui::Color32::from_rgb(80, 80, 160))
                        .stroke(egui::Stroke::new(2.0, egui::Color32::WHITE)),
                    )
                    .on_hover_text("Return to main menu")
                    .clicked()
                {
                    app_state.set(AppState::MainMenu);
                }
            });
        });
}
fn setup_game_over(
    mut contexts: EguiContexts,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(240, 240, 255, 0)))
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);

                // Game Over text with shadow effect
                let title = egui::RichText::new("Game Over")
                    .size(48.0)
                    .color(egui::Color32::from_rgb(150, 50, 50))
                    .strong();

                ui.add(egui::Label::new(title));
                ui.add_space(60.0);

                // Retry button
                let retry_button = egui::Button::new(
                    egui::RichText::new("Retry")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(240, 240, 255)),
                );
                if ui
                    .add_sized([220.0, 60.0], retry_button)
                    .on_hover_text("Try again!")
                    .clicked()
                {
                    app_state.set(AppState::InGame);
                    game_state.set(GameState::Playing);
                }

                ui.add_space(30.0);

                // Main Menu button
                let menu_button = egui::Button::new(
                    egui::RichText::new("Main Menu")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(240, 240, 255)),
                );
                if ui
                    .add_sized([220.0, 60.0], menu_button)
                    .on_hover_text("Return to main menu")
                    .clicked()
                {
                    app_state.set(AppState::MainMenu)
                }

                ui.add_space(30.0);

                // Quit button
                let quit_button = egui::Button::new(
                    egui::RichText::new("Quit")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(240, 240, 255)),
                );
                if ui
                    .add_sized([220.0, 60.0], quit_button)
                    .on_hover_text("Exit the game")
                    .clicked()
                {
                    std::process::exit(0);
                }
            });
        });
}
fn setup_pause_menu(
    mut contexts: EguiContexts,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, 250)))
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);

                ui.heading(
                    egui::RichText::new("PAUSED")
                        .size(64.0)
                        .color(egui::Color32::from_rgb(240, 240, 255))
                        .strong(),
                );

                ui.add_space(50.0);

                // Resume button
                let resume_button = egui::Button::new(
                    egui::RichText::new("Resume")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(240, 240, 255)),
                );
                if ui
                    .add_sized([220.0, 60.0], resume_button)
                    .on_hover_text("Return to game")
                    .clicked()
                {
                    game_state.set(GameState::Playing);
                }

                ui.add_space(30.0);

                // Main Menu button
                let menu_button = egui::Button::new(
                    egui::RichText::new("Main Menu")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(240, 240, 255)),
                );
                if ui
                    .add_sized([220.0, 60.0], menu_button)
                    .on_hover_text("Return to main menu")
                    .clicked()
                {
                    app_state.set(AppState::MainMenu);
                }

                ui.add_space(30.0);

                // Quit button
                let quit_button = egui::Button::new(
                    egui::RichText::new("Quit")
                        .size(32.0)
                        .color(egui::Color32::from_rgb(240, 240, 255)),
                );
                if ui
                    .add_sized([220.0, 60.0], quit_button)
                    .on_hover_text("Exit the game")
                    .clicked()
                {
                    std::process::exit(0);
                }
            });
        });
}
fn listen_pause(
    key_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if key_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Paused);
    }
}

struct UpgradeCard {
    name: String,
    icon: egui::TextureId, // Path to icon asset
    description: String,
    available: bool,
    taken: Option<bool>,
    prereq_met: bool,
    upgrade_id: u32,
}
// Start of Selection
#[derive(Resource)]
struct UpgradeCards {
    upgrades: [UpgradeCard; 21],
}

#[derive(Resource, Default)]
struct SelectedUpgradeIndices {
    indices: Vec<usize>,
}

fn generate_available_upgrades(
    mut selected_indices_res: ResMut<SelectedUpgradeIndices>,
    mut upgrades: ResMut<UpgradeCards>,
    player_stats: Res<PlayerStats>,
) {
    // 1) Re-check each upgrade’s “prereq_met” based on current player_stats
    for upgrade in &mut upgrades.upgrades {
        let meets_requirements = match upgrade.upgrade_id {
            5 => player_stats.num_shields > 0,
            6 => player_stats.num_shields > 0,
            8 => player_stats.freeze_chance > 0,
            10 | 11 => player_stats.fire_chance > 0,
            12 => player_stats.freeze_chance > 0 && player_stats.fire_chance > 0,
            13 => player_stats.flash_freeze,
            14 => player_stats.freeze_chance > 0 && player_stats.fire_chance > 0,
            15 | 16 => player_stats.freezer_burn,
            18 => {
                player_stats.num_shields > 0
                    && (player_stats.fire_chance > 0 || player_stats.freeze_chance > 0)
            }
            // etc. for any others that have special requirements
            _ => true,
        };
        upgrade.prereq_met = meets_requirements;
    }

    // 2) Clear out any old selection first
    selected_indices_res.indices.clear();

    // 3) Collect the indices of available (and now up-to-date) upgrades
    let mut rng = rand::thread_rng();
    let available_upgrade_indices: Vec<usize> = upgrades
        .upgrades
        .iter()
        .enumerate()
        .filter(|(_, u)| u.available && u.prereq_met)
        .map(|(i, _)| i)
        .collect();

    // 4) If at least 3 are available, choose 3 at random. Otherwise, pick all.
    let new_selection = if available_upgrade_indices.len() >= 3 {
        available_upgrade_indices
            .choose_multiple(&mut rng, 3)
            .cloned()
            .collect::<Vec<usize>>()
    } else {
        available_upgrade_indices
    };

    // 5) Store them in the resource for later
    selected_indices_res.indices = new_selection;
}

fn upgrade_screen(
    mut egui_ctx: EguiContexts,
    mut app_state: ResMut<NextState<GameState>>,
    mut player_stats: ResMut<PlayerStats>,
    mut upgrades: ResMut<UpgradeCards>,
    selected_indices_res: Res<SelectedUpgradeIndices>,
) {
    let ctx = egui_ctx.ctx_mut();
    // Get the indices chosen by generate_available_upgrades
    let selected_indices = &selected_indices_res.indices;

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, 250)))
        .show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Choose an Upgrade").size(48.0).strong());
                ui.add_space(15.0);

                ui.horizontal_centered(|ui| {
                    // (Optional) Position the 3 upgrade cards in the center
                    ui.add_space(ui.available_width() / 2.0 - (300.0 * 3 as f32) / 2.0);

                    for &index in selected_indices {
                        let upgrade = &mut upgrades.upgrades[index];
                        ui.group(|ui| {
                            ui.set_min_size(egui::vec2(300.0, 400.0));
                            ui.set_max_size(egui::vec2(300.0, 400.0));
                            ui.vertical_centered(|ui| {
                                ui.add_space(10.0);
                                ui.label(egui::RichText::new(&upgrade.name).size(28.0).strong());
                                ui.add_space(10.0);
                                ui.add(egui::Image::new(egui::load::SizedTexture::new(
                                    upgrade.icon,
                                    [100.0, 100.0],
                                )));
                                ui.add_space(10.0);
                                ui.label(egui::RichText::new(&upgrade.description).size(16.0));
                                ui.add_space(15.0);

                                if ui
                                    .button(
                                        egui::RichText::new("Select")
                                            .size(20.0)
                                            .color(egui::Color32::WHITE),
                                    )
                                    .clicked()
                                {
                                    // Apply the upgrade effects based on upgrade_id
                                    match upgrade.upgrade_id {
                                        0 => player_stats.num_shields += 1,
                                        1 => player_stats.rate_of_fire *= 0.5,
                                        2 => player_stats.projectile_speed *= 2.,
                                        3 => player_stats.damage *= 1.25,
                                        4 => player_stats.acceleration_rate *= 1.5,
                                        5 => player_stats.shield_damage *= 1.25,
                                        6 => player_stats.shield_rotation_speed += 0.02,
                                        7 => player_stats.freeze_chance += 20,
                                        8 => player_stats.freeze_duration += 1.,
                                        9 => player_stats.fire_chance += 20,
                                        10 => player_stats.fire_duration += 2.0,
                                        11 => player_stats.fire_dps *= 1.5,
                                        12 => player_stats.flash_freeze = true,
                                        13 => player_stats.flash_freeze_percent_damage += 0.1,
                                        14 => player_stats.freezer_burn = true,
                                        15 => player_stats.freezer_burn_duration += 1.,
                                        16 => player_stats.freezer_burn_multiplier *= 1.5,
                                        17 => player_stats.projectile_piercing += 1,
                                        18 => player_stats.shield_apply_effects = true,
                                        19 => player_stats.snowball_damage_multiplier += 0.5,
                                        20 => player_stats.knockback_strength += 1.0,

                                        _ => {}
                                    }

                                    // Mark the upgrade as taken and unavailable
                                    if let Some(taken) = upgrade.taken.as_mut() {
                                        *taken = true;
                                        upgrade.available = false;
                                    }

                                    // Exit upgrade screen
                                    app_state.set(GameState::Playing);
                                }
                            });
                        });
                    }
                });
            });
        });
}

fn credits_screen(mut egui_ctx: EguiContexts, mut app_state: ResMut<NextState<AppState>>) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, 250)))
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Credits").size(48.0).strong());
                ui.add_space(15.0);
                ui.separator();
                ui.add_space(15.0);

                // Developers Section
                ui.label(egui::RichText::new("Developers").size(32.0).strong());
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Safe Gergis - Developer").size(24.0));
                ui.label(egui::RichText::new("Alexander Blocker - Developer").size(24.0));
                ui.add_space(20.0);

                // Artists Section
                ui.label(egui::RichText::new("Artists").size(32.0).strong());
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Brandon Martin Del Campo - 2D Artist").size(24.0));
                ui.label(egui::RichText::new("Ryan Agundez - Music").size(24.0));
                ui.add_space(30.0);

                // Back Button
                if ui
                    .add_sized(
                        [220.0, 60.0],
                        egui::Button::new(
                            egui::RichText::new("Back")
                                .size(20.0)
                                .color(egui::Color32::WHITE),
                        )
                        .fill(egui::Color32::from_rgb(80, 80, 160))
                        .stroke(egui::Stroke::new(2.0, egui::Color32::WHITE)),
                    )
                    .on_hover_text("Return to main menu")
                    .clicked()
                {
                    app_state.set(AppState::MainMenu);
                }
            });
        });
}
