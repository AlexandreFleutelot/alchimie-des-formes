use crate::logic::grid::{Grid, QPos};
use crate::logic::levels::QUARTERS;
use crate::render::colors::to_color;
use crate::render::shapes::{CELL_SIZE, quarter_triangle};
use bevy::mesh::Mesh2d;
use bevy::prelude::*;

#[derive(Component)]
pub struct CellVisual;

pub fn spawn_grid(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    grid: &Grid,
    offset: Vec2,
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
                        offset[0] + CELL_SIZE * x as f32,
                        offset[1] - CELL_SIZE * y as f32,
                        0.,
                    ),
                    CellVisual,
                ));
            }
        }
    }
}
