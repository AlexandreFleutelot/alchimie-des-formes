use crate::logic::grid::{Grid, QPos};
use crate::logic::levels::QUARTERS;
use crate::render::colors::to_color;
use crate::render::shapes::{CELL_SIZE, quarter_triangle};
use bevy::mesh::Mesh2d;
use bevy::prelude::*;

pub fn spawn_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    grid: Grid,
    offset_x: f32,
    offset_y: f32,
) {
    for x in 0..grid.width {
        for y in 0..grid.height {
            for q in QUARTERS.iter() {
                let Some(rgb) = grid.get(QPos::new(x, y, *q)).and_then(to_color) else {
                    continue;
                };
                commands.spawn((
                    Mesh2d(meshes.add(quarter_triangle(*q))),
                    MeshMaterial2d(materials.add(rgb)),
                    Transform::from_xyz(
                        offset_x + CELL_SIZE * x as f32,
                        offset_y + CELL_SIZE * y as f32, 0.
                    ),
                ));
            }
        }
    }
}
