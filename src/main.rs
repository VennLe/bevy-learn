use bevy::prelude::*;

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
                ..default()
            }),
            ..default()
        }))
        .init_state::<game_state::GameState>()
        .add_plugins((
            core::CorePlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            ui::UiPlugin,
            audio::AudioPlugin,
            camera::CameraPlugin,
        ))
        .run();
}