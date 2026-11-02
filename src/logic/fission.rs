use crate::logic::colors::Color;
use super::grid::{Direction, Grid};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Movement{
    dx:i32,
    dy:i32
}

/// TODO: faire une struct (x:usize, y:usize, d:Direction) pour simplifier l'écriture des fonctions

fn move_piece(grid: &Grid, x: usize, y: usize, q:Direction, m:Movement) -> Option<Grid> {

    let mut new_grid = grid.clone();
    let shape = grid.get_piece(x, y, q);

    // Check move possible and get target cells
    let mut targets = Vec::new();
    for &(sx, sy, dir) in &shape {
        let tx = usize::try_from(sx as i32 + m.dx).ok()?;
        let ty = usize::try_from(sy as i32 + m.dy).ok()?;
        if !grid.check_bounds(tx, ty) {
            return None;
        }
        targets.push((tx, ty, dir));
    }

    // Clean position initale
    for &(sx, sy, dir) in &shape {
        new_grid.set(sx, sy, dir, Color::Empty);
    }

    // Copie a la destinaton
    for (&(sx, sy, sdir), &(tx, ty, tdir)) in shape.iter().zip(targets.iter()) {
        let src_color = grid.get(sx, sy, sdir)?;
        let dst_color = new_grid.get(tx, ty, tdir)?;
        let new_color = dst_color.mix(&src_color);
        new_grid.set(tx, ty, tdir, new_color);
    }

    Some(new_grid)
}

#[cfg(test)]
mod tests {
    use crate::logic::colors::Color;
    use crate::logic::fission::{move_piece, Movement};
    use crate::logic::grid::{Direction, Grid};

    fn add_square(grid: &mut Grid, startx: usize, starty:usize, size:usize, color:Color) {
        for i in 0..size {
            for j in 0..size {
                for dir in Direction::ALL {
                    grid.set(startx+i,starty+j,dir, color);
                }
            }
        }
    }

    #[test]
    fn test_movement_with_empty_start() {
    todo!()
    }

    #[test]
    fn test_movement_nul() {
        let mut src = Grid::new(10, 10);
        src.set(5,5,Direction::Nord, Color::Bleu);
        let movement = Movement {dx:0, dy:0};
        let dst = move_piece(&src, 5,5,Direction::Nord, movement);
        assert_eq!(dst,Some(src));
    }

    #[test]
    fn test_movement_outside() {
        let mut src_grid = Grid::new(5,5);
        add_square(&mut src_grid, 2,2,2, Color::Bleu);
        let out_grid = move_piece(&src_grid, 3,3,Direction::Nord,Movement {dx:2, dy:1});
        assert_eq!(out_grid,None);
        let out_grid = move_piece(&src_grid, 3,3,Direction::Nord,Movement {dx:-3, dy:1});
        assert_eq!(out_grid,None);
        let out_grid = move_piece(&src_grid, 2,2,Direction::Ouest,Movement {dx:1, dy:2});
        assert_eq!(out_grid,None);
        let out_grid = move_piece(&src_grid, 3,3,Direction::Nord,Movement {dx:2, dy:-3});
        assert_eq!(out_grid,None);
    }

    #[test]
    fn test_movement_to_empty_area() {
        let mut src_grid = Grid::new(5,5);
        add_square(&mut src_grid, 1,1,2, Color::Bleu);

        let mut expected_grid = Grid::new(5,5);
        add_square(&mut expected_grid, 3,3,2, Color::Bleu);

        let out_grid = move_piece(&src_grid, 1,1,Direction::Nord,Movement {dx:2, dy:2});
        assert_eq!(out_grid,Some(expected_grid));
    }

    #[test]
    fn test_movement_with_overlap() {
        todo!()
    }

    #[test]
    fn test_movement_with_fission() {
        let mut src_grid = Grid::new(5,5);
        add_square(&mut src_grid, 1,1,2, Color::Bleu);
        add_square(&mut src_grid, 3,3,2, Color::Rouge);

        let mut expected_grid = Grid::new(5,5);
        add_square(&mut expected_grid, 2,2,2, Color::Bleu);
        add_square(&mut expected_grid, 3,3,2, Color::Rouge);
        add_square(&mut expected_grid, 3,3,1, Color::Violet);

        let out_grid = move_piece(&src_grid, 1,1,Direction::Nord,Movement {dx:1, dy:1});
        assert_eq!(out_grid,Some(expected_grid));
    }

    #[test]
    fn test_movement_with_diagonal_piece() {
        todo!()
    }

}
