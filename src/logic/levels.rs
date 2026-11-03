use crate::logic::colors::QColor;
use crate::logic::grid::{Grid, QPos, Quarter};

#[derive(Debug, PartialEq)]
enum ParseError {
    UnknownChar,
    InvalidFormat
}

const QUARTERS: [Quarter; 4] = [Quarter::Nord,Quarter::Est,Quarter::Sud,Quarter::Ouest];

struct Level {
    grid:Grid,
    target:Grid
}

fn char_to_color(c:char) -> Option<QColor> {
    match c {
        'R' => Some(QColor::Rouge),
        'J' => Some(QColor::Jaune),
        'B' => Some(QColor::Bleu),
        'O' => Some(QColor::Orange),
        'V' => Some(QColor::Vert),
        'I' => Some(QColor::Violet),
        'U' => Some(QColor::Brun),
        '.' => Some(QColor::Empty),
        _ => None
    }
}

impl Level {

    /// TODO: Creer read_line(...) pour simplifer ?
    pub fn load_from_text(level_text: &str) -> Result<Grid, ParseError> {

        let level_text = level_text.trim();

        let height = level_text.lines().count();
        let width = level_text.lines().next().map_or(0, |l| l.split_whitespace().count());

        let mut grid = Grid::new(width,height);

        for (y, ligne) in level_text.lines().enumerate() {

            if ligne.split_whitespace().count() > width { return Err(ParseError::InvalidFormat) };
            for (x, cell) in ligne.split_whitespace().enumerate() {

                if cell.chars().count() != 4 {  return Err(ParseError::InvalidFormat) };
                for (i, c) in cell.chars().enumerate() {
                    let color = char_to_color(c).ok_or(ParseError::UnknownChar)?;
                    let q: Quarter = *QUARTERS.get(i).ok_or(ParseError::InvalidFormat)?;
                    grid.set(QPos::new(x, y, q), color);
                }
            }
        }
        Ok(grid)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_load_from_text() {
        todo!()
    }
}