mod colors;
mod grid;
mod shapes;
mod layout;

use crate::game;
use crate::game::GameSession;
use crate::render::grid::spawn_grid;
use crate::render::shapes::CELL_SIZE;
use bevy::prelude::*;
use crate::render::layout::{current_offset, target_offset};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        println!("RenderPlugin build");
        app.add_systems(Startup, setup.after(game::load_session));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_session: Res<GameSession>,
) {
    commands.spawn(Camera2d);

    let session = &game_session.session;
    let grid_size = session.current().width as f32 * CELL_SIZE;

    spawn_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        session.current(),
        current_offset(grid_size),
    );
    spawn_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        session.target(),
        target_offset(),
    );
}
