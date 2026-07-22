use bevy::prelude::*;

#[derive(Component, Debug)]
#[require(
    Transform,
    crate::core::DespawnOnGameOver,
)]
pub struct Enemy {
    pub damage: f32,
    pub speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            damage: 10.0,
            speed: 2.0,
        }
    }
}