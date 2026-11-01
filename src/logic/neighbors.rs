use super::grid::Direction;
use super::grid::Grid;

impl Grid {
    pub fn get_neighbors(
        &self,
        x: usize,
        y: usize,
        quarter: Direction,
    ) -> Vec<(usize, usize, Direction)> {
        let mut neighbors = Vec::new();

        match quarter {
            Direction::Nord => {
                neighbors.push((x, y, Direction::Est));
                neighbors.push((x, y, Direction::Ouest));
                if y > 0 {
                    neighbors.push((x, y - 1, Direction::Sud));
                }
            }
            Direction::Est => {
                neighbors.push((x, y, Direction::Nord));
                neighbors.push((x, y, Direction::Sud));
                if x < self.width - 1 {
                    neighbors.push((x + 1, y, Direction::Ouest));
                }
            }
            Direction::Sud => {
                neighbors.push((x, y, Direction::Est));
                neighbors.push((x, y, Direction::Ouest));
                if y < self.height - 1 {
                    neighbors.push((x, y + 1, Direction::Nord));
                }
            }
            Direction::Ouest => {
                neighbors.push((x, y, Direction::Nord));
                neighbors.push((x, y, Direction::Sud));
                if x > 0 {
                    neighbors.push((x - 1, y, Direction::Est));
                }
            }
        };

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbors() {
        let grid = Grid::new(10, 10);
        let neighbors = grid.get_neighbors(5, 5, Direction::Nord);
        let target_neighbors = vec![
            (5, 5, Direction::Est),
            (5, 5, Direction::Ouest),
            (5, 4, Direction::Sud),
        ];
        assert_eq!(neighbors.len(), 3);
        assert_eq!(neighbors, target_neighbors);
    }

    #[test]
    fn test_get_neighbors_edge() {
        let grid = Grid::new(10, 10);
        let neighbors = grid.get_neighbors(0, 0, Direction::Ouest);
        let target_neighbors = vec![(0, 0, Direction::Nord), (0, 0, Direction::Sud)];
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors, target_neighbors);
    }
}
