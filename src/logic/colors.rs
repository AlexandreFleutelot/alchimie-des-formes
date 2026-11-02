#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum QColor {
    #[default]
    Empty = 0b000,
    Rouge = 0b001,
    Jaune = 0b010,
    Bleu = 0b100,
    Orange = 0b011,
    Vert = 0b110,
    Violet = 0b101,
    Brun = 0b111,
}

impl QColor {
    fn get_bits(self) -> u8 {
        self as u8
    }

    fn from_bits(bits: u8) -> QColor {
        match bits {
            0b000 => QColor::Empty,
            0b001 => QColor::Rouge,
            0b010 => QColor::Jaune,
            0b100 => QColor::Bleu,
            0b011 => QColor::Orange,
            0b110 => QColor::Vert,
            0b101 => QColor::Violet,
            0b111 => QColor::Brun,
            _ => panic!("Invalid bits for Color: {}", bits),
        }
    }

    pub fn mix(&self, other: &QColor) -> QColor {
        QColor::from_bits(self.get_bits() | other.get_bits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bits() {
        assert_eq!(QColor::Empty.get_bits(), 0b000);
        assert_eq!(QColor::Rouge.get_bits(), 0b001);
        assert_eq!(QColor::Jaune.get_bits(), 0b010);
        assert_eq!(QColor::Bleu.get_bits(), 0b100);
        assert_eq!(QColor::Orange.get_bits(), 0b011);
        assert_eq!(QColor::Vert.get_bits(), 0b110);
        assert_eq!(QColor::Violet.get_bits(), 0b101);
        assert_eq!(QColor::Brun.get_bits(), 0b111);
    }

    #[test]
    fn test_from_bits() {
        assert_eq!(QColor::from_bits(0b000), QColor::Empty);
        assert_eq!(QColor::from_bits(0b001), QColor::Rouge);
        assert_eq!(QColor::from_bits(0b010), QColor::Jaune);
        assert_eq!(QColor::from_bits(0b100), QColor::Bleu);
        assert_eq!(QColor::from_bits(0b011), QColor::Orange);
        assert_eq!(QColor::from_bits(0b110), QColor::Vert);
        assert_eq!(QColor::from_bits(0b101), QColor::Violet);
        assert_eq!(QColor::from_bits(0b111), QColor::Brun);
    }

    #[test]
    #[should_panic(expected = "Invalid bits for Color: 42")]
    fn test_from_bits_invalid() {
        QColor::from_bits(42);
    }

    #[test]
    fn test_mix() {
        assert_eq!(QColor::Empty.mix(&QColor::Rouge), QColor::Rouge);
        assert_eq!(QColor::Rouge.mix(&QColor::Jaune), QColor::Orange);
        assert_eq!(QColor::Bleu.mix(&QColor::Rouge), QColor::Violet);
        assert_eq!(QColor::Jaune.mix(&QColor::Bleu), QColor::Vert);
        assert_eq!(QColor::Jaune.mix(&QColor::Violet), QColor::Brun);
    }
}
