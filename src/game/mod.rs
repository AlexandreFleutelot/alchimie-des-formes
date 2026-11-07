pub mod session;

pub(crate) use crate::game::session::load_session;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        println!("GamePlugin build");
        app.add_systems(Startup, load_session);
    }
}
