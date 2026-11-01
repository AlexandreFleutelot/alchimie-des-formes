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
        let mut visited = vec![false; self.width * self.height * 4];
        let mut piece = Vec::<(usize, usize, Direction)>::new();
        let mut queue = VecDeque::new();

        queue.push_back((x, y, quarter));

        while let Some((cx, cy, cquarter)) = queue.pop_front() {
            if visited[(cy * self.width + cx) * 4 + cquarter as usize] {
                continue;
            }
            visited[(cy * self.width + cx) * 4 + cquarter as usize] = true;
            piece.push((cx, cy, cquarter));

            let neighbors = self.get_neighbors(cx, cy, cquarter);
            for (nx, ny, dir) in neighbors {
                if !visited[(ny * self.width + nx) * 4 + dir as usize] {
                    queue.push_back((nx, ny, dir));
                }
            }
        }

        piece
    }
}
