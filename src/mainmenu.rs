use crate::AppState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
pub struct MainMenuPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for MainMenuPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Startup, load_fonts);
        app.add_systems(Update, setup_main_menu.run_if(in_state(self.state.clone())));
        app.add_systems(Update, setup_game_over.run_if(in_state(AppState::GameOver)));
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
fn setup_main_menu(mut contexts: EguiContexts, mut app_state: ResMut<NextState<AppState>>) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(240, 240, 255, 0)))
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);

                // Title with shadow effect
                let title = egui::RichText::new("Snow Elf Adventure")
                    .size(48.0)
                    .color(egui::Color32::from_rgb(50, 50, 150))
                    .strong();

                ui.add(egui::Label::new(title));
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
                    app_state.set(AppState::InGame)
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
fn setup_game_over(mut contexts: EguiContexts, mut app_state: ResMut<NextState<AppState>>) {
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
                    app_state.set(AppState::InGame)
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
