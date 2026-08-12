<!-- 支持格式：glTF、BSN（Bevy Scene Notation） -->

// src/levels/plugin.rs
use bevy::prelude::*;

pub fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 方式一：加载 glTF 场景（包含网格、材质、灯光、相机等）
    commands.spawn((
        SceneRoot(asset_server.load("levels/dungeon.glb#Scene0")),
        Transform::default(),
    ));
    
    // 方式二：使用 BSN 定义场景（Bevy 0.19 新特性）
    commands.spawn_scene(bsn! {
        Player { hp: 100 }
        Transform::from_xyz(0.0, 1.0, 0.0)
        Children [
            bsn! { Weapon { damage: 50 } }
        ]
    });
}

// 监听场景加载完成事件
fn on_scene_loaded(
    mut events: EventReader<SceneInstanceReady>,
    scene_query: Query<&SceneRoot>,
) {
    for event in events.read() {
        if let Ok(scene_root) = scene_query.get(event.parent) {
            println!("Scene loaded: {:?}", scene_root.0.id());
        }
    }
}