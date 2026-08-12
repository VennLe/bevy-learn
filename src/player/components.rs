use bevy::prelude::*;

/// 第一人称玩家组件
#[derive(Component, Debug)]
#[require(Transform, crate::core::DespawnOnGameOver,)]
pub struct Player {
    pub speed: f32,
    pub sensitivity: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub velocity: Vec3,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hp: 100.0,
            max_hp: 100.0,
            speed: 5.0,
            sensitivity: 0.002,  
            velocity: Vec3::ZERO,
        }
    }
}

/// 标记第一人称相机
#[derive(Component, Default, Debug)]
pub struct PlayerCamera;

/// 玩家视角状态（Yaw 偏航角 / Pitch 俯仰角）
#[derive(Component, Default, Debug)]
pub struct PlayerLook {
    pub yaw: f32,
    pub pitch: f32,
}

/// 标记相机要跟随的玩家
#[derive(Component, Debug)]
pub struct AttachedToPlayer {
    pub target: Entity,
}