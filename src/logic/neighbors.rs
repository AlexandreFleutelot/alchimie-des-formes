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
