use bevy::prelude::*;

use crate::game_state::{AssetsLoading, InitGamePlugin};

mod game_state;
mod core;
mod player;
mod enemy;
mod ui;
mod audio;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 0.19 Demo".into(),
                resolution: (1280, 720).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resizable: false,
                decorations: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<game_state::GameState>()
        .insert_resource(AssetsLoading(Vec::new()))
        .add_plugins((
            InitGamePlugin,
            core::CorePlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            audio::AudioPlugin,
            camera::CameraPlugin,
        ))
        .run();
}