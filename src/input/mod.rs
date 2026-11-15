mod drag;
mod keyboard;

pub use drag::cursor_to_world;

use crate::game::{AppState, GamePhase};
use crate::logic::grid::QPos;
use bevy::prelude::*;
use drag::drag_piece;
use keyboard::{keyboard_commands, next_level_on_win};

#[derive(Resource, Default)]
pub struct Drag {
    pub from: Option<QPos>,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Drag>()
            .add_systems(Update, drag_piece.run_if(in_state(GamePhase::Playing)))
            .add_systems(Update, keyboard_commands.run_if(in_state(AppState::InGame)))
            .add_systems(Update, next_level_on_win.run_if(in_state(GamePhase::Won)))
            .add_systems(OnExit(AppState::InGame), exit_in_game); //handle escape while dragging
    }
}

fn exit_in_game(mut drag: ResMut<Drag>) {
    drag.from = None;
}
