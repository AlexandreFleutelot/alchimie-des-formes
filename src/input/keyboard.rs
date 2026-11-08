use super::Drag;
use crate::game::AppState;
use crate::game::session::GameSession;
use crate::logic::levels::{CurrentLevel, LEVELS};
use bevy::prelude::*;

pub fn next_level_on_win(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_session: ResMut<GameSession>,
    mut current_level: ResMut<CurrentLevel>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
        current_level.index = (current_level.index + 1) % LEVELS.len();
        *game_session = GameSession::new_from_level(current_level.index);
        next_state.set(AppState::Playing);
    }
}

pub fn undo_restart(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_session: ResMut<GameSession>,
    mut current_level: ResMut<CurrentLevel>,
    mut drag: ResMut<Drag>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        *game_session = GameSession::new_from_level(current_level.index);
        drag.from = None;
        next_state.set(AppState::Playing);
    } else if keys.just_pressed(KeyCode::KeyN) {
        current_level.index = (current_level.index + 1) % LEVELS.len();
        *game_session = GameSession::new_from_level(current_level.index);
        drag.from = None;
        next_state.set(AppState::Playing);
    } else if keys.just_pressed(KeyCode::KeyU) {
        let _ = game_session.session.undo();
        drag.from = None;
        next_state.set(AppState::Playing);
    }
}
