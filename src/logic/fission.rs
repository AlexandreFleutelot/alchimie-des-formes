use super::grid::{Grid, QPos};
use crate::logic::colors::QColor;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoveError {
    NoShapeSelected,
    SelectionOutOfGrid,
    TargetOutOfGrid,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Movement {
    dx: i32,
    dy: i32,
}

pub fn move_piece(grid: &Grid, pos: QPos, m: Movement) -> Result<Grid, MoveError> {
    let mut new_grid = grid.clone();
    let shape = grid.get_piece(pos);

    match grid.get(pos) {
        None => Err(MoveError::SelectionOutOfGrid),
        Some(QColor::Empty) => Err(MoveError::NoShapeSelected),
        Some(c) => Ok(c),
    }?;

    // Check move possible and get target cells
    let mut targets = Vec::new();
    for &s_pos in &shape {
        let tx = usize::try_from(s_pos.x as i32 + m.dx).map_err(|_| MoveError::TargetOutOfGrid)?;
        let ty = usize::try_from(s_pos.y as i32 + m.dy).map_err(|_| MoveError::TargetOutOfGrid)?;
        if !grid.check_bounds(tx, ty) {
            return Err(MoveError::TargetOutOfGrid);
        }
        targets.push(QPos::new(tx, ty, s_pos.q));
    }

    // Clean position initale
    for &s_pos in &shape {
        new_grid.set(s_pos, QColor::Empty);
    }

    // Copie a la destinaton
    for (&s_pos, &t_pos) in shape.iter().zip(targets.iter()) {
        let src_color = grid.get(s_pos).expect("source control failed");
        let dst_color = new_grid.get(t_pos).expect("destination control failed");
        let new_color = dst_color.mix(&src_color);
        new_grid.set(t_pos, new_color);
    }

    Ok(new_grid)
}

#[cfg(test)]
mod tests {
    use crate::logic::colors::QColor;
    use crate::logic::fission::{MoveError, Movement, move_piece};
    use crate::logic::grid::{Grid, QPos, Quarter};
    use bevy::window::SystemCursorIcon::Move;

    fn add_square(grid: &mut Grid, startx: usize, starty: usize, size: usize, color: QColor) {
        for i in 0..size {
            for j in 0..size {
                for dir in Quarter::ALL {
                    let pos = QPos::new(startx + i, starty + j, dir);
                    grid.set(pos, color);
                }
            }
        }
    }

    #[test]
    fn test_movement_with_empty_start() {
        let mut src = Grid::new(10, 10);
        src.set(QPos::new(5, 5, Quarter::Nord), QColor::Bleu);
        let movement = Movement { dx: 1, dy: 2 };
        let dst = move_piece(&src, QPos::new(6, 5, Quarter::Nord), movement);
        assert_eq!(dst, Err(MoveError::NoShapeSelected));
    }

    #[test]
    fn test_movement_nul() {
        let mut src = Grid::new(10, 10);
        src.set(QPos::new(5, 5, Quarter::Nord), QColor::Bleu);
        let movement = Movement { dx: 0, dy: 0 };
        let dst = move_piece(&src, QPos::new(5, 5, Quarter::Nord), movement);
        assert_eq!(dst, Ok(src));
    }

    #[test]
    fn test_movement_outside() {
        let mut src_grid = Grid::new(5, 5);
        add_square(&mut src_grid, 2, 2, 2, QColor::Bleu);
        let out_grid = move_piece(
            &src_grid,
            QPos::new(3, 3, Quarter::Nord),
            Movement { dx: 2, dy: 1 },
        );
        assert_eq!(out_grid, Err(MoveError::TargetOutOfGrid));
        let out_grid = move_piece(
            &src_grid,
            QPos::new(3, 3, Quarter::Nord),
            Movement { dx: -3, dy: 1 },
        );
        assert_eq!(out_grid, Err(MoveError::TargetOutOfGrid));
        let out_grid = move_piece(
            &src_grid,
            QPos::new(2, 2, Quarter::Ouest),
            Movement { dx: 1, dy: 2 },
        );
        assert_eq!(out_grid, Err(MoveError::TargetOutOfGrid));
        let out_grid = move_piece(
            &src_grid,
            QPos::new(3, 3, Quarter::Nord),
            Movement { dx: 2, dy: -3 },
        );
        assert_eq!(out_grid, Err(MoveError::TargetOutOfGrid));
    }

    #[test]
    fn test_movement_to_empty_area() {
        let mut src_grid = Grid::new(5, 5);
        add_square(&mut src_grid, 1, 1, 2, QColor::Bleu);

        let mut expected_grid = Grid::new(5, 5);
        add_square(&mut expected_grid, 3, 3, 2, QColor::Bleu);

        let out_grid = move_piece(
            &src_grid,
            QPos::new(1, 1, Quarter::Nord),
            Movement { dx: 2, dy: 2 },
        );
        assert_eq!(out_grid, Ok(expected_grid));
    }

    #[test]
    fn test_movement_with_overlap() {
        let mut src_grid = Grid::new(5, 5);
        add_square(&mut src_grid, 1, 1, 2, QColor::Bleu);

        let mut expected_grid = Grid::new(5, 5);
        add_square(&mut expected_grid, 2, 2, 2, QColor::Bleu);

        let out_grid = move_piece(
            &src_grid,
            QPos::new(1, 1, Quarter::Nord),
            Movement { dx: 1, dy: 1 },
        );
        assert_eq!(out_grid, Ok(expected_grid));
    }

    #[test]
    fn test_movement_with_fission() {
        let mut src_grid = Grid::new(5, 5);
        add_square(&mut src_grid, 1, 1, 2, QColor::Bleu);
        add_square(&mut src_grid, 3, 3, 2, QColor::Rouge);

        let mut expected_grid = Grid::new(5, 5);
        add_square(&mut expected_grid, 2, 2, 2, QColor::Bleu);
        add_square(&mut expected_grid, 3, 3, 2, QColor::Rouge);
        add_square(&mut expected_grid, 3, 3, 1, QColor::Violet);

        let out_grid = move_piece(
            &src_grid,
            QPos::new(1, 1, Quarter::Nord),
            Movement { dx: 1, dy: 1 },
        );
        assert_eq!(out_grid, Ok(expected_grid));
    }

    #[test]
    fn test_movement_with_diagonal_piece() {
        let mut src_grid = Grid::new(3, 3);
        add_square(&mut src_grid, 1, 1, 1, QColor::Bleu);
        src_grid.set(QPos::new(2, 1, Quarter::Est), QColor::Jaune);
        src_grid.set(QPos::new(2, 1, Quarter::Sud), QColor::Jaune);

        let out_grid = move_piece(
            &src_grid,
            QPos::new(2, 1, Quarter::Sud),
            Movement { dx: -1, dy: 0 },
        )
        .unwrap();

        assert_eq!(
            out_grid.get(QPos::new(1, 1, Quarter::Nord)),
            Some(QColor::Bleu)
        );
        assert_eq!(
            out_grid.get(QPos::new(1, 1, Quarter::Ouest)),
            Some(QColor::Bleu)
        );
        assert_eq!(
            out_grid.get(QPos::new(1, 1, Quarter::Est)),
            Some(QColor::Vert)
        );
        assert_eq!(
            out_grid.get(QPos::new(1, 1, Quarter::Sud)),
            Some(QColor::Vert)
        );
    }
}
