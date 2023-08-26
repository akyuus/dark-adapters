use bevy::app::{App, AppExit, PluginGroupBuilder};
use bevy::asset::AssetServer;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::Input;
use bevy::prelude::{
    default, in_state, AlignItems, BuildChildren, ButtonBundle, Camera, Camera2d, Camera2dBundle,
    Changed, Color, Commands, Component, EventReader, EventWriter, ImageBundle, IntoSystemConfigs,
    JustifyContent, KeyCode, NextState, NodeBundle, OnEnter, OnExit, Plugin, PluginGroup, Query,
    Res, ResMut, Resource, State, TextBundle, TextStyle, UiImage, Update, With,
};
use bevy::ui::{BackgroundColor, BorderColor, FlexDirection, Style, UiRect, Val};
use bevy_ui_navigation::components::FocusableButtonBundle;
use bevy_ui_navigation::events::NavEvent;
use bevy_ui_navigation::prelude::{FocusState, Focusable, NavEventReaderExt};
use bevy_ui_navigation::systems::InputMapping;
use bevy_ui_navigation::NavRequestSystem;

use crate::modes::dungeon::dungeonmode::DungeonAssets;
use crate::modes::dungeon::dungeonplayer::{DungeonPlayer, MovementState};
use crate::modes::mode_state::{cleanup_system, GameModeState};

#[derive(Resource, Default)]
struct PreviousState(GameModeState);

#[derive(Component)]
struct PauseModeEntity;

#[derive(Component)]
enum PauseMenuButton {
    Resume,
    Exit,
}

struct PauseMode;

const PAUSE_MODE_ALPHA: f32 = 0.7;

impl PauseMode {
    fn check_pause(
        keyboard_input: Res<Input<KeyCode>>,
        mut previous_state: ResMut<PreviousState>,
        current_state: Res<State<GameModeState>>,
        mut next_state: ResMut<NextState<GameModeState>>,
        player_query: Query<&MovementState, With<DungeonPlayer>>,
    ) {
        // don't allow pausing when moving
        if let Ok(&movement_state) = player_query.get_single() {
            if !(movement_state == MovementState::Stationary) {
                return;
            }
        }

        if keyboard_input.just_pressed(KeyCode::Escape) {
            previous_state.0 = **current_state;
            next_state.set(GameModeState::Paused);
        }
    }

    fn update_button_borders(
        keyboard_input: Res<Input<KeyCode>>,
        previous_state: Res<PreviousState>,
        mut next_state: ResMut<NextState<GameModeState>>,
        mut button_query: Query<(&Focusable, &mut BorderColor), Changed<Focusable>>,
    ) {
        for (focusable, mut border_color) in button_query.iter_mut() {
            match focusable.state() {
                FocusState::Focused => border_color.0 = Color::RED,
                _ => border_color.0 = Color::PURPLE,
            }
        }
        if keyboard_input.just_pressed(KeyCode::Escape) {
            next_state.set(previous_state.0);
        }
    }

    fn button_activation_system(
        mut button_query: Query<&mut PauseMenuButton>,
        mut events: EventReader<NavEvent>,
        previous_state: Res<PreviousState>,
        mut next_state: ResMut<NextState<GameModeState>>,
        mut exit: EventWriter<AppExit>,
    ) {
        events.nav_iter().activated_in_query_foreach_mut(
            &mut button_query,
            |button| match *button {
                PauseMenuButton::Resume => {
                    next_state.set(previous_state.0);
                }
                PauseMenuButton::Exit => exit.send(AppExit),
            },
        )
    }

    fn darken_screen_and_show_menu(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut input_mapping: ResMut<InputMapping>,
        dungeon_assets: Res<DungeonAssets>,
    ) {
        input_mapping.keyboard_navigation = true;
        input_mapping.key_action = KeyCode::Z;
        input_mapping.key_free = KeyCode::F24;
        input_mapping.focus_follows_mouse = false;

        // region camera
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    order: 2,
                    ..default()
                },
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::None,
                },
                ..default()
            },
            PauseModeEntity,
        ));
        //endregion

        let black_background = NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK.with_a(PAUSE_MODE_ALPHA)),
            ..default()
        };

        let menu = ImageBundle {
            image: UiImage {
                texture: asset_server.load("pause/pause_menu_container.png"),
                ..default()
            },
            style: Style {
                width: Val::Percent(30.),
                height: Val::Percent(40.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        };

        let pause_menu_button = FocusableButtonBundle {
            button_bundle: ButtonBundle {
                style: Style {
                    width: Val::Percent(80.),
                    height: Val::Percent(30.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                background_color: Color::NONE.into(),
                border_color: BorderColor(Color::PURPLE),
                ..default()
            },
            ..default()
        };

        let resume_text_bundle = TextBundle::from_section(
            "Resume",
            TextStyle {
                font: dungeon_assets.ui_font.clone(),
                font_size: 50.0,
                color: Color::WHITE,
            },
        );

        let exit_text_bundle = TextBundle::from_section(
            "Exit game",
            TextStyle {
                font: dungeon_assets.ui_font.clone(),
                font_size: 50.0,
                color: Color::WHITE,
            },
        );

        commands
            .spawn((black_background, PauseModeEntity))
            .with_children(|root_background| {
                root_background.spawn(menu).with_children(|menu| {
                    menu.spawn((pause_menu_button.clone(), PauseMenuButton::Resume))
                        .with_children(|b| {
                            b.spawn(resume_text_bundle);
                        });
                    menu.spawn((pause_menu_button.clone(), PauseMenuButton::Exit))
                        .with_children(|b| {
                            b.spawn(exit_text_bundle);
                        });
                });
            });
    }
}

impl Plugin for PauseMode {
    fn build(&self, app: &mut App) {
        app.init_resource::<PreviousState>()
            .add_systems(
                OnEnter(GameModeState::Paused),
                PauseMode::darken_screen_and_show_menu,
            )
            .add_systems(
                Update,
                (
                    PauseMode::check_pause
                        .run_if(|state: Res<State<GameModeState>>| state.can_pause()),
                    (
                        PauseMode::update_button_borders.after(PauseMode::button_activation_system),
                        PauseMode::button_activation_system,
                    )
                        .after(NavRequestSystem)
                        .run_if(in_state(GameModeState::Paused)),
                ),
            )
            .add_systems(
                OnExit(GameModeState::Paused),
                (cleanup_system::<PauseModeEntity>,),
            );
    }
}

pub struct PauseModePlugins;

impl PluginGroup for PauseModePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PauseMode)
    }
}
