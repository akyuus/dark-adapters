use bevy::app::App;
use bevy::prelude::{
    in_state, BorderColor, Changed, Color, Commands, Component, DespawnRecursiveExt, Entity,
    EventReader, Input, IntoSystemConfigs, KeyCode, NextState, Plugin, Query, Res, ResMut, Update,
    Window, With,
};
use bevy::window::{MonitorSelection, WindowMode};
use bevy_ui_navigation::prelude::*;

use crate::modes::mode_state::GameModeState;
use crate::modes::pause::pausemode::{OptionsMenuRoot, PauseMenuState};
use crate::utils::utilsystems::{BASE_WINDOW_HEIGHT, BASE_WINDOW_WIDTH};

#[derive(Component)]
struct OptionsMenu;

#[derive(Component)]
pub enum ResolutionOptions {
    Small,
    Medium,
    Large,
    Fullscreen,
}

fn option_button_hover(
    mut interaction_query: Query<(&Focusable, &mut BorderColor), Changed<Focusable>>,
) {
    for (focus, mut border_color) in interaction_query.iter_mut() {
        let new_border: BorderColor = match focus.state() {
            FocusState::Focused => Color::RED.into(),
            _ => Color::BLACK.into(),
        };
        *border_color = new_border;
    }
}

fn handle_option_menu_nav_events(
    mut res_button_query: Query<&ResolutionOptions>,
    mut windows: Query<&mut Window>,
    options_menu_root_query: Query<Entity, With<OptionsMenuRoot>>,
    mut events: EventReader<NavEvent>,
    mut next_state: ResMut<NextState<PauseMenuState>>,
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::X) {
        next_state.set(PauseMenuState::Stationary);
        commands
            .entity(options_menu_root_query.single())
            .despawn_recursive();
        return;
    }

    let mut window = windows.single_mut();
    for res_option in events.nav_iter().activated_in_query(&mut res_button_query) {
        let mut mode = WindowMode::Windowed;
        match res_option {
            ResolutionOptions::Small => {
                window.resolution.set(BASE_WINDOW_WIDTH, BASE_WINDOW_HEIGHT);
            }
            ResolutionOptions::Medium => {
                window
                    .resolution
                    .set(BASE_WINDOW_WIDTH * 2.0, BASE_WINDOW_HEIGHT * 2.0);
            }
            ResolutionOptions::Large => {
                window
                    .resolution
                    .set(BASE_WINDOW_WIDTH * 2.5, BASE_WINDOW_HEIGHT * 2.5);
            }
            ResolutionOptions::Fullscreen => {
                mode = WindowMode::BorderlessFullscreen;
            }
        }
        window.mode = mode;
        window.position.center(MonitorSelection::Current);
    }
}

pub struct OptionsMenuPlugin;

impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((option_button_hover, handle_option_menu_nav_events)
                .run_if(in_state(PauseMenuState::InOptionsMenu)))
            .run_if(in_state(GameModeState::Paused)),
        );
    }
}
