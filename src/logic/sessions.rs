use super::fission::{MoveError, Movement, move_piece};
use super::grid::{Grid, QPos};

#[derive(Debug, PartialEq, Eq)]
pub enum SessionError {
    NothingToUndo,
}

pub struct Session {
    history: Vec<Grid>,
}

impl Session {
    pub fn new(initial: Grid) -> Session {
        Session {
            history: vec![initial],
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

        let session = Session::new(grid.clone());
        assert_eq!(session.current(), &grid);
    }

    #[test]
    fn test_undo_initial_state_fails() {
        let grid = Grid::new(3, 3);
        let mut session = Session::new(grid.clone());

        assert_eq!(session.undo(), Err(SessionError::NothingToUndo));
        assert_eq!(session.current(), &grid);
    }

    #[test]
    fn test_play_valid_move_and_undo() {
        let mut initial_grid = Grid::new(3, 3);
        let start_pos = QPos::new(0, 0, Quarter::Nord);
        let target_pos = QPos::new(1, 0, Quarter::Nord);
        initial_grid.set(start_pos, QColor::Rouge);

        let mut session = Session::new(initial_grid.clone());

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

        let mut session = Session::new(grid.clone());

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
}
