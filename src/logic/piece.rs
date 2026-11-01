use super::colors::Color;
use super::grid::{Direction, Grid};
use std::collections::VecDeque;

impl Grid {
    /// Use BFS to find all connected cells starting from (x, y, quarter)
    pub fn get_piece(
        &self,
        x: usize,
        y: usize,
        quarter: Direction,
    ) -> Vec<(usize, usize, Direction)> {
        let mut visited = vec![false; self.cells.len()];
        let mut piece = Vec::<(usize, usize, Direction)>::new();
        let mut queue = VecDeque::new();

        let Some(piece_color) = self.get(x, y, quarter) else {
            return Vec::new(); // outside of grid bounds
        };
        if piece_color == Color::Empty {
            return Vec::new(); // empty cell, no piece to find
        }
        queue.push_back((x, y, quarter));
        while let Some((cx, cy, cquarter)) = queue.pop_front() {
            if visited[self.get_index(cx, cy, cquarter)] {
                continue;
            }
            visited[self.get_index(cx, cy, cquarter)] = true;
            piece.push((cx, cy, cquarter));

            let neighbors = self.get_neighbors(cx, cy, cquarter);
            for (nx, ny, dir) in neighbors {
                if !visited[self.get_index(nx, ny, dir)]
                    && self.get(nx, ny, dir) == Some(piece_color)
                {
                    queue.push_back((nx, ny, dir));
                }
            }
        }

        piece
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::colors::Color;
    use crate::logic::grid::{Direction, Grid};

    #[test]
    fn test_out_of_grid() {
        let grid = Grid::new(3, 3);
        let piece = grid.get_piece(5, 5, Direction::Nord);
        assert!(piece.is_empty());
    }

    #[test]
    fn test_quart_isole() {
        let mut grid = Grid::new(3, 3);
        grid.set(1, 1, Direction::Nord, Color::Bleu);

        let piece = grid.get_piece(1, 1, Direction::Nord);
        assert_eq!(piece, vec![(1, 1, Direction::Nord)]);
    }

    #[test]
    fn test_case_entiere() {
        let mut grid = Grid::new(3, 3);
        for dir in Direction::ALL {
            grid.set(1, 1, dir, Color::Bleu);
        }

        for start_dir in Direction::ALL {
            let piece = grid.get_piece(1, 1, start_dir);
            assert_eq!(piece.len(), 4);
            for dir in Direction::ALL {
                assert!(piece.contains(&(1, 1, dir)));
            }
        }
    }

    #[test]
    fn test_nord_sud() {
        let mut grid = Grid::new(3, 3);
        grid.set(1, 1, Direction::Nord, Color::Bleu);
        grid.set(1, 1, Direction::Sud, Color::Bleu);

        let piece = grid.get_piece(1, 1, Direction::Nord);
        assert_eq!(piece, vec![(1, 1, Direction::Nord)]);
        assert!(!piece.contains(&(1, 1, Direction::Sud)));
    }

    #[test]
    fn test_two_cells() {
        let mut grid = Grid::new(3, 3);

        // horizontal
        grid.set(0, 1, Direction::Est, Color::Rouge);
        grid.set(1, 1, Direction::Ouest, Color::Rouge);
        let piece_h = grid.get_piece(1, 1, Direction::Ouest);
        assert_eq!(piece_h.len(), 2);

        // vertical
        grid.set(1, 0, Direction::Sud, Color::Bleu);
        grid.set(1, 1, Direction::Nord, Color::Bleu);
        let piece_v = grid.get_piece(1, 1, Direction::Nord);
        assert_eq!(piece_v.len(), 2);
    }

    #[test]
    fn test_two_colors() {
        let mut grid = Grid::new(3, 3);
        for dir in Direction::ALL {
            grid.set(1, 1, dir, Color::Bleu);
            grid.set(0, 1, dir, Color::Rouge);
        }

        let piece = grid.get_piece(0, 1, Direction::Est);
        assert_eq!(piece.len(), 4);
        assert!(piece.iter().all(|&(x, _, _)| x == 0));
    }

    #[test]
    fn test_empty_cell() {
        let grid = Grid::new(3, 3);
        let piece = grid.get_piece(1, 1, Direction::Nord);
        assert!(piece.is_empty());
    }

    #[test]
    fn test_coin() {
        let mut grid = Grid::new(3, 3);
        for dir in Direction::ALL {
            grid.set(0, 0, dir, Color::Orange);
        }

        let piece = grid.get_piece(0, 0, Direction::Nord);
        assert_eq!(piece.len(), 4);
        for dir in Direction::ALL {
            assert!(piece.contains(&(0, 0, dir)));
        }
    }

    #[test]
    fn test_forme_l() {
        let mut grid = Grid::new(3, 3);
        for dir in Direction::ALL {
            grid.set(0, 0, dir, Color::Rouge);
            grid.set(0, 1, dir, Color::Rouge);
            grid.set(1, 1, dir, Color::Rouge);
        }

        let piece = grid.get_piece(0, 0, Direction::Nord);
        assert_eq!(piece.len(), 12);
        assert!(piece.contains(&(1, 1, Direction::Est)));
    }
}
