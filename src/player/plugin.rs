use super::systems;
use crate::game_state::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), systems::spawn_player)
           .add_systems(Update, (
               systems::player_look,
               systems::sync_player_and_camera,  // 合并后的系统
               systems::player_move,
               systems::toggle_mouse_lock,
           ).chain()  // 使用 chain() 确保按顺序执行
           .run_if(in_state(GameState::Playing)));
    }
}