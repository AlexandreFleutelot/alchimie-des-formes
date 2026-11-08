use crate::game::session::GameSession;
use crate::input::{Drag, cursor_to_world};
use crate::logic::fission::{Movement, move_piece};
use crate::render::layout::current_offset;
use crate::render::shapes::{CELL_SIZE, quarter_triangle, world_to_qpos};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct GhostVisual;

pub fn draw_ghost(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_session: Res<GameSession>,
    drag: Res<Drag>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    ghosts: Query<Entity, With<GhostVisual>>,
) {
    for e in &ghosts {
        commands.entity(e).despawn();
    }

    let Some(from) = drag.from else {
        return;
    };
    let (camera, camera_transform) = camera.into_inner();
    let Some(point) = cursor_to_world(*window, camera, camera_transform) else {
        return;
    };
    let grid = game_session.session.current();
    let offset = current_offset(grid.width as f32);
    let Some(to) = world_to_qpos(grid, point, offset) else {
        return;
    };

    let m = Movement {
        dx: to.x as i32 - from.x as i32,
        dy: to.y as i32 - from.y as i32,
    };
    let color = if move_piece(grid, from, m).is_ok() {
        Color::srgba(1., 1., 1., 0.5)
    } else {
        Color::srgba(1., 0., 0., 0.5)
    };
    let dx = m.dx as f32;
    let dy = m.dy as f32;

    for p in grid.get_piece(from) {
        commands.spawn((
            Mesh2d(meshes.add(quarter_triangle(p.q))),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                offset.x + CELL_SIZE * (p.x as f32 + dx),
                offset.y - CELL_SIZE * (p.y as f32 + dy + 1.),
                1.,
            ),
            GhostVisual,
        ));
    }
}
