//! Shows how to create graphics that snap to the pixel grid by rendering to a texture in 2D

use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
};

use bevy::asset::AssetMetaCheck;
#[derive(Resource)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
    pub base_width: u32,
    pub base_height: u32,
}

#[derive(Resource)]
struct CanvasHandle(Handle<Image>);

/// In-game resolution width.
pub const RES_WIDTH: u32 = 480;

/// In-game resolution height.
pub const RES_HEIGHT: u32 = 270;

/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

/// Render layers for high-resolution rendering.
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Resolution {
            width: RES_WIDTH,
            height: RES_HEIGHT,
            base_width: RES_WIDTH,
            base_height: RES_HEIGHT,
        });
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        );
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, (rotate, (update_canvas_size, fit_canvas).chain()));
    }
}

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
//         .add_systems(Startup, (setup_camera, setup_sprite, setup_mesh))
//         .add_systems(Update, (rotate, fit_canvas))
//         .run();
// }

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.

#[derive(Component)]
struct Canvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
pub struct InGameCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
pub struct OuterCamera;

#[derive(Component)]
pub struct Rotate {
    pub speed: f32,
}

fn update_canvas_size(
    mut images: ResMut<Assets<Image>>,
    resolution: Res<Resolution>,
    canvas_handle: Res<CanvasHandle>,
) {
    if let Some(canvas) = images.get_mut(&canvas_handle.0) {
        // New canvas size
        let new_width = resolution.width;
        let new_height = resolution.height;

        // Update the Extent3d size
        let new_size = Extent3d {
            width: new_width,
            height: new_height,
            ..canvas.texture_descriptor.size
        };

        // Update the texture descriptor and resize the buffer
        canvas.texture_descriptor.size = new_size;
        canvas.resize(new_size);

        // // The canvas size has now been updated
        // info!("Canvas size updated to {}x{}", new_width, new_height);
    }
}

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    // this Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    commands.insert_resource(CanvasHandle(image_handle.clone()));

    // this camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    commands.spawn((
        Camera2d,
        Camera {
            // render before the "main pass" camera
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.92, 0.92, 0.92)),
            target: RenderTarget::Image(image_handle.clone()),
            ..default()
        },
        Msaa::Off,
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));

    // spawn the canvas
    commands.spawn((Sprite::from_image(image_handle), Canvas, HIGH_RES_LAYERS));

    // the "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2d, Msaa::Off, OuterCamera, HIGH_RES_LAYERS));
}

/// Rotates entities to demonstrate grid snapping.
fn rotate(time: Res<Time>, mut rotates: Query<(&mut Transform, &Rotate), With<Rotate>>) {
    for (mut transform, rotate) in &mut rotates {
        let dt = time.delta_secs() * rotate.speed;
        transform.rotate_z(dt);
    }
}

/// Scales camera projection to fit the window (integer multiples only).
fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projection: Single<&mut OrthographicProjection, With<OuterCamera>>,
    resolution: Res<Resolution>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / resolution.width as f32;
        let v_scale = event.height / resolution.height as f32;
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
