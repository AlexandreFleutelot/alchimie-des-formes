use crate::render::shapes::CELL_SIZE;
use bevy::math::Vec2;

pub fn current_offset(width_cells: f32) -> Vec2 {
    let width = width_cells * CELL_SIZE;
    Vec2::new(-width - 50., 50.)
}

pub fn target_offset() -> Vec2 {
    Vec2::new(50., 50.)
}
