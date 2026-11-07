use crate::logic::grid::{Grid, QPos, Quarter};
use crate::render::layout::current_offset;
use bevy::math::Vec2;
use bevy::prelude::Triangle2d;

pub const CELL_SIZE: f32 = 50.;

pub fn quarter_triangle(q: Quarter) -> Triangle2d {
    let mid = Vec2::new(CELL_SIZE / 2., CELL_SIZE / 2.);
    match q {
        Quarter::Nord => Triangle2d::new(
            Vec2::new(0., CELL_SIZE),
            Vec2::new(CELL_SIZE, CELL_SIZE),
            mid,
        ),
        Quarter::Est => Triangle2d::new(
            Vec2::new(CELL_SIZE, CELL_SIZE),
            Vec2::new(CELL_SIZE, 0.),
            mid,
        ),
        Quarter::Sud => Triangle2d::new(Vec2::new(CELL_SIZE, 0.), Vec2::new(0., 0.), mid),
        Quarter::Ouest => Triangle2d::new(Vec2::new(0., 0.), Vec2::new(0., CELL_SIZE), mid),
    }
}

pub fn world_to_qpos(grid: &Grid, point: Vec2, offset: Vec2) -> Option<QPos> {
    let grid_start = current_offset(grid.width as f32);
    let grid_end = grid_start + Vec2::new(grid.width as f32, grid.height as f32);

    //Check point in grid
    if point.x > grid_start.x
        && point.x < grid_end.x
        && point.y > grid_start.y
        && point.y < grid_end.y
    {
        let cell_x = point.x - grid_start.x / grid.width as f32;
        let cell_y = point.y - grid_start.y / grid.height as f32;
        let q = Quarter::Nord; //TODO: find the righ quarter
        return Some(QPos::new(cell_x as usize, cell_y as usize, q));
    }
    None
}
