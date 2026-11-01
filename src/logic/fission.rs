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

    // Check move possible
    let targets= shape.iter()
        .map(|&(sx,sy,dir)| {
            let tx =usize::try_from(sx as i32 + m.dx).ok()?;
            let ty = usize::try_from(sy as i32 + m.dy).ok()?;
            Some((tx, ty, dir))
        }).collect::<Option<Vec<(usize, usize, Direction)>>>()?;

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
        todo!()
    }

    #[test]
    fn test_movement_empty_area() {

    }

    #[test]
    fn test_movement_with_fission() {

    }
}
