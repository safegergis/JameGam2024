use crate::camera::InGameCamera;
use bevy::prelude::*;
use rand::Rng;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
        app.add_systems(Update, move_background);
    }
}

#[derive(Component)]
pub struct Background;

fn move_background(
    q_camera: Query<(&Camera, &GlobalTransform), With<InGameCamera>>,
    mut q: Query<(&mut Transform, &Background)>,
) {
    let (_camera, camera_transform) = q_camera.single();
    //println!("PlayerPositon coords: {}/{}", player.translation().x, player.translation().y);
    let mut i = 0;
    for (mut tf, background) in q.iter_mut() {
        let w: f32 = 50.0;
        let h: f32 = 50.0;
        let rectangle = Rectangle::new(w, h);
        let snap_x = (camera_transform.translation().x / w).round() * w;
        let snap_y = (camera_transform.translation().y / h).round() * h;
        
        let mut pos = Vec3::new(snap_x, snap_y, 0.0);

        // Can't think of a different way to do this
        if (i == 1 || i == 2 || i == 3) {
            pos.x += w;
        }

        if (i == 5 || i == 6 || i == 7) {
            pos.x -= w;
        }

        if (i == 7 || i == 8 || i == 1) {
            pos.y += h;
        }

        if (i == 3 || i == 4 || i == 5) {
            pos.y -= h;
        }
        
        tf.translation = pos;

        i = i + 1;
    }
}

fn setup(
    q_camera: Query<(&Camera, &GlobalTransform), With<InGameCamera>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Background4848.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 14, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let (_camera, camera_transform) = q_camera.single();
    // Use only the subset of sprites in the sheet that make up the run animation

    let w: f32 = 50.0;
    let h: f32 = 50.0;
    let rectangle = Rectangle::new(w, h);
    let snap_x = (camera_transform.translation().x / w).round() * w;
    let snap_y = (camera_transform.translation().y / h).round() * h;

    let mut i = 0;
    while i < 9 {
        let mut pos = Vec3::new(snap_x, snap_y, 0.0);

        // Can't think of a different way to do this
        if (i == 1 || i == 2 || i == 3) {
            pos.x += w;
        }

        if (i == 5 || i == 6 || i == 7) {
            pos.x -= w;
        }

        if (i == 7 || i == 8 || i == 1) {
            pos.y += h;
        }

        if (i == 3 || i == 4 || i == 5) {
            pos.y -= h;
        }

        println!("PlayerPositon coords: {}/{}", pos, pos);
        commands.spawn((
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: i,
                },
            ),
            Transform::from_translation(pos),
            Background,
        ));

        //let mut o = 0;
        // while o < 70 {
        //     let mut boundary_pt = rectangle.sample_interior(&mut rand::thread_rng()).extend(-100.0);
        //     boundary_pt.x += snap_x;
        //     boundary_pt.y += snap_y;
        //     let num_offset = rand::thread_rng().gen_range(1..13);
        //     commands.spawn((
        //         Sprite::from_atlas_image(
        //             texture.clone(),
        //             TextureAtlas {
        //                 layout: texture_atlas_layout.clone(),
        //                 index: num_offset,
        //             },
        //         ),
        //         Transform::from_translation(boundary_pt),

        //     ));

        //     o = o + 1;
        // }

        i = i + 1;
    }
}
