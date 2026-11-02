use super::colors::QColor;
use super::grid::{Grid, QPos};
use std::collections::VecDeque;

impl Grid {
    /// Use BFS to find all connected cells starting from (x, y, quarter)
    pub fn get_piece(&self, pos: QPos) -> Vec<QPos> {
        let mut visited = vec![false; self.cells.len()];
        let mut piece = Vec::<QPos>::new();
        let mut queue = VecDeque::<QPos>::new();

        let Some(piece_color) = self.get(pos) else {
            return Vec::new(); // outside of grid bounds
        };
        if piece_color == QColor::Empty {
            return Vec::new(); // empty cell, no piece to find
        }
        queue.push_back(pos);
        while let Some(c_pos) = queue.pop_front() {
            if visited[self.get_index(c_pos)] {
                continue;
            }
            visited[self.get_index(c_pos)] = true;
            piece.push(c_pos);

            let neighbors = self.get_neighbors(c_pos);
            for n_pos in neighbors {
                if !visited[self.get_index(n_pos)] && self.get(n_pos) == Some(piece_color) {
                    queue.push_back(n_pos);
                }
            }
        }

        piece
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::colors::QColor;
    use crate::logic::grid::{Grid, QPos, Quarter};

    #[test]
    fn test_out_of_grid() {
        let grid = Grid::new(3, 3);
        let piece = grid.get_piece(QPos::new(5, 5, Quarter::Nord));
        assert!(piece.is_empty());
    }

    #[test]
    fn test_quart_isole() {
        let mut grid = Grid::new(3, 3);
        let pos = QPos::new(1, 1, Quarter::Nord);
        grid.set(pos, QColor::Bleu);

        let piece = grid.get_piece(pos);
        assert_eq!(piece, vec![pos]);
    }

    #[test]
    fn test_case_entiere() {
        let mut grid = Grid::new(3, 3);
        for dir in Quarter::ALL {
            grid.set(QPos::new(1, 1, dir), QColor::Bleu);
        }

        for start_dir in Quarter::ALL {
            let piece = grid.get_piece(QPos::new(1, 1, start_dir));
            assert_eq!(piece.len(), 4);
            for dir in Quarter::ALL {
                assert!(piece.contains(&QPos::new(1, 1, dir)));
            }
        }
    }

    #[test]
    fn test_nord_sud() {
        let mut grid = Grid::new(3, 3);
        grid.set(QPos::new(1, 1, Quarter::Nord), QColor::Bleu);
        grid.set(QPos::new(1, 1, Quarter::Sud), QColor::Bleu);

        let piece = grid.get_piece(QPos::new(1, 1, Quarter::Nord));
        assert_eq!(piece, vec![QPos::new(1, 1, Quarter::Nord)]);
        assert!(!piece.contains(&QPos::new(1, 1, Quarter::Sud)));
    }

    #[test]
    fn test_two_cells() {
        let mut grid = Grid::new(3, 3);

        // horizontal
        grid.set(QPos::new(0, 1, Quarter::Est), QColor::Rouge);
        grid.set(QPos::new(1, 1, Quarter::Ouest), QColor::Rouge);
        let piece_h = grid.get_piece(QPos::new(1, 1, Quarter::Ouest));
        assert_eq!(piece_h.len(), 2);

        // vertical
        grid.set(QPos::new(1, 0, Quarter::Sud), QColor::Bleu);
        grid.set(QPos::new(1, 1, Quarter::Nord), QColor::Bleu);
        let piece_v = grid.get_piece(QPos::new(1, 1, Quarter::Nord));
        assert_eq!(piece_v.len(), 2);
    }

    #[test]
    fn test_two_colors() {
        let mut grid = Grid::new(3, 3);
        for dir in Quarter::ALL {
            grid.set(QPos::new(1, 1, dir), QColor::Bleu);
            grid.set(QPos::new(0, 1, dir), QColor::Rouge);
        }

        let piece = grid.get_piece(QPos::new(0, 1, Quarter::Est));
        assert_eq!(piece.len(), 4);
        assert!(piece.iter().all(|&pos| pos.x == 0));
    }

    #[test]
    fn test_empty_cell() {
        let grid = Grid::new(3, 3);
        let piece = grid.get_piece(QPos::new(1, 1, Quarter::Nord));
        assert!(piece.is_empty());
    }

    #[test]
    fn test_coin() {
        let mut grid = Grid::new(3, 3);
        for dir in Quarter::ALL {
            grid.set(QPos::new(0, 0, dir), QColor::Orange);
        }

        let piece = grid.get_piece(QPos::new(0, 0, Quarter::Nord));
        assert_eq!(piece.len(), 4);
        for dir in Quarter::ALL {
            assert!(piece.contains(&QPos::new(0, 0, dir)));
        }
    }

    #[test]
    fn test_forme_l() {
        let mut grid = Grid::new(3, 3);
        for dir in Quarter::ALL {
            grid.set(QPos::new(0, 0, dir), QColor::Rouge);
            grid.set(QPos::new(0, 1, dir), QColor::Rouge);
            grid.set(QPos::new(1, 1, dir), QColor::Rouge);
        }

        let piece = grid.get_piece(QPos::new(0, 0, Quarter::Nord));
        assert_eq!(piece.len(), 12);
        assert!(piece.contains(&QPos::new(1, 1, Quarter::Est)));
    }
}
