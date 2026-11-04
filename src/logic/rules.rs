use crate::logic::grid::Grid;

pub fn is_won(test_grid: &Grid, target_grid: &Grid) -> bool {
    test_grid == target_grid
}

#[cfg(test)]
mod test {
    use crate::logic::colors::QColor;
    use crate::logic::grid::{Grid, QPos, Quarter};
    use crate::logic::rules::is_won;

    #[test]
    pub fn test_is_won() {
        let mut grid1 = Grid::new(3, 3);
        let grid2 = Grid::new(3, 3);
        grid1.set(QPos::new(1, 1, Quarter::Nord), QColor::Rouge);

        assert!(is_won(&grid1, &grid1));
        assert!(!is_won(&grid1, &grid2));
    }
}
