use super::grid::{Grid, QPos, Quarter};

impl Grid {
    pub fn get_neighbors(&self, pos: QPos) -> Vec<QPos> {
        let mut neighbors = Vec::new();

        match pos.q {
            Quarter::Nord => {
                neighbors.push(QPos::new(pos.x, pos.y, Quarter::Est));
                neighbors.push(QPos::new(pos.x, pos.y, Quarter::Ouest));
                if pos.y > 0 {
                    neighbors.push(QPos::new(pos.x, pos.y - 1, Quarter::Sud));
                }
            }
            Quarter::Est => {
                neighbors.push(QPos::new(pos.x, pos.y, Quarter::Nord));
                neighbors.push(QPos::new(pos.x, pos.y, Quarter::Sud));
                if pos.x < self.width - 1 {
                    neighbors.push(QPos::new(pos.x + 1, pos.y, Quarter::Ouest));
                }
            }
            Quarter::Sud => {
                neighbors.push(QPos::new(pos.x, pos.y, Quarter::Est));
                neighbors.push(QPos::new(pos.x, pos.y, Quarter::Ouest));
                if pos.y < self.height - 1 {
                    neighbors.push(QPos::new(pos.x, pos.y + 1, Quarter::Nord));
                }
            }
            Quarter::Ouest => {
                neighbors.push(QPos::new(pos.x, pos.y, Quarter::Nord));
                neighbors.push(QPos::new(pos.x, pos.y, Quarter::Sud));
                if pos.x > 0 {
                    neighbors.push(QPos::new(pos.x - 1, pos.y, Quarter::Est));
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
        let neighbors = grid.get_neighbors(QPos::new(5, 5, Quarter::Nord));
        let target_neighbors = vec![
            QPos::new(5, 5, Quarter::Est),
            QPos::new(5, 5, Quarter::Ouest),
            QPos::new(5, 4, Quarter::Sud),
        ];
        assert_eq!(neighbors.len(), 3);
        assert_eq!(neighbors, target_neighbors);
    }

    #[test]
    fn test_get_neighbors_edge() {
        let grid = Grid::new(10, 10);
        let neighbors = grid.get_neighbors(QPos::new(0, 0, Quarter::Ouest));
        let target_neighbors = vec![
            QPos::new(0, 0, Quarter::Nord),
            QPos::new(0, 0, Quarter::Sud),
        ];
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors, target_neighbors);
    }

    #[test]
    fn test_grid_coherency() {
        let grid = Grid::new(5, 5);

        for x in 0..grid.width {
            for y in 0..grid.height {
                for quarter in Quarter::ALL {
                    let neighbors = grid.get_neighbors(QPos::new(x, y, quarter));
                    for pos in neighbors {
                        let neighbor_neighbors = grid.get_neighbors(pos);
                        assert!(
                            neighbor_neighbors.contains(&QPos::new(x, y, quarter)),
                            "Neighbor ({},{},{:?}) does not have ({x},{y},{quarter:?}) as a neighbor",
                            pos.x,
                            pos.y,
                            pos.q
                        );
                    }
                }
            }
        }
    }
}
