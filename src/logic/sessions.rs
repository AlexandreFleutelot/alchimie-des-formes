use super::fission::{MoveError, Movement, move_piece};
use super::grid::{Grid, QPos};

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

    pub fn undo(&mut self) -> bool {
        if self.history.len() > 1 {
            self.history.pop();
            true
        } else {
            false
        }
    }

    pub fn current(&self) -> &Grid {
        self.history.last().unwrap()
    }
}
