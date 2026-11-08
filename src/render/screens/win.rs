use crate::game::AppState;
use crate::logic::levels::{CurrentLevel, LEVELS};
use bevy::prelude::*;

pub fn show_win_screen(mut commands: Commands, current_level: Res<CurrentLevel>) {
    let message = if current_level.index + 1 == LEVELS.len() {
        "You finished the game!\nPress Space to play again"
    } else {
        "Level complete!\nPress Space for the next level"
    };

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0., 0., 0., 0.6)),
        DespawnOnExit(AppState::Won),
        children![(
            Text::new(message),
            TextFont {
                font_size: FontSize::Px(48.0),
                ..default()
            },
            TextLayout::justify(Justify::Center),
        )],
    ));
}
