use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
      println!("RenderPlugin build");
      app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(
      (Sprite::from_color(Color::srgb(0.2, 0.8, 0.3), 
      Vec2::splat(40.0)), 
      Transform::from_xyz(50.0, 30.0, 0.0)));
}