use bevy::{prelude::*};
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}};

use crate::{game_state::InitGamePlugin, ui::MenuPlugin};

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
        .add_plugins(RapierPhysicsPlugin::<()>::default())  // 添加物理引擎
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((
            InitGamePlugin,
            MenuPlugin,
            core::CorePlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            audio::AudioPlugin,
            // camera::CameraPlugin,
        ))
        // 启动时直接进入 Playing（简化，跳过 Loading）
        .add_systems(Startup, |mut next: ResMut<NextState<game_state::GameState>>| {
            next.set(game_state::GameState::Menu);
        })
        .run();
}