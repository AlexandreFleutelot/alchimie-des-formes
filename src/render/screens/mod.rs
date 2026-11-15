mod level_select;
mod menu;
mod win;

use crate::{
    game::{AppState, GamePhase},
    render::screens::level_select::show_level_select_screen,
};
use bevy::prelude::*;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), menu::show_menu_screen)
            .add_systems(OnEnter(AppState::LevelSelect), show_level_select_screen)
            .add_systems(OnEnter(GamePhase::Won), win::show_win_screen);
    }
}
