mod drag;
mod keyboard;

use crate::logic::grid::QPos;
use bevy::prelude::*;
use drag::drag_piece;
use keyboard::undo_restart;

#[derive(Resource, Default)]
pub struct Drag {
    from: Option<QPos>,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Drag>()
            .add_systems(Update, (drag_piece, undo_restart));
    }
}
