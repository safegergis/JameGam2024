use crate::player::PlayerXp;
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
        app.add_systems(FixedUpdate, update_xp_bar);
    }
}
#[derive(Component)]
struct XpBar;
fn update_xp_bar(player_q: Query<&PlayerXp>, mut xp_bar_q: Query<&mut Node, With<XpBar>>) {
    let player_xp = player_q.single();
    let mut xp_bar = xp_bar_q.single_mut();
    xp_bar.width = Val::Px(player_xp.xp as f32 * 600.0 / 1000.0);
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("XP"),
                        TextFont {
                            font: asset_server.load("fonts/slkscr.ttf"),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                    parent
                        .spawn((
                            Node {
                                margin: UiRect::bottom(Val::Px(20.0)),
                                width: Val::Px(600.0),
                                height: Val::Px(30.0),
                                border: UiRect::all(Val::Px(3.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.5)),
                            BorderColor(Color::BLACK),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ImageNode {
                                    image: asset_server.load("xp-bar.png"),
                                    image_mode: NodeImageMode::Tiled {
                                        tile_x: true,
                                        tile_y: false,
                                        stretch_value: 1.0,
                                    },
                                    ..default()
                                },
                                Node {
                                    width: Val::Px(0.0),
                                    height: Val::Px(24.0),
                                    ..default()
                                },
                                XpBar,
                            ));
                        });
                });
        });
}
