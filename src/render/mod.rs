mod colors;
mod grid;
mod shapes;

use super::logic::levels::{LEVEL_01, Level, QUARTERS};
use bevy::prelude::*;
use crate::render::grid::spawn_grid;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        println!("RenderPlugin build");
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>,) {
    commands.spawn(Camera2d);

    let lvl = Level::from_file_text(LEVEL_01).expect("Can't to load level 01");
    spawn_grid(&mut commands,  &mut meshes, &mut materials,lvl, 0., 0.);
}
