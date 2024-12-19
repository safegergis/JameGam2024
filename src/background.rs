use bevy::prelude::*;
use rand::Rng;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Background4848.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 14, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation

    let circle = Circle::new(1000.0);
    let mut i = 0;
    while i < 300 {
        let boundary_pt = circle.sample_interior(&mut rand::thread_rng()).extend(-100.0);
        let num_offset = rand::thread_rng().gen_range(1..13);
        commands.spawn((
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: num_offset,
                },
            ),
            Transform::from_translation(boundary_pt),
            
        ));

        i = i + 1;
    }

}
