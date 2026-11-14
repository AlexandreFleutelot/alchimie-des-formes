use crate::game::AppState;
use bevy::prelude::*;
use bevy::ui_widgets::Button as UiButton;

pub fn show_menu_screen(mut commands: Commands) {
    let despawn = DespawnOnExit(AppState::Menu);

    commands.spawn_scene(bsn! {
        #MainMenu
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
            Text("FormAl-Chemy")
            TextFont { font_size: FontSize::Px(56.0) }
            --
            #PlayButton
            UiButton
            Node {
                padding: UiRect::axes(Val::Px(36.0), Val::Px(16.0)),
            }
            BackgroundColor(Color::srgb(0.2, 0.45, 0.8))
            on(|_: On<PointerClick>, mut next_state: ResMut<NextState<AppState>>| {
                next_state.set(AppState::InGame);
            })
            Children [
                Text("Jouer")
                TextFont { font_size: FontSize::Px(28.0) }
            ]
        ]
    });
}
