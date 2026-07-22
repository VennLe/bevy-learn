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
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hp: 100.0,
            max_hp: 100.0,
            speed: 5.0,
        }
    }
}

/// 速度向量组件（用于物理运动）
#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec3);