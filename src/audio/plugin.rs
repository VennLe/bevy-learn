use bevy::prelude::*;
use crate::game_state::GameState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), play_background_music);
    }
}

fn play_background_music(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // 0.19 中 AudioPlayer 接收 Handle<AudioSource>
    // 此处仅为示意，实际需要加载音频文件

    let audio_source = asset_server.load("audio/****.mp3");

    cmd.spawn((
        AudioPlayer::new(audio_source),
        PlaybackSettings::LOOP,
    ));
}