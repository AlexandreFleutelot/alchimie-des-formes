use crate::logic::colors::QColor;
use bevy::color::Color;

pub fn to_color(c: QColor) -> Option<Color> {
    match c {
        QColor::Empty => None,
        QColor::Rouge => Some(Color::srgb(1., 0., 0.)),
        QColor::Jaune => Some(Color::srgb(1., 1., 0.)),
        QColor::Bleu => Some(Color::srgb(0., 0., 1.)),
        QColor::Orange => Some(Color::srgb(1., 0.5, 0.)),
        QColor::Vert => Some(Color::srgb(0., 1., 0.)),
        QColor::Violet => Some(Color::srgb(1., 0., 1.)),
        QColor::Brun => Some(Color::srgb(0.45, 0.3, 0.15)),
    }
}
