use crate::game::session::GameSession;
use crate::render::layout::{current_offset, target_offset};
use crate::render::shapes::CELL_SIZE;
use bevy::prelude::*;

const GRID_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

pub fn draw_outlines(mut gizmos: Gizmos, game_session: Res<GameSession>) {
    let current = game_session.session.current();
    let target = game_session.session.target();
    draw_grid_lines(
        &mut gizmos,
        current.width,
        current.height,
        current_offset(current.width as f32),
    );
    draw_grid_lines(&mut gizmos, target.width, target.height, target_offset());
}

fn draw_grid_lines(gizmos: &mut Gizmos, width: usize, height: usize, offset: Vec2) {
    let total_width = width as f32 * CELL_SIZE;
    let total_height = height as f32 * CELL_SIZE;

    for i in 0..=width {
        let x = offset.x + i as f32 * CELL_SIZE;
        gizmos.line_2d(
            Vec2::new(x, offset.y),
            Vec2::new(x, offset.y - total_height),
            GRID_COLOR,
        );
    }
    for j in 0..=height {
        let y = offset.y - j as f32 * CELL_SIZE;
        gizmos.line_2d(
            Vec2::new(offset.x, y),
            Vec2::new(offset.x + total_width, y),
            GRID_COLOR,
        );
    }
}
