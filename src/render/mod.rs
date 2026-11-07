mod colors;
mod grid;
mod shapes;

use super::logic::levels::{LEVEL_01, Level};
use crate::logic::sessions::Session;
use crate::render::grid::spawn_grid;
use crate::render::shapes::CELL_SIZE;
use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        println!("RenderPlugin build");
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let lvl = Level::from_file_text(LEVEL_01).expect("Can't to load level 01");
    let session: Session = Session::new(lvl.start, lvl.target);

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
