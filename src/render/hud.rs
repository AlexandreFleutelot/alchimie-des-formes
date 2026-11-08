use crate::logic::levels::{CurrentLevel, HINTS};
use bevy::prelude::*;

#[derive(Component)]
pub enum HudText {
    Level,
    Hint,
}

pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("FormAl-Chemy"),
        TextFont {
            font_size: FontSize::Px(32.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            width: Val::Percent(100.0),
            ..default()
        },
    ));

    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            width: Val::Percent(100.0),
            ..default()
        },
        HudText::Level,
    ));

    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(18.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(120.0),
            width: Val::Percent(100.0),
            ..default()
        },
        HudText::Hint,
    ));

    commands.spawn((
        Text::new("Drag : move\nU : undo\nR : restart level"),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
}

pub fn update_hud_texts(current_level: Res<CurrentLevel>, mut texts: Query<(&mut Text, &HudText)>) {
    for (mut text, kind) in &mut texts {
        *text = match kind {
            HudText::Level => {
                Text::new(format!("Level {}/{}", current_level.index + 1, HINTS.len()))
            }
            HudText::Hint => Text::new(HINTS[current_level.index]),
        };
    }
}
