use crate::logic::grid::{Grid, QPos, Quarter};
use bevy::math::Vec2;
use bevy::prelude::Triangle2d;

pub const CELL_SIZE: f32 = 50.;

pub fn quarter_triangle(q: Quarter) -> Triangle2d {
    let mid = Vec2::new(CELL_SIZE / 2., CELL_SIZE / 2.);
    match q {
        Quarter::Nord => Triangle2d::new(
            Vec2::new(0., CELL_SIZE),
            Vec2::new(CELL_SIZE, CELL_SIZE),
            mid,
        ),
        Quarter::Est => Triangle2d::new(
            Vec2::new(CELL_SIZE, CELL_SIZE),
            Vec2::new(CELL_SIZE, 0.),
            mid,
        ),
        Quarter::Sud => Triangle2d::new(Vec2::new(CELL_SIZE, 0.), Vec2::new(0., 0.), mid),
        Quarter::Ouest => Triangle2d::new(Vec2::new(0., 0.), Vec2::new(0., CELL_SIZE), mid),
    }
}

pub fn world_to_qpos(grid: &Grid, point: Vec2, offset: Vec2) -> Option<QPos> {
    let cx = (point.x - offset.x) / CELL_SIZE;
    let cy = (offset.y - point.y) / CELL_SIZE;

    if cx >= 0.0 && cx < grid.width as f32 && cy >= 0.0 && cy < grid.height as f32 {
        let cx_int = cx.floor();
        let cy_int = cy.floor();

        let dx = cx - cx_int;
        let dy = 1.0 - (cy - cy_int);

        let q = match (dy > dx, 1.0 - dy < dx) {
            (true, true) => Quarter::Nord,
            (true, false) => Quarter::Ouest,
            (false, true) => Quarter::Est,
            (false, false) => Quarter::Sud,
        };

        return Some(QPos::new(cx_int as usize, cy_int as usize, q));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_world_to_qpos_top_border() {
        let grid = Grid::new(4, 4);
        let offset = Vec2::new(50.0, 50.0);

        let p_nord = Vec2::new(offset.x + CELL_SIZE / 2., offset.y);
        assert_eq!(
            world_to_qpos(&grid, p_nord, offset),
            Some(QPos::new(0, 0, Quarter::Nord))
        );
    }

    #[test]
    fn test_world_to_qpos_out_of_grid() {
        let grid = Grid::new(4, 4);
        let offset = Vec2::new(50.0, 50.0);

        let p_out = Vec2::new(offset.x - CELL_SIZE / 2., offset.y);
        assert_eq!(world_to_qpos(&grid, p_out, offset), None);
    }

    #[test]
    fn test_world_to_qpos_to_q() {
        let grid = Grid::new(4, 4);
        let offset = Vec2::new(50.0, 50.0);

        let eps = CELL_SIZE / 5.;
        // le centre de chaque cellule (avec un décalage dans chaque direction)
        for row in 0..grid.height {
            for col in 0..grid.width {
                let center = Vec2::new(
                    offset.x + (col as f32 + 0.5) * CELL_SIZE,
                    offset.y - (row as f32 + 0.5) * CELL_SIZE,
                );

                let test_nord = center + eps * Vec2::new(0.0, 1.0);
                assert_eq!(
                    world_to_qpos(&grid, test_nord, offset),
                    Some(QPos::new(col, row, Quarter::Nord)),
                    "Erreur de cohérence Quart Nord pour la cellule ({col}, {row})",
                );

                let test_est = center + eps * Vec2::new(1.0, 0.0);
                assert_eq!(
                    world_to_qpos(&grid, test_est, offset),
                    Some(QPos::new(col, row, Quarter::Est)),
                    "Erreur de cohérence Quart est pour la cellule ({col}, {row})",
                );

                let test_sud = center + eps * Vec2::new(0.0, -1.0);
                assert_eq!(
                    world_to_qpos(&grid, test_sud, offset),
                    Some(QPos::new(col, row, Quarter::Sud)),
                    "Erreur de cohérence Quart Sud pour la cellule ({col}, {row})",
                );

                let test_ouest = center + eps * Vec2::new(-1.0, 0.0);
                assert_eq!(
                    world_to_qpos(&grid, test_ouest, offset),
                    Some(QPos::new(col, row, Quarter::Ouest)),
                    "Erreur de cohérence Quart Ouest pour la cellule ({col}, {row})",
                );
            }
        }
    }
}
