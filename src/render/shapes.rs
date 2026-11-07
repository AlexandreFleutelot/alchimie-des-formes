use crate::logic::grid::Quarter;
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
