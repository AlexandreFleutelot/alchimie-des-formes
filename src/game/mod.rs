pub mod session;

pub(crate) use crate::game::session::load_session;
use crate::logic::levels::CurrentLevel;
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Playing,
    Won,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        println!("GamePlugin build");
        app.init_state::<AppState>()
            .init_resource::<CurrentLevel>()
            .add_systems(Startup, load_session)
            .add_systems(OnEnter(AppState::Won), || println!("Won"));
    }
}
