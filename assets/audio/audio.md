<!-- 支持格式：OGG、WAV、MP3、FLAC -->

// src/audio/plugin.rs
use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_audio_assets)
           .add_systems(Update, play_footsteps);
    }
}

fn load_audio_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 加载背景音乐
    commands.insert_resource(BackgroundMusic {
        handle: asset_server.load("audio/bgm.ogg"),
    });
    
    // 加载音效
    commands.insert_resource(SoundEffects {
        footstep: asset_server.load("audio/footstep.wav"),
        sword_swing: asset_server.load("audio/sword_swing.mp3"),
        hit: asset_server.load("audio/hit.flac"),
    });
}

// 资源定义
#[derive(Resource)]
struct BackgroundMusic {
    handle: Handle<AudioSource>,
}

#[derive(Resource)]
struct SoundEffects {
    footstep: Handle<AudioSource>,
    sword_swing: Handle<AudioSource>,
    hit: Handle<AudioSource>,
}

// 播放音效示例
fn play_footsteps(
    mut commands: Commands,
    sounds: Res<SoundEffects>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyW) {
        commands.spawn((
            AudioPlayer::new(sounds.footstep.clone()),
            PlaybackSettings::DESPAWN,  // 播完后自动销毁
        ));
    }
}