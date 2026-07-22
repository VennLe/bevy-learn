use bevy::prelude::*;
use crate::game_state::GameState;
use super::systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), systems::spawn_player)
           .add_systems(Update, (
               systems::player_move,
               systems::take_damage,
           ).run_if(in_state(GameState::Playing)));
    }
}