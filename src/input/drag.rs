use super::Drag;
use crate::game::AppState;
use crate::game::session::GameSession;
use crate::logic::colors::QColor;
use crate::logic::fission::Movement;
use crate::render::layout::current_offset;
use crate::render::shapes::world_to_qpos;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn drag_piece(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut game_session: ResMut<GameSession>,
    mut drag: ResMut<Drag>,
    mut next_state: ResMut<NextState<AppState>>,
) -> Result {
    let (camera, camera_transform) = camera.into_inner();
    let offset = current_offset(game_session.session.current());

    if mouse.just_pressed(MouseButton::Left) {
        let Some(world_point) = cursor_to_world(*window, camera, camera_transform) else {
            return Ok(());
        };
        let Some(qpos) = world_to_qpos(game_session.session.current(), world_point, offset) else {
            return Ok(());
        };
        let piece_color = game_session.session.current().get(qpos);

        if piece_color.is_some_and(|c| c != QColor::Empty) {
            println!("Selected piece: {:?} added to drag", qpos);
            drag.from = Some(qpos);
        }
    }

    if mouse.just_released(MouseButton::Left) {
        let Some(from) = drag.from.take() else {
            return Ok(());
        };
        let Some(world_point) = cursor_to_world(*window, camera, camera_transform) else {
            return Ok(());
        };
        let Some(to) = world_to_qpos(game_session.session.current(), world_point, offset) else {
            return Ok(());
        };
        let dx = to.x as i32 - from.x as i32;
        let dy = to.y as i32 - from.y as i32;

        if dx == 0 && dy == 0 {
            return Ok(());
        }; // simple click

        let m = Movement { dx, dy };
        println!("{:?} -> {:?}", from, m);
        if let Err(e) = game_session.session.play(from, m) {
            println!("Move error: {:?}", e)
        };

        if game_session.session.is_won() {
            next_state.set(AppState::Won);
        }
    }
    Ok(())
}

pub fn cursor_to_world(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let position = window.cursor_position()?;
    camera.viewport_to_world_2d(camera_transform, position).ok()
}
