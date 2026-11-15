use super::screens::menu::hover_color;
use crate::game::AppState;
use crate::game::session::GameSession;
use crate::input::Drag;
use crate::logic::levels::{CurrentLevel, HINTS};
use bevy::prelude::*;
use bevy::ui_widgets::Button as UiButton;

#[derive(Component, Clone, Default)]
pub enum HudText {
    #[default]
    Level,
    Hint,
}

pub fn setup_hud(mut commands: Commands) {
    let despawn_header = DespawnOnExit(AppState::InGame);

    commands.spawn_scene(bsn! {
        #HudTopBar
        despawn_header
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(20.0), Val::Px(12.0))
        }
        BackgroundColor(Color::srgba(0.08, 0.09, 0.14, 0.85))
        Children [
            #Title
            Node {
                flex_grow: 1.0,
                flex_basis: Val::Px(0.0),
                justify_content: JustifyContent::FlexStart
            }
            Children [
                #TitleText
                Text("FormAl-Chemy")
                TextFont { font_size: FontSize::Px(32.0) }
            ]
            --
            #LevelText
            Node {
                flex_grow: 1.0,
                flex_basis: Val::Px(0.0),
                justify_content: JustifyContent::Center
            }
            Children [
                #LevelTextContent
                HudText::Level
                Text("")
                TextFont { font_size: FontSize::Px(24.0) }
            ]
            --
            #TopButtons
            Node {
                flex_grow: 1.0,
                flex_basis: Val::Px(0.0),
                justify_content: JustifyContent::FlexEnd,
                column_gap: Val::Px(10.0),
            }
            Children [
                #UndoButton
                @hud_button("Undo", Color::srgb(0.2, 0.7, 0.2))
                on(|_: On<PointerClick>, mut game_session: ResMut<GameSession>, mut drag: ResMut<Drag>| {
                    let _ = game_session.session.undo();
                    drag.from = None;
                })
                --
                #RestartButton
                @hud_button("Restart", Color::srgb(0.2, 0.2, 0.7))
                on(|_:On<PointerClick>, mut game_session: ResMut<GameSession>, current_level: Res<CurrentLevel>, mut drag: ResMut<Drag>| {
                    *game_session = GameSession::new_from_level(current_level.index);
                    drag.from = None;
                })
                --
                #QuitButton
                @hud_button("Quit", Color::srgb(0.7, 0.2, 0.2))
                on(|_: On<PointerClick>, mut next_state: ResMut<NextState<AppState>>| {
                    next_state.set(AppState::Menu);
                })
            ]

        ]
    });

    let despawn_hint = DespawnOnExit(AppState::InGame);
    commands.spawn_scene(bsn! {
        #HintText
        despawn_hint
        HudText::Hint
        Text("")
        TextFont { font_size: FontSize::Px(18.0) }
        TextLayout::justify(Justify::Center)
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(120.0),
            width: Val::Percent(100.0),
        }
    });

    let despawn_controls = DespawnOnExit(AppState::InGame);
    commands.spawn_scene(bsn! {
        #ControlsText
        despawn_controls
        Text("Drag : move  |  U : undo  |  R : restart  |  N : next  |  Esc : menu")
        TextFont { font_size: FontSize::Px(14.0) }
        TextColor(Color::srgb(0.6, 0.64, 0.72))
        TextLayout::justify(Justify::Center)
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            width: Val::Percent(100.0),
        }
    });
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

fn hud_button(label: impl Into<String>, color: Color) -> impl Scene {
    let label = label.into();
    bsn! {
        UiButton
        Node {
            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
        }
        BackgroundColor(color)
        @hover_color(color)
        Children [
            Text(label)
            TextFont { font_size: FontSize::Px(18.0) }
        ]
    }
}
