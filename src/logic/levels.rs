use crate::logic::colors::QColor;
use crate::logic::grid::{Grid, QPos, Quarter};
use bevy::prelude::*;

pub const LEVEL_01: &str = include_str!("../../assets/levels/level_01.txt");
pub const LEVEL_02: &str = include_str!("../../assets/levels/level_02.txt");

pub const LEVEL_03: &str = include_str!("../../assets/levels/level_03.txt");

pub const LEVEL_04: &str = include_str!("../../assets/levels/level_04.txt");

pub const LEVEL_05: &str = include_str!("../../assets/levels/level_05.txt");

pub const LEVELS: [&str; 5] = [LEVEL_01, LEVEL_02, LEVEL_03, LEVEL_04, LEVEL_05];

pub const HINTS: [&str; LEVELS.len()] = [
    "Drag a piece onto another to mix their colors",
    "Position matters: drop each piece where the target needs it",
    "A triangle keeps its orientation when you move it",
    "Pieces that touch stick together, mind the order! Red over blue makes violet",
    "Yellow over red makes orange. Pieces that touch stick together",
];

#[derive(Resource, Default)]
pub struct CurrentLevel {
    pub index: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownChar,
    InvalidFormat,
}

pub struct Level {
    pub start: Grid,
    pub target: Grid,
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
            for (q, c) in Quarter::ALL.iter().zip(colors) {
                grid.set(QPos::new(x, y, *q), c);
            }
        }
    }
    Ok(grid)
}

impl Level {
    pub fn new(start_grid: Grid, target_grid: Grid) -> Level {
        Level {
            start: start_grid,
            target: target_grid,
        }
    }
    pub fn from_text(start_text: &str, target_text: &str) -> Result<Level, ParseError> {
        let start_grid = parse_grid(start_text)?;
        let target_grid = parse_grid(target_text)?;
        Ok(Level {
            start: start_grid,
            target: target_grid,
        })
    }

    pub fn from_file_text(text: &str) -> Result<Level, ParseError> {
        let mut parts = text.split("---");
        let start = parts.next().ok_or(ParseError::InvalidFormat)?;
        let target = parts.next().ok_or(ParseError::InvalidFormat)?;
        Self::from_text(start, target)
    }
}

#[cfg(test)]
mod test {
    use crate::logic::colors::QColor;
    use crate::logic::grid::{QPos, Quarter};
    use crate::logic::levels::{Level, ParseError, ParseError::*, parse_cell, parse_grid};
    use crate::logic::rules::is_won;

    const NIVEAU_TEST: &str = "RRRR ....\n.... BBBJ\n---\nRRRR ....\n.... BBBJ";

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

    #[test]
    fn test_from_file_text() {
        let lvl = Level::from_file_text(NIVEAU_TEST).unwrap();

        assert_eq!(
            &lvl.start.get(QPos::new(0, 0, Quarter::Est)),
            &Some(QColor::Rouge)
        );

        assert_eq!(
            &lvl.start.get(QPos::new(0, 1, Quarter::Est)),
            &Some(QColor::Empty)
        );

        assert_eq!(
            &lvl.target.get(QPos::new(0, 0, Quarter::Est)),
            &Some(QColor::Rouge)
        );

        assert_eq!(
            &lvl.target.get(QPos::new(0, 1, Quarter::Est)),
            &Some(QColor::Empty)
        );

        assert!(is_won(&lvl.start, &lvl.target));
    }
}
