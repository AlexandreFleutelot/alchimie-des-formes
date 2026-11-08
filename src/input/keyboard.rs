use super::Drag;
use crate::game::session::GameSession;
use bevy::prelude::*;

pub fn undo_restart(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_session: ResMut<GameSession>,
    mut drag: ResMut<Drag>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        *game_session = GameSession::new_from_level();
        drag.from = None;
    } else if keys.just_pressed(KeyCode::KeyU) {
        if game_session
            .bypass_change_detection()
            .session
            .undo()
            .is_ok()
        {
            game_session.set_changed();
            drag.from = None;
        }
    }
}
