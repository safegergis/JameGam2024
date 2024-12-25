use crate::{camera::InGameCamera, utils::YSort};
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashSet;

pub struct BackgroundPlugin;

const BACKGROUND_WIDTH: f32 = 300.0;
const BACKGROUND_HEIGHT: f32 = 300.0;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
        app.add_systems(FixedUpdate, move_background);
    }
}

#[derive(Component)]
pub struct Background;

fn move_background(
    q_camera: Query<(&Camera, &GlobalTransform), With<InGameCamera>>,
    mut q: Query<(&mut Transform, &Background)>, // Ensure mutability here
) {
    let (_camera, camera_transform) = q_camera.single();

    let w: f32 = BACKGROUND_WIDTH;
    let h: f32 = BACKGROUND_HEIGHT;

    // Snap camera position to grid
    let snap_x = (camera_transform.translation().x / w).round() as i32;
    let snap_y = (camera_transform.translation().y / h).round() as i32;

    // Generate valid grid positions around the camera
    let mut valid_positions = HashSet::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            valid_positions.insert((snap_x + dx, snap_y + dy));
        }
    }

    let mut occupied_positions = HashSet::new();
    let mut entities_to_reassign = Vec::new();

    // Identify occupied positions and entities to reassign
    for (transform, _background) in q.iter_mut() {
        // Use mut Transform here
        let grid_x = (transform.translation.x / w).round() as i32;
        let grid_y = (transform.translation.y / h).round() as i32;

        if valid_positions.contains(&(grid_x, grid_y))
            && !occupied_positions.contains(&(grid_x, grid_y))
        {
            // Entity is in a valid position and not overlapping
            occupied_positions.insert((grid_x, grid_y));
        } else {
            // Entity needs reassignment
            entities_to_reassign.push(transform);
        }
    }

    // Reassign entities to available valid positions
    let mut available_positions = valid_positions
        .difference(&occupied_positions)
        .copied()
        .collect::<Vec<_>>();

    for mut transform in entities_to_reassign {
        if let Some(new_pos) = available_positions.pop() {
            //println!("PlayerPositon coords: {}/{}", new_pos.0, new_pos.1);
            transform.translation = Vec3::new(
                new_pos.0 as f32 * w,
                new_pos.1 as f32 * h,
                transform.translation.z,
            );
            occupied_positions.insert(new_pos);
        }
    }
}

fn setup(
    q_camera: Query<(&Camera, &GlobalTransform), With<InGameCamera>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let (_camera, camera_transform) = q_camera.single();

    let w: f32 = BACKGROUND_WIDTH;
    let h: f32 = BACKGROUND_HEIGHT;
    let rectangle = Rectangle::new(w, h);
    let snap_x = (camera_transform.translation().x / w).round() as i32;
    let snap_y = (camera_transform.translation().y / h).round() as i32;

    let texture = asset_server.load("background4848.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 14, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut i: i32 = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            let pos = Vec3::new((snap_x + dx) as f32 * w, (snap_y + dy) as f32 * h, 0.0);

            let num_offset = rand::thread_rng().gen_range(1..13);
            //println!("PlayerPositon coords: {}/{}", pos.x, pos.y);
            let background_holder = commands
                .spawn((
                    Transform::from_translation(pos),
                    GlobalTransform::default(),
                    Background,
                    Sprite::from_atlas_image(
                        texture.clone(),
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: num_offset,
                        },
                    ),
                    YSort { z: -100.0 },
                ))
                .id();

            let mut o = 0;
            while o < 5 {
                let boundary_pt = rectangle
                    .sample_interior(&mut rand::thread_rng())
                    .extend(-100.0);
                //boundary_pt.x += pos.x;
                //boundary_pt.y += pos.y;
                let num_offset = rand::thread_rng().gen_range(1..13);
                let background_prop = commands
                    .spawn((
                        Sprite::from_atlas_image(
                            texture.clone(),
                            TextureAtlas {
                                layout: texture_atlas_layout.clone(),
                                index: num_offset,
                            },
                        ),
                        Transform::from_translation(boundary_pt),
                    ))
                    .id();

                commands
                    .entity(background_holder)
                    .add_child(background_prop);

                o = o + 1;
            }

            i += 1;
        }
    }
}
