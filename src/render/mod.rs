mod colors;
mod ghost;
mod grid;
mod hud;
mod outline;
pub(crate) mod layout;
mod screens;
pub(crate) mod shapes;

use crate::game;
use crate::game::AppState;
use crate::game::session::GameSession;
use crate::render::grid::{GridVisual, spawn_grid};
use crate::logic::levels::CurrentLevel;
use crate::render::layout::{current_offset, target_offset};
use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        println!("RenderPlugin build");
        app.add_systems(Startup, (setup.after(game::load_session), hud::setup_hud))
            .add_plugins(screens::ScreensPlugin)
            .add_systems(
                Update,
                redraw_grids.run_if(resource_changed::<GameSession>),
            )
            .add_systems(
                Update,
                ghost::draw_ghost.run_if(in_state(AppState::Playing)),
            )
            .add_systems(Update, outline::draw_outlines)
            .add_systems(
                Update,
                hud::update_hud_texts.run_if(resource_changed::<CurrentLevel>),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn redraw_grids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_session: Res<GameSession>,
    visuals: Query<Entity, With<GridVisual>>,
) {
    for e in &visuals {
        commands.entity(e).despawn();
    }
    let current = game_session.session.current();
    spawn_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        current,
        current_offset(current),
        "Your grid",
    );
    let target = game_session.session.target();
    spawn_grid(
        &mut commands,
        &mut meshes,
        &mut materials,
        target,
        target_offset(target),
        "Target",
    );
}
