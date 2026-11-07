use crate::logic::levels::{LEVEL_01, Level};
use crate::logic::sessions::Session;
use bevy::prelude::*;

pub struct GamePlugin;

#[derive(Resource)]
pub struct GameSession {
    pub session: Session,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        println!("GamePlugin build");
        app.add_systems(Startup, load_session);
    }
}

pub(crate) fn load_session(mut commands: Commands) {
    let lvl = Level::from_file_text(LEVEL_01).expect("Can't to load level 01");
    commands.insert_resource(GameSession {
        session: Session::new(lvl.start, lvl.target),
    });
}
