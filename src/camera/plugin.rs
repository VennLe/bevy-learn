use crate::game_state::GameState;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // 第一人称相机已经作为 Player 的子节点生成
        // 这里可以留空，或用于额外的"观察者相机"等
        app.add_systems(OnEnter(GameState::Playing), spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-15.0, 20.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        // 用 order 让它在主相机之后渲染，不抢主画面
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,  // 不清除主相机的画面
            ..default()
        },
    ));
}
