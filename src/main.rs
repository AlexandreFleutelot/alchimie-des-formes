extern crate core;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

mod game;
mod input;
mod logic;
mod render;

use game::GamePlugin;
use input::InputPlugin;
use render::RenderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins((InputPlugin, RenderPlugin, GamePlugin))
        .run();
}
