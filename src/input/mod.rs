use crate::game::session::GameSession;
use crate::render::layout::current_offset;
use crate::render::shapes::world_to_qpos;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_piece);
    }
}

pub fn select_piece(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    game_session: Res<GameSession>,
) -> Result {
    let (camera, camera_transform) = camera.into_inner();
    if mouse.just_pressed(MouseButton::Left) {
        let Some(position) = window.cursor_position() else {
            return Ok(());
        };
        let world_point = camera.viewport_to_world_2d(camera_transform, position)?;
        println!("Click at {world_point:?}",);
        let offset = current_offset(game_session.session.current().width as f32);
        let qpos = world_to_qpos(game_session.session.current(), world_point, offset);
        println!("Selected piece: {:?}", qpos);
    }
    Ok(())
}


