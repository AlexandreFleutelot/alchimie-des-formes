mod colors;
mod grid;
pub(crate) mod layout;
pub(crate) mod shapes;

use crate::game;
use crate::game::session::GameSession;
use crate::render::grid::spawn_grid;
use crate::render::layout::{current_offset, target_offset};
use bevy::prelude::*;

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
    let grid_size = current_offset(session.current().width as f32);

    spawn_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        session.current(),
        grid_size,
    );
    spawn_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        session.target(),
        target_offset(),
    );
}
