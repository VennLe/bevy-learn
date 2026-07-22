use bevy::prelude::*;
use crate::game_state::GameState;
use super::systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), systems::spawn_enemies)
           .add_systems(Update, systems::enemy_move.run_if(in_state(GameState::Playing)));
    }
}