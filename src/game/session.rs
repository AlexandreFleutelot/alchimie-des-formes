use crate::logic::levels::{CurrentLevel, LEVELS, Level};
use crate::logic::sessions::Session;
use bevy::prelude::{Commands, Res, Resource};

#[derive(Resource)]
pub struct GameSession {
    pub session: Session,
}

impl GameSession {
    pub fn new_from_level(level: usize) -> Self {
        let lvl = Level::from_file_text(LEVELS[level]).expect("Can't load level");
        GameSession {
            session: Session::new(lvl.start, lvl.target),
        }
    }
}

pub fn load_session(mut commands: Commands, current_level: Res<CurrentLevel>) {
    commands.insert_resource(GameSession::new_from_level(current_level.index));
}
