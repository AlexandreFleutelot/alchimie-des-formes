use super::colors::QColor;

/// Quart de case
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Quarter {
    Nord,
    Est,
    Sud,
    Ouest,
}

impl Quarter {
    pub const ALL: [Quarter; 4] = [Quarter::Nord, Quarter::Est, Quarter::Sud, Quarter::Ouest];
}

/// Quarter position
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QPos {
    pub x: usize,
    pub y: usize,
    pub q: Quarter,
}

impl QPos {
    pub fn new(x: usize, y: usize, q: Quarter) -> QPos {
        QPos { x, y, q }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<QColor>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        assert!(
            width > 0 && height > 0,
            "La largeur et la hauteur de la grille doivent être supérieures à zéro"
        );
        Grid {
            width,
            height,
            cells: vec![QColor::Empty; 4 * width * height],
        }
    }

    pub fn check_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_index(&self, pos: QPos) -> usize {
        4 * (pos.y * self.width + pos.x) + pos.q as usize
    }

    pub fn get(&self, pos: QPos) -> Option<QColor> {
        if self.check_bounds(pos.x, pos.y) {
            Some(self.cells[self.get_index(pos)])
        } else {
            None
        }
    }

    pub fn set(&mut self, pos: QPos, value: QColor) {
        assert!(
            self.check_bounds(pos.x, pos.y),
            "case ({},{}) hors grille",
            pos.x,
            pos.y
        );
        let index = self.get_index(pos);
        self.cells[index] = value;
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
        assert_eq!(
            grid.get(QPos::new(5, 5, Quarter::Nord)),
            Some(QColor::Empty)
        );
    }

    #[test]
    fn test_set_cell() {
        let mut grid = Grid::new(10, 10);
        grid.set(QPos::new(5, 5, Quarter::Est), QColor::Rouge);
        assert_eq!(grid.get(QPos::new(5, 5, Quarter::Est)), Some(QColor::Rouge));
    }

    #[test]
    #[should_panic(expected = "case (10,10) hors grille")]
    fn test_set_cell_out_of_bounds() {
        let mut grid = Grid::new(10, 10);
        grid.set(QPos::new(10, 10, Quarter::Ouest), QColor::Rouge);
    }

    #[test]
    fn test_get_cell_out_of_bounds() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.get(QPos::new(10, 10, Quarter::Sud)), None);
    }
    #[test]
    fn test_get_index() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.get_index(QPos::new(0, 0, Quarter::Nord)), 0);
    }
}
