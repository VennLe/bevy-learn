use bevy::prelude::*;

/// 玩家标签组件，配合 #[require] 自动补全必需组件
#[derive(Component, Debug)]
#[require(
    Transform,
    
    crate::core::DespawnOnGameOver,
)]
pub struct Player {
    pub hp: f32,
    pub max_hp: f32,
    pub speed: f32,
    pub sensitivity: f32, // 鼠标灵敏度
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

/// 速度向量组件（用于物理运动）
#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec3);

// 标记第一人称相机
#[derive(Component, Default, Debug)]
pub struct PlayerCamera;

// 玩家输入状态（Yaw 偏航角 / Pitch 俯仰角）
#[derive(Component, Default, Debug)]
pub struct PlayerLook {
    pub yaw: f32,
    pub pitch: f32,
}