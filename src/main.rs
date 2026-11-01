use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

mod logic;
mod render;
mod input;
mod game;

use render::RenderPlugin;
use input::InputPlugin;
use game::GamePlugin;

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