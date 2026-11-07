mod colors;
mod grid;
mod shapes;

use crate::game;
use crate::game::GameSession;
use crate::render::grid::spawn_grid;
use crate::render::shapes::CELL_SIZE;
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
    game: Res<GameSession>,
) {
    commands.spawn(Camera2d);

    let session = &game.session;
    let grid_size = session.current().width as f32 * CELL_SIZE;

    spawn_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        &session.current(),
        Vec2::new(-grid_size - 50., 50.),
    );
    spawn_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        &session.target(),
        Vec2::new(50., 50.),
    );
}
