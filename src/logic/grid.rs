use super::colors::Color;

#[repr(u8)]
pub enum Direction { Nord, Est, Sud, Ouest }

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
            cells: vec![Color::Empty; width * height],
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Color {
        self.cells[y * self.width + x]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: Color) {
        self.cells[y * self.width + x] = value;
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
        assert_eq!(grid.cells.len(), 100);
    }

    #[test]
    fn test_get_cell() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.get_cell(5, 5), Color::Empty);
    }

    #[test]
    fn test_set_cell() {
        let mut grid = Grid::new(10, 10);
        grid.set_cell(5, 5, Color::Rouge);
        assert_eq!(grid.get_cell(5, 5), Color::Rouge);
    }
}