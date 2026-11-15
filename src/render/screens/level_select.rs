use bevy::prelude::*;
use bevy::ui_widgets::Button as UiButton;

use crate::{
    game::AppState,
    logic::levels::{CurrentLevel, LEVELS},
    render::screens::menu::{hover_color, menu_button},
};

pub fn show_level_select_screen(mut commands: Commands) {
    let level_buttons: Vec<_> = (0..LEVELS.len())
        .map(|level_index| level_button(level_index))
        .collect();
    let despawn = DespawnOnExit(AppState::LevelSelect);

    commands.spawn_scene(bsn! {
        #LevelSelect
        despawn
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(32.0),
        }
        Children [
            #Title
            Text("Level Select")
            TextFont { font_size: FontSize::Px(44.0) }
            --
            #LevelGrid
            Node {
              display: Display::Grid,
              grid_template_columns: RepeatedGridTrack::px::<Vec<_>>(3, 80.0),
                row_gap: Val::Px(16.0),
                column_gap: Val::Px(16.0),
            }
            Children [
                {level_buttons}
            ]
            --
            #BackButton
            @menu_button("Back", Color::srgb(0.7, 0.2, 0.2))
            on(|_: On<PointerClick>, mut next_state: ResMut<NextState<AppState>>| {
                next_state.set(AppState::Menu);
            })
        ]
    });
}

fn level_button(level_index: usize) -> impl Scene {
    let label = format!("{}", level_index + 1);
    bsn! {
        UiButton
        Node {
              width: Val::Px(80.0),
              height: Val::Px(80.0),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
          }
          BackgroundColor(Color::srgb(0.2, 0.45, 0.8))
          @hover_color(Color::srgb(0.2, 0.45, 0.8))
          on(move |_: On<PointerClick>, mut current_level:ResMut<CurrentLevel>, mut next_state:ResMut<NextState<AppState>>| {
              current_level.index = level_index;
              next_state.set(AppState::InGame);
          })

          Children [
              Text(label)
              TextFont { font_size: FontSize::Px(32.0) }
          ]
    }
}
