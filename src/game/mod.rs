pub mod session;

pub(crate) use crate::game::session::load_session;
use crate::logic::levels::CurrentLevel;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        println!("GamePlugin build");
        app.init_resource::<CurrentLevel>()
            .add_systems(Startup, load_session);
    }
}
