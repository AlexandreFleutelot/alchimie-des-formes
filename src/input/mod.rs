mod drag;
mod keyboard;

pub use drag::cursor_to_world;

use crate::logic::grid::QPos;
use bevy::prelude::*;
use drag::drag_piece;
use keyboard::undo_restart;

#[derive(Resource, Default)]
pub struct Drag {
    pub from: Option<QPos>,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Drag>()
            .add_systems(Update, (drag_piece, undo_restart));
    }
}
