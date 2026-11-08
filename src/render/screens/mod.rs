mod win;

use crate::game::AppState;
use bevy::prelude::*;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Won), win::show_win_text);
    }
}
