use crate::game::AppState;
use bevy::prelude::*;

pub fn show_win_text(mut commands: Commands) {
    commands.spawn((
        Text::new("Bravo ! Space to continue"),
        TextFont {
            font_size: FontSize::Px(40.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        DespawnOnExit(AppState::Won),
    ));
}
