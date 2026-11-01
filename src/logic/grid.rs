use super::colors::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Direction {
    Nord,
    Est,
    Sud,
    Ouest,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Color>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![Color::Empty; 4 * width * height],
        }
    }

    pub fn check_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_cell(&self, x: usize, y: usize, dir: Direction) -> Option<Color> {
        if self.check_bounds(x, y) {
            Some(self.cells[4 * (y * self.width + x) + dir as usize])
        } else {
            None
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, dir: Direction, value: Color) {
        assert!(self.check_bounds(x, y), "case ({x},{y}) hors grille");
        self.cells[4 * (y * self.width + x) + dir as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.cells.len(), 400);
    }

    #[test]
    fn test_get_cell() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.get_cell(5, 5, Direction::Nord), Some(Color::Empty));
    }

    #[test]
    fn test_set_cell() {
        let mut grid = Grid::new(10, 10);
        grid.set_cell(5, 5, Direction::Est, Color::Rouge);
        assert_eq!(grid.get_cell(5, 5, Direction::Est), Some(Color::Rouge));
    }

    #[test]
    #[should_panic(expected = "case (10,10) hors grille")]
    fn test_set_cell_out_of_bounds() {
        let mut grid = Grid::new(10, 10);
        grid.set_cell(10, 10, Direction::Ouest, Color::Rouge);
    }

    #[test]
    fn test_get_cell_out_of_bounds() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.get_cell(10, 10, Direction::Sud), None);
    }
}
