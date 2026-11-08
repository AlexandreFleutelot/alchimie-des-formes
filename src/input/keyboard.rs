use super::Drag;
use crate::game::session::GameSession;
use crate::logic::levels::{CurrentLevel, LEVELS};
use bevy::prelude::*;

pub fn undo_restart(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_session: ResMut<GameSession>,
    mut current_level: ResMut<CurrentLevel>,
    mut drag: ResMut<Drag>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        *game_session = GameSession::new_from_level(current_level.index);
        drag.from = None;
    } else if keys.just_pressed(KeyCode::KeyN) {
        current_level.index = (current_level.index + 1) % LEVELS.len();
        *game_session = GameSession::new_from_level(current_level.index);
    } else if keys.just_pressed(KeyCode::KeyU) {
        let _ = game_session.session.undo();
        drag.from = None;
    }
}
