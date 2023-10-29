use bevy::app::{App, AppExit, PluginGroupBuilder};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::Input;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{
    default, in_state, AlignItems, BuildChildren, ButtonBundle, Camera, Camera2d, Camera2dBundle,
    Color, Commands, Component, Event, EventReader, EventWriter, IntoSystemConfigs, KeyCode,
    NextState, NodeBundle, OnEnter, OnExit, Plugin, PluginGroup, Query, Res, ResMut, Resource,
    SpatialBundle, State, States, SystemSet, TextBundle, Transform, Update, Window, With, Without,
};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::text::TextStyle;
use bevy::ui::{FlexDirection, JustifyContent, Style, UiRect, Val};
use bevy::window::WindowResized;
use bevy_asset_loader::prelude::LoadingStateAppExt;
use bevy_tweening::Animator;
use bevy_ui_navigation::components::FocusableButtonBundle;
use bevy_ui_navigation::systems::InputMapping;

use crate::modes::dungeon::dungeonplayer::{DungeonPlayer, DungeonPlayerMovementState};
use crate::modes::mode_state::GameModeState;
use crate::modes::pause::optionsmenu::ResolutionOptions::Fullscreen;
use crate::modes::pause::optionsmenu::{OptionsMenuPlugin, ResolutionOptions};
use crate::modes::pause::pausemenucard::{
    spawn_cards, CardTween, PauseMenuCardType, PauseMenuText,
};
use crate::modes::pause::pausemenucardtracker::{
    PauseMenuCardTracker, RotationDirection, PAUSE_BUTTON_CARD_WIDTH,
};
use crate::modes::sharedassets::shared::FontAssets;
use crate::utils::spriteutils::get_middle_left_of_window;
use crate::utils::tweenutils::ExitTweenValues;
use crate::utils::utilresources::WindowScaleFactor;
use crate::utils::utilsystems::{cleanup_system, ScalableSpriteComponent};

#[derive(Resource, Default)]
struct PreviousState(GameModeState);

#[derive(Component)]
struct PauseModeEntity;

struct PauseMode;

#[derive(Component)]
pub struct OptionsMenuRoot;

#[derive(Event)]
struct PauseMenuCardSelected;

#[derive(Component)]
struct PauseMenuAnchor;

const PAUSE_MODE_ALPHA: f32 = 0.7;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, SystemSet)]
pub enum PauseMenuState {
    #[default]
    Stationary,
    RotatingCard,
    InOptionsMenu,
}

impl PauseMode {
    fn check_pause(
        keyboard_input: Res<Input<KeyCode>>,
        mut previous_state: ResMut<PreviousState>,
        current_state: Res<State<GameModeState>>,
        mut next_state: ResMut<NextState<GameModeState>>,
        player_query: Query<&DungeonPlayerMovementState, With<DungeonPlayer>>,
    ) {
        // don't allow pausing when moving
        if let Ok(&movement_state) = player_query.get_single() {
            if !(movement_state == DungeonPlayerMovementState::Stationary) {
                return;
            }
        }

        if keyboard_input.just_pressed(KeyCode::Escape) {
            previous_state.0 = **current_state;
            next_state.set(GameModeState::Paused);
        }
    }

    fn move_cards_on_window_resize(
        mut anchor_query: Query<&mut Transform, With<PauseMenuAnchor>>,
        mut card_query: Query<&mut Transform, (With<PauseMenuCardType>, Without<PauseMenuAnchor>)>,
        mut resize_reader: EventReader<WindowResized>,
        mut pause_menu_card_tracker: ResMut<PauseMenuCardTracker>,
        window_scale_factor: Res<WindowScaleFactor>,
    ) {
        let mut anchor_transform = anchor_query.single_mut();
        for e in resize_reader.iter() {
            let anchor_point = Vec3::new(-e.width / 2.0, 0.0, 0.0);
            println!("{}, {}", e.width, e.height);
            anchor_transform.translation = anchor_point;
            pause_menu_card_tracker.anchor_point = anchor_point;
            for (i, e) in pause_menu_card_tracker.cards.iter().enumerate() {
                let scale_value = if i == 2 {
                    1.0
                } else {
                    1.0 - 0.15 * (2.0 - i as f32).abs()
                };
                let mut card_transform = card_query.get_mut(*e).unwrap();
                *card_transform = Transform::from_xyz(
                    (PAUSE_BUTTON_CARD_WIDTH / 2.0 - 20.0) * window_scale_factor.0,
                    0.,
                    card_transform.translation.z,
                )
                .with_scale(Vec3::new(scale_value, scale_value, 1.0));
                card_transform.rotate_around(
                    anchor_point,
                    Quat::from_rotation_z(pause_menu_card_tracker.angles[i]),
                );
            }
        }
    }

    fn darken_screen_and_show_menu(
        mut commands: Commands,
        mut input_mapping: ResMut<InputMapping>,
        mut pause_menu_card_tracker: ResMut<PauseMenuCardTracker>,
        mut pause_state: ResMut<NextState<PauseMenuState>>,
        scale_factor: Res<WindowScaleFactor>,
        window_query: Query<&Window>,
    ) {
        input_mapping.keyboard_navigation = true;
        input_mapping.key_action = KeyCode::Z;
        input_mapping.key_cancel = KeyCode::X;
        input_mapping.key_free = KeyCode::F24;
        input_mapping.focus_follows_mouse = false;

        let window = window_query.single();

        pause_state.set(PauseMenuState::Stationary);

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

        let bg_width = window.width() + 10.0;
        let bg_height = window.height() + 10.0;

        let black_background = SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK.with_a(PAUSE_MODE_ALPHA),
                custom_size: Some(Vec2::new(bg_width, bg_height)),
                ..default()
            },
            ..default()
        };

        // initial card order
        let card_types = [
            PauseMenuCardType::Resume,
            PauseMenuCardType::Options,
            PauseMenuCardType::Resume,
            PauseMenuCardType::Resume,
            PauseMenuCardType::Exit,
        ];
        // let current_index = 2;

        // mak
        let anchor_point = get_middle_left_of_window(window);
        pause_menu_card_tracker.anchor_point = anchor_point;

        let menu_anchor = SpatialBundle {
            transform: Transform::from_translation(anchor_point),
            ..default()
        };

        commands.spawn((
            black_background,
            PauseModeEntity,
            ScalableSpriteComponent {
                base_width: bg_width,
                base_height: bg_height,
            },
        ));
        commands
            .spawn((menu_anchor, PauseModeEntity, PauseMenuAnchor))
            .with_children(|anchor| {
                spawn_cards(
                    &card_types,
                    anchor,
                    &mut pause_menu_card_tracker,
                    scale_factor.0,
                );
            });
    }

    fn exit_pause_menu(
        keyboard_input: Res<Input<KeyCode>>,
        prev_state: Res<PreviousState>,
        current_state: Res<State<PauseMenuState>>,
        mut next_game_state: ResMut<NextState<GameModeState>>,
    ) {
        let esc_pressed = keyboard_input.just_pressed(KeyCode::Escape);
        let x_pressed_in_stationary_state =
            keyboard_input.just_pressed(KeyCode::X) && *current_state == PauseMenuState::Stationary;

        if esc_pressed || x_pressed_in_stationary_state {
            next_game_state.set(prev_state.0);
            return;
        }
    }
    fn handle_menu_input(
        mut card_query: Query<
            (
                &mut Animator<Transform>,
                &mut Animator<Sprite>,
                &mut Transform,
            ),
            (With<PauseMenuCardType>, Without<PauseMenuText>),
        >,
        mut text_query: Query<&mut Transform, With<PauseMenuText>>,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_pause_state: ResMut<NextState<PauseMenuState>>,
        mut pause_menu_card_tracker: ResMut<PauseMenuCardTracker>,
        mut exit_tween_values: ResMut<ExitTweenValues<CardTween>>,
        mut event_writer: EventWriter<PauseMenuCardSelected>,
    ) {
        if keyboard_input.pressed(KeyCode::Down) {
            pause_menu_card_tracker.rotate(
                RotationDirection::Counterclockwise,
                &mut card_query,
                &mut text_query,
                &mut exit_tween_values,
                &mut next_pause_state,
            );
        } else if keyboard_input.pressed(KeyCode::Up) {
            pause_menu_card_tracker.rotate(
                RotationDirection::Clockwise,
                &mut card_query,
                &mut text_query,
                &mut exit_tween_values,
                &mut next_pause_state,
            );
        }

        if keyboard_input.just_pressed(KeyCode::Z) {
            event_writer.send(PauseMenuCardSelected);
        }
    }

    fn handle_selected_card(
        card_type_query: Query<&PauseMenuCardType>,
        event_reader: EventReader<PauseMenuCardSelected>,
        pause_menu_card_tracker: Res<PauseMenuCardTracker>,
        (prev_state, mut next_game_state, mut next_pause_state): (
            Res<PreviousState>,
            ResMut<NextState<GameModeState>>,
            ResMut<NextState<PauseMenuState>>,
        ),
        mut exit_writer: EventWriter<AppExit>,
        font_assets: Res<FontAssets>,
        mut commands: Commands,
    ) {
        if event_reader.is_empty() {
            return;
        }

        let card_type = card_type_query
            .get(pause_menu_card_tracker.cards[2])
            .unwrap();
        match card_type {
            PauseMenuCardType::Resume => next_game_state.set(prev_state.0),
            PauseMenuCardType::Exit => exit_writer.send(AppExit),
            PauseMenuCardType::Options => {
                PauseMode::spawn_options_menu(&mut commands, &font_assets);
                next_pause_state.set(PauseMenuState::InOptionsMenu);
            }
        }
    }

    fn spawn_options_menu(commands: &mut Commands, font_assets: &Res<FontAssets>) {
        // bg color: #B9C6D8
        let root = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        };

        let option_menu_bg = NodeBundle {
            style: Style {
                width: Val::Percent(95.0),
                height: Val::Percent(75.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::hex("#B9C6D8").unwrap().into(),
            ..default()
        };

        let resolution_options_container = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        };
        let button_labels = [
            ("640 x 360", ResolutionOptions::Small),
            ("1280 x 720", ResolutionOptions::Medium),
            ("1600 x 900", ResolutionOptions::Large),
            ("Fullscreen", Fullscreen),
        ];
        commands
            .spawn((root, PauseModeEntity, OptionsMenuRoot))
            .with_children(|parent| {
                parent.spawn(option_menu_bg).with_children(|bg| {
                    bg.spawn(resolution_options_container).with_children(|roc| {
                        for (label, res_option_type) in button_labels {
                            roc.spawn((
                                FocusableButtonBundle {
                                    button_bundle: ButtonBundle {
                                        style: Style {
                                            width: Val::Percent(20.0),
                                            height: Val::Percent(90.0),
                                            border: UiRect::all(Val::Px(4.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::GRAY.into(),
                                        border_color: Color::BLACK.into(),
                                        ..default()
                                    },
                                    ..default()
                                },
                                res_option_type,
                            ))
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section(
                                    label,
                                    TextStyle {
                                        font: font_assets.ui_font.clone(),
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                    },
                                ));
                            });
                        }
                    });
                });
            });
    }
}

impl Plugin for PauseMode {
    fn build(&self, app: &mut App) {
        app.init_resource::<PreviousState>()
            .init_resource::<ExitTweenValues<CardTween>>()
            .init_resource_after_loading_state::<_, PauseMenuCardTracker>(
                GameModeState::LoadingSharedAssets,
            ) // the font doesn't get loaded otherwise
            .add_state::<PauseMenuState>()
            .add_event::<PauseMenuCardSelected>()
            .add_systems(
                OnEnter(GameModeState::Paused),
                PauseMode::darken_screen_and_show_menu,
            )
            .add_systems(
                Update,
                (PauseMode::check_pause
                    .run_if(|state: Res<State<GameModeState>>| state.can_pause()),),
            )
            .add_systems(
                Update,
                (
                    (
                        PauseMode::handle_menu_input,
                        PauseMode::handle_selected_card,
                    )
                        .run_if(in_state(PauseMenuState::Stationary)),
                    ExitTweenValues::<CardTween>::step_state_when_tweens_completed(
                        PauseMenuState::Stationary,
                    )
                    .run_if(in_state(PauseMenuState::RotatingCard)),
                    (
                        PauseMode::exit_pause_menu,
                        PauseMode::move_cards_on_window_resize,
                    ),
                )
                    .run_if(in_state(GameModeState::Paused)),
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
        PluginGroupBuilder::start::<Self>()
            .add(PauseMode)
            .add(OptionsMenuPlugin)
    }
}
