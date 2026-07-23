use bevy::prelude::*;
use crate::{game_state::GameState, ui::main_menu};
use super::hud;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), hud::spawn_hud)
           .add_systems(Update, hud::update_hud.run_if(in_state(GameState::Playing)));
    }
}
