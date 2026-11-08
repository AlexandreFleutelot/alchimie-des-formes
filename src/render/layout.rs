use crate::logic::grid::Grid;
use crate::render::shapes::CELL_SIZE;
use bevy::math::Vec2;

pub fn current_offset(grid: &Grid) -> Vec2 {
    let width = grid.width as f32 * CELL_SIZE;
    let height = grid.height as f32 * CELL_SIZE;
    Vec2::new(-width - 50., height / 2.)
}

pub fn target_offset(grid: &Grid) -> Vec2 {
    let height = grid.height as f32 * CELL_SIZE;
    Vec2::new(50., height / 2.)
}
