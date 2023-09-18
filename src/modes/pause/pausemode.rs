use bevy::app::{App, AppExit, PluginGroupBuilder};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::Input;
use bevy::math::Vec2;
use bevy::prelude::{
    default, in_state, BuildChildren, Camera, Camera2d, Camera2dBundle, Color, Commands, Component,
    Event, EventReader, EventWriter, IntoSystemConfigs, KeyCode, NextState, OnEnter, OnExit,
    Plugin, PluginGroup, Query, Res, ResMut, Resource, SpatialBundle, State, States, SystemSet,
    Transform, Update, Window, With, Without,
};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy_asset_loader::prelude::LoadingStateAppExt;
use bevy_tweening::Animator;
use bevy_ui_navigation::systems::InputMapping;

use crate::model::spriteutils::get_middle_left_of_window;
use crate::model::tweenutils::ExitTweenValues;
use crate::modes::dungeon::dungeonplayer::{DungeonPlayer, DungeonPlayerMovementState};
use crate::modes::mode_state::{cleanup_system, GameModeState};
use crate::modes::pause::pausemenucard::{
    spawn_cards, CardTween, PauseMenuButtonType, PauseMenuText,
};
use crate::modes::pause::pausemenucardtracker::{PauseMenuCardTracker, RotationDirection};

#[derive(Resource, Default)]
struct PreviousState(GameModeState);

#[derive(Component)]
struct PauseModeEntity;

struct PauseMode;

#[derive(Event)]
struct PauseMenuCardSelected;

const PAUSE_MODE_ALPHA: f32 = 0.7;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, SystemSet)]
pub enum PauseMenuState {
    #[default]
    Stationary,
    RotatingCard,
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

    fn darken_screen_and_show_menu(
        mut commands: Commands,
        mut input_mapping: ResMut<InputMapping>,
        mut pause_menu_card_tracker: ResMut<PauseMenuCardTracker>,
        window_query: Query<&Window>,
    ) {
        input_mapping.keyboard_navigation = true;
        input_mapping.key_action = KeyCode::Z;
        input_mapping.key_free = KeyCode::F24;
        input_mapping.focus_follows_mouse = false;

        let window = window_query.single();

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

        let black_background = SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK.with_a(PAUSE_MODE_ALPHA),
                custom_size: Some(Vec2::new(window.width() + 10.0, window.height() + 10.0)),
                ..default()
            },
            ..default()
        };

        // initial card order
        let card_types = [
            PauseMenuButtonType::Resume,
            PauseMenuButtonType::Resume,
            PauseMenuButtonType::Resume,
            PauseMenuButtonType::Resume,
            PauseMenuButtonType::Exit,
        ];
        // let current_index = 2;

        // mak
        let anchor_point = get_middle_left_of_window(window);
        pause_menu_card_tracker.anchor_point = anchor_point;

        let menu_anchor = SpatialBundle {
            transform: Transform::from_translation(anchor_point),
            ..default()
        };

        commands.spawn((black_background, PauseModeEntity));
        commands
            .spawn((menu_anchor, PauseModeEntity))
            .with_children(|anchor| {
                spawn_cards(&card_types, anchor, &mut pause_menu_card_tracker);
            });
    }

    fn handle_menu_input(
        mut card_query: Query<
            (
                &mut Animator<Transform>,
                &mut Animator<Sprite>,
                &mut Transform,
            ),
            (With<PauseMenuButtonType>, Without<PauseMenuText>),
        >,
        mut text_query: Query<&mut Transform, With<PauseMenuText>>,
        keyboard_input: Res<Input<KeyCode>>,
        (prev_state, mut next_game_state, mut next_pause_state): (
            Res<PreviousState>,
            ResMut<NextState<GameModeState>>,
            ResMut<NextState<PauseMenuState>>,
        ),
        mut pause_menu_card_tracker: ResMut<PauseMenuCardTracker>,
        mut exit_tween_values: ResMut<ExitTweenValues<CardTween>>,
        mut event_writer: EventWriter<PauseMenuCardSelected>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            next_game_state.set(prev_state.0);
            return;
        }

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
        card_type_query: Query<&PauseMenuButtonType>,
        event_reader: EventReader<PauseMenuCardSelected>,
        pause_menu_card_tracker: Res<PauseMenuCardTracker>,
        (prev_state, mut next_game_state): (Res<PreviousState>, ResMut<NextState<GameModeState>>),
        mut exit_writer: EventWriter<AppExit>,
    ) {
        if event_reader.is_empty() {
            return;
        }

        let card_type = card_type_query
            .get(pause_menu_card_tracker.cards[2])
            .unwrap();
        match card_type {
            PauseMenuButtonType::Resume => next_game_state.set(prev_state.0),
            PauseMenuButtonType::Exit => exit_writer.send(AppExit),
        }
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
        PluginGroupBuilder::start::<Self>().add(PauseMode)
    }
}
