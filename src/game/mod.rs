pub mod session;

pub(crate) use crate::game::session::load_session;
use crate::logic::levels::CurrentLevel;
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(AppState = AppState::InGame)]
pub enum GamePhase {
    #[default]
    Playing,
    Won,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        println!("GamePlugin build");
        app.init_state::<AppState>()
            .add_sub_state::<GamePhase>()
            .init_resource::<CurrentLevel>()
            .add_systems(OnEnter(AppState::InGame), load_session)
            .add_systems(OnEnter(GamePhase::Won), || println!("Won"));
    }
}
