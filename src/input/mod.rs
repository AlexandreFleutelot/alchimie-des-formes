mod drag;
mod keyboard;

pub use drag::cursor_to_world;

use crate::game::AppState;
use crate::logic::grid::QPos;
use bevy::prelude::*;
use drag::drag_piece;
use keyboard::{next_level_on_win, undo_restart};

#[derive(Resource, Default)]
pub struct Drag {
    pub from: Option<QPos>,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Drag>()
            .add_systems(Update, drag_piece.run_if(in_state(AppState::Playing)))
            .add_systems(Update, undo_restart)
            .add_systems(Update, next_level_on_win.run_if(in_state(AppState::Won)));
    }
}
