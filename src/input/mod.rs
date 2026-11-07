use crate::game::session::GameSession;
use crate::logic::colors::QColor;
use crate::logic::grid::QPos;
use crate::render::layout::current_offset;
use crate::render::shapes::world_to_qpos;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct Drag {
    from: Option<QPos>,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Drag>().add_systems(Update, drag_piece);
    }
}

pub fn drag_piece(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    game_session: Res<GameSession>,
    mut drag: ResMut<Drag>,
) -> Result {
    let (camera, camera_transform) = camera.into_inner();
    if mouse.just_pressed(MouseButton::Left) {
        let Some(world_point) = cursor_to_world(*window, camera, camera_transform) else {
            return Ok(());
        };
        let offset = current_offset(game_session.session.current().width as f32);
        let Some(qpos) = world_to_qpos(game_session.session.current(), world_point, offset) else {
            return Ok(());
        };
        let piece_color = game_session.session.current().get(qpos);
        match piece_color {
            Some(piece_color) => {
                if piece_color != QColor::Empty {
                    println!("Selected piece: {:?} added to drag", qpos);
                    drag.from = Some(qpos);
                }
            }
            None => {}
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
