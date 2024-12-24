use crate::AppState;
use crate::GameState;
use crate::Volume;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
struct ImageIcons {
    projectile_icon: Handle<Image>,
    shield_icon: Handle<Image>,
    pierce_icon: Handle<Image>,
}
impl FromWorld for ImageIcons {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        Self {
            projectile_icon: asset_server.load("candycane_shuriken.png"),
            shield_icon: asset_server.load("chestnut.png"),
            pierce_icon: asset_server.load("candycane.png"),
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
        app.add_systems(Startup, load_fonts);
        app.add_systems(Update, setup_main_menu.run_if(in_state(self.state.clone())));
        app.add_systems(Update, setup_game_over.run_if(in_state(AppState::GameOver)));
        app.add_systems(Update, setup_pause_menu.run_if(in_state(GameState::Paused)));
        app.add_systems(Update, listen_pause.run_if(in_state(AppState::InGame)));
        app.add_systems(Update, upgrade_screen.run_if(in_state(GameState::Upgrade)));
        app.add_systems(Update, setup_settings.run_if(in_state(AppState::Settings)));
        app.add_systems(Update, credits_screen.run_if(in_state(AppState::Credits)));
    }
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
                ui.add_space(100.0);
                ui.add(egui::Image::new(egui::load::SizedTexture::new(
                    logo_image,
                    [400.0, 200.0],
                )));
                ui.add_space(60.0);

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
}

fn upgrade_screen(
    mut egui_ctx: EguiContexts,
    mut app_state: ResMut<NextState<GameState>>,
    icons: Local<ImageIcons>,
) {
    let rendered_projectile_icon = egui_ctx.add_image(icons.projectile_icon.clone());
    let rendered_shield_icon = egui_ctx.add_image(icons.shield_icon.clone());
    let rendered_pierce_icon = egui_ctx.add_image(icons.pierce_icon.clone());

    let ctx = egui_ctx.ctx_mut();

    // Define available upgrades
    let upgrades = vec![
        UpgradeCard {
            name: "Shield".to_string(),
            icon: rendered_shield_icon,
            description: "Adds an orbiting shield that protects you from enemies".to_string(),
        },
        UpgradeCard {
            name: "Speed Up".to_string(),
            icon: rendered_projectile_icon,
            description: "Increases your movement and attack speed".to_string(),
        },
        UpgradeCard {
            name: "Pierce".to_string(),
            icon: rendered_pierce_icon,
            description: "Your projectiles pierce through multiple enemies".to_string(),
        },
    ];

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, 250)))
        .show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Choose an Upgrade").size(48.0).strong());
                ui.add_space(15.0);
                ui.horizontal_centered(|ui| {
                    ui.add_space(
                        ui.available_width() / 2.0 - (220.0 * upgrades.len() as f32) / 2.0,
                    );
                    for upgrade in upgrades.iter() {
                        ui.group(|ui| {
                            ui.set_max_size(egui::vec2(220.0, 320.0));
                            ui.vertical_centered(|ui| {
                                ui.add_space(10.0);
                                ui.label(egui::RichText::new(&upgrade.name).size(28.0).strong());
                                // Start of Selection
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
