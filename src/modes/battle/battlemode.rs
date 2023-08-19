use bevy::app::App;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_tweening::TweenCompleted;

use crate::model::modetraits::Registerable;
use crate::modes::battle::backgroundtiles::{
    spawn_background_tiles, unvacuum_background_tiles, ExitTweenValues,
};
use crate::modes::battle::battlemoderesources::{BattleModeAssets, BattleModeAtlases};
use crate::modes::mode_state::{cleanup_system, GameModeState};

pub struct BattleMode;

#[derive(Component)]
pub struct BattleModeEntity;

#[derive(Component)]
pub struct BattleModeCamera;

impl BattleMode {
    fn update(
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState<GameModeState>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Semicolon) {
            next_state.set(GameModeState::ExitingBattle);
        }
    }

    // TODO: this could potentially be genericized. If we use tweens to drive transition animations,
    // then we could maybe make a marker for certain types of tweens and put them in a Resource<T: Marker>
    // then make this function generic on the next state.
    // yeah that seems cool.
    fn check_unvacuum_completed(
        mut event_reader: EventReader<TweenCompleted>,
        mut exit_tween_values: ResMut<ExitTweenValues>,
        mut next_state: ResMut<NextState<GameModeState>>,
    ) {
        for _ in event_reader.iter() {
            exit_tween_values.count += 1;
        }

        if exit_tween_values.count == exit_tween_values.max {
            next_state.set(GameModeState::InDungeon);
            exit_tween_values.count = 0;
        }
    }

    fn on_exit_loading(mut commands: Commands) {
        commands.spawn((
            BattleModeCamera,
            BattleModeEntity,
            Camera2dBundle {
                camera: Camera {
                    order: 1,
                    ..default()
                },
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::None,
                },
                ..default()
            },
        ));
    }
}

impl Registerable for BattleMode {
    fn init(app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameModeState::LoadingBattle)
                .continue_to_state(GameModeState::InBattle),
        )
        .add_collection_to_loading_state::<_, BattleModeAssets>(GameModeState::LoadingBattle)
        .init_resource_after_loading_state::<_, BattleModeAtlases>(GameModeState::LoadingBattle)
        .init_resource::<ExitTweenValues>()
        .add_systems(
            OnExit(GameModeState::LoadingBattle),
            (
                BattleMode::on_exit_loading.before(spawn_background_tiles),
                spawn_background_tiles,
            ),
        )
        .add_systems(
            Update,
            (BattleMode::update).run_if(in_state(GameModeState::InBattle)),
        )
        .add_systems(
            OnEnter(GameModeState::ExitingBattle),
            unvacuum_background_tiles,
        )
        .add_systems(
            Update,
            BattleMode::check_unvacuum_completed.run_if(in_state(GameModeState::ExitingBattle)),
        )
        .add_systems(
            OnExit(GameModeState::ExitingBattle),
            cleanup_system::<BattleModeEntity>,
        );
    }
}
