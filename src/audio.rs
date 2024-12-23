use crate::Volume;
use bevy::prelude::*;
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, play_music);
        app.add_systems(Update, update_music_volume);
        app.add_systems(Update, update_sfx_volume);
    }
}
#[derive(Component)]
struct Music;

fn play_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/music.ogg")),
        Music,
        PlaybackSettings::LOOP,
    ));
}
fn update_music_volume(q_music: Query<&AudioSink, With<Music>>, volume: Res<Volume>) {
    let music = q_music.get_single();
    if let Ok(music) = music {
        music.set_volume(volume.music);
    }
}
fn update_sfx_volume(q_sfx: Query<&AudioSink, Without<Music>>, volume: Res<Volume>) {
    for sink in q_sfx.iter() {
        sink.set_volume(volume.sfx);
    }
}
