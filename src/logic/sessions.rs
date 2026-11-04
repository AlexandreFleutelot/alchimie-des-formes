use super::fission::{MoveError, Movement, move_piece};
use super::grid::{Grid, QPos};
use super::rules::is_won;

#[derive(Debug, PartialEq, Eq)]
pub enum SessionError {
    NothingToUndo,
}

pub struct Session {
    history: Vec<Grid>,
    target: Grid,
}

impl Session {
    pub fn new(initial: Grid, target: Grid) -> Session {
        Session {
            history: vec![initial],
            target,
        }
    }

    pub fn play(&mut self, pos: QPos, mov: Movement) -> Result<(), MoveError> {
        let new_grid = move_piece(self.current(), pos, mov)?;
        self.history.push(new_grid);
        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), SessionError> {
        if self.history.len() > 1 {
            self.history.pop();
            Ok(())
        } else {
            Err(SessionError::NothingToUndo)
        }
    }

    pub fn current(&self) -> &Grid {
        self.history.last().unwrap()
    }
    pub fn target(&self) -> &Grid {
        &self.target
    }

    pub fn is_won(&self) -> bool {
        is_won(self.current(), self.target())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::logic::colors::QColor;
    use crate::logic::grid::Quarter;

    #[test]
    fn test_new_and_current() {
        let mut grid = Grid::new(3, 3);
        grid.set(QPos::new(1, 1, Quarter::Nord), QColor::Bleu);

        let session = Session::new(grid.clone(), grid.clone());
        assert_eq!(session.current(), &grid);
    }

    #[test]
    fn test_undo_initial_state_fails() {
        let grid = Grid::new(3, 3);
        let mut session = Session::new(grid.clone(), grid.clone());

        assert_eq!(session.undo(), Err(SessionError::NothingToUndo));
        assert_eq!(session.current(), &grid);
    }

    #[test]
    fn test_play_valid_move_and_undo() {
        let mut initial_grid = Grid::new(3, 3);
        let start_pos = QPos::new(0, 0, Quarter::Nord);
        let target_pos = QPos::new(1, 0, Quarter::Nord);
        initial_grid.set(start_pos, QColor::Rouge);

        let mut session = Session::new(initial_grid.clone(), initial_grid.clone());

        let res = session.play(start_pos, Movement { dx: 1, dy: 0 });
        assert_eq!(res, Ok(()));
        assert_eq!(session.current().get(start_pos), Some(QColor::Empty));
        assert_eq!(session.current().get(target_pos), Some(QColor::Rouge));

        assert_eq!(session.undo(), Ok(()));
        assert_eq!(session.current(), &initial_grid);

        assert_eq!(session.undo(), Err(SessionError::NothingToUndo));
    }

    #[test]
    fn test_play_invalid_moves_do_not_change_history() {
        let mut grid = Grid::new(3, 3);
        let pos = QPos::new(0, 0, Quarter::Nord);
        grid.set(pos, QColor::Bleu);

        let mut session = Session::new(grid.clone(), grid.clone());

        assert_eq!(
            session.play(QPos::new(1, 1, Quarter::Nord), Movement { dx: 1, dy: 0 }),
            Err(MoveError::NoShapeSelected)
        );

        assert_eq!(
            session.play(QPos::new(5, 5, Quarter::Nord), Movement { dx: 1, dy: 0 }),
            Err(MoveError::SelectionOutOfGrid)
        );

        assert_eq!(
            session.play(pos, Movement { dx: -1, dy: 0 }),
            Err(MoveError::TargetOutOfGrid)
        );

        assert_eq!(session.current(), &grid);
        assert_eq!(session.undo(), Err(SessionError::NothingToUndo));
    }

    #[test]
    pub fn test_is_won() {
        let mut grid1 = Grid::new(3, 3);
        let grid2 = Grid::new(3, 3);
        grid1.set(QPos::new(1, 1, Quarter::Nord), QColor::Rouge);

        let session1 = Session::new(grid1.clone(), grid1.clone());
        assert!(session1.is_won());

        let session2 = Session::new(grid1.clone(), grid2.clone());
        assert!(!session2.is_won());
    }
}
