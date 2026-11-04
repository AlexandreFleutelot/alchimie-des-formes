use crate::logic::colors::QColor;
use crate::logic::grid::{Grid, QPos, Quarter};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownChar,
    InvalidFormat,
}

const QUARTERS: [Quarter; 4] = [Quarter::Nord, Quarter::Est, Quarter::Sud, Quarter::Ouest];

struct Level {
    start: Grid,
    target: Grid,
}

fn char_to_color(c: char) -> Option<QColor> {
    match c {
        'R' => Some(QColor::Rouge),
        'J' => Some(QColor::Jaune),
        'B' => Some(QColor::Bleu),
        'O' => Some(QColor::Orange),
        'V' => Some(QColor::Vert),
        'I' => Some(QColor::Violet),
        'U' => Some(QColor::Brun),
        '.' => Some(QColor::Empty),
        _ => None,
    }
}

pub fn parse_cell(cell: &str) -> Result<[QColor; 4], ParseError> {
    if cell.chars().count() != 4 {
        return Err(ParseError::InvalidFormat);
    };
    let mut parsed_cell = [QColor::Empty; 4];
    for (i, c) in cell.chars().enumerate() {
        parsed_cell[i] = char_to_color(c).ok_or(ParseError::UnknownChar)?;
    }
    Ok(parsed_cell)
}

pub fn parse_grid(text_grid: &str) -> Result<Grid, ParseError> {
    let level_text = text_grid.trim();

    let height = level_text.lines().count();
    let width = level_text
        .lines()
        .next()
        .map_or(0, |l| l.split_whitespace().count());
    let mut grid = Grid::new(width, height);

    for (y, ligne) in level_text.lines().enumerate() {
        if ligne.split_whitespace().count() != width {
            return Err(ParseError::InvalidFormat);
        };
        for (x, cell) in ligne.split_whitespace().enumerate() {
            let colors = parse_cell(cell)?;
            for (q, c) in QUARTERS.iter().zip(colors) {
                grid.set(QPos::new(x, y, *q), c);
            }
        }
    }
    Ok(grid)
}



impl Level {
        pub fn new(start_grid:Grid, target_grid:Grid) -> Level {
            Level {
                start: start_grid,
                target: target_grid
            }
        }
    pub fn from_text(start_text:&str, target_text:&str) -> Result<Level, ParseError> {

        let start_grid = parse_grid(start_text)?;
        let target_grid = parse_grid(target_text)?;
        Ok(Level {
            start: start_grid,
            target: target_grid
        })
    }
}

#[cfg(test)]
mod test {
    use crate::logic::colors::QColor;
    use crate::logic::grid::{QPos, Quarter};
    use crate::logic::levels::{ParseError, ParseError::*, parse_cell, parse_grid};

    #[test]
    fn test_parse_cell() {
        assert_eq!(
            parse_cell("R..B"),
            Ok([QColor::Rouge, QColor::Empty, QColor::Empty, QColor::Bleu])
        );
        assert_eq!(parse_cell("R.B"), Err(InvalidFormat));
        assert_eq!(parse_cell("R.ZB"), Err(UnknownChar));
    }

    #[test]
    fn test_parse_grid() {
        let grid = parse_grid("R..B R..B\nR..B R..B").unwrap();
        assert_eq!(
            &grid.get(QPos::new(1, 1, Quarter::Nord)),
            &Some(QColor::Rouge)
        );
        assert_eq!(
            &grid.get(QPos::new(0, 1, Quarter::Ouest)),
            &Some(QColor::Bleu)
        );
        assert_eq!(
            &grid.get(QPos::new(0, 0, Quarter::Est)),
            &Some(QColor::Empty)
        );
    }

    #[test]
    fn test_parse_grid_wrong_width() {
        assert_eq!(
            parse_grid("R..B R..B\nR..B R..B JJJJ"),
            Err(ParseError::InvalidFormat)
        );
    }
}
