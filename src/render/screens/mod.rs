mod win;

use crate::game::GamePhase;
use bevy::prelude::*;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Won), win::show_win_screen);
    }
}
