mod colors;
mod ghost;
mod grid;
pub(crate) mod layout;
pub(crate) mod shapes;

use crate::game;
use crate::game::AppState;
use crate::game::session::GameSession;
use crate::render::grid::{CurrentVisual, TargetVisual, spawn_grid};
use crate::render::layout::{current_offset, target_offset};
use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        println!("RenderPlugin build");
        app.add_systems(Startup, setup.after(game::load_session))
            .add_systems(
                Update,
                redraw_current.run_if(resource_changed::<GameSession>),
            )
            .add_systems(
                Update,
                ghost::draw_ghost.run_if(in_state(AppState::Playing)),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn redraw_current(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_session: Res<GameSession>,
    visuals: Query<Entity, Or<(With<CurrentVisual>, With<TargetVisual>)>>,
) {
    for e in &visuals {
        commands.entity(e).despawn();
    }
    spawn_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        game_session.session.target(),
        target_offset(),
        false,
    );
    let current = game_session.session.current();
    spawn_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        current,
        current_offset(current.width as f32),
        true,
    );
}
