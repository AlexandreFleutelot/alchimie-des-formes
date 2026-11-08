use crate::logic::levels::{LEVEL_01, Level};
use crate::logic::sessions::Session;
use bevy::prelude::{Commands, Resource};

#[derive(Resource)]
pub struct GameSession {
    pub session: Session,
}

impl GameSession {

    pub fn new_from_level() -> Self {
        let lvl = Level::from_file_text(LEVEL_01).expect("Can't to load level 01");
        GameSession {
            session: Session::new(lvl.start, lvl.target),
        }
    }
}

pub fn load_session(mut commands: Commands) {
    commands.insert_resource(GameSession::new_from_level());
}
