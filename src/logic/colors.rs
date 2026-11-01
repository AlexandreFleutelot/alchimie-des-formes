

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum Color {
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

impl Color {

  fn get_bits(self) -> u8 {self as u8}

  fn from_bits(bits: u8) -> Color {
    match bits {
      0b000 => Color::Empty,
      0b001 => Color::Rouge,
      0b010 => Color::Jaune,
      0b100 => Color::Bleu,
      0b011 => Color::Orange,
      0b110 => Color::Vert,
      0b101 => Color::Violet,
      0b111 => Color::Brun,
      _ => panic!("Invalid bits for Color: {}", bits),
    }
  }
  
  fn mix(&self, other: &Color) -> Color {
    Color::from_bits(self.get_bits() | other.get_bits())
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bits() {
        assert_eq!(Color::Empty.get_bits(), 0b000);
        assert_eq!(Color::Rouge.get_bits(), 0b001);
        assert_eq!(Color::Jaune.get_bits(), 0b010);
        assert_eq!(Color::Bleu.get_bits(), 0b100);
        assert_eq!(Color::Orange.get_bits(), 0b011);
        assert_eq!(Color::Vert.get_bits(), 0b110);
        assert_eq!(Color::Violet.get_bits(), 0b101);
        assert_eq!(Color::Brun.get_bits(), 0b111);
    }

    #[test]
    fn test_from_bits() {
        assert_eq!(Color::from_bits(0b000), Color::Empty);
        assert_eq!(Color::from_bits(0b001), Color::Rouge);
        assert_eq!(Color::from_bits(0b010), Color::Jaune);
        assert_eq!(Color::from_bits(0b100), Color::Bleu);
        assert_eq!(Color::from_bits(0b011), Color::Orange);
        assert_eq!(Color::from_bits(0b110), Color::Vert);
        assert_eq!(Color::from_bits(0b101), Color::Violet);
        assert_eq!(Color::from_bits(0b111), Color::Brun);
    }

    #[test]
    #[should_panic(expected = "Invalid bits for Color: 42")]
    fn test_from_bits_invalid() {
        Color::from_bits(42);
    }

    #[test]
    fn test_mix() {
        assert_eq!(Color::Empty.mix(&Color::Rouge), Color::Rouge);
        assert_eq!(Color::Rouge.mix(&Color::Jaune), Color::Orange);
        assert_eq!(Color::Bleu.mix(&Color::Rouge), Color::Violet);
        assert_eq!(Color::Jaune.mix(&Color::Bleu), Color::Vert);
        assert_eq!(Color::Jaune.mix(&Color::Violet), Color::Brun);
    }
}
