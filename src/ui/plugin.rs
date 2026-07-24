use bevy::prelude::*;
use crate::{game_state::GameState};
use super::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
           .add_systems(Update, menu_button_system.run_if(in_state(GameState::Menu)))
           .add_systems(OnExit(GameState::Menu), close_main_menu);
    }
}
