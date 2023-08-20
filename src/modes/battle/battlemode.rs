use crate::model::tweenutils::ExitTweenValues;
use bevy::app::{App, PluginGroupBuilder};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use crate::modes::battle::backgroundtiles::{BackgroundTilePlugin, UnvacuumTween};
use crate::modes::battle::battlemoderesources::{BattleModeAssets, BattleModeAtlases};
use crate::modes::mode_state::{cleanup_system, GameModeState};

pub struct BattleMode;

pub struct BattleModePlugins;

#[derive(Component)]
pub struct BattleModeEntity;

#[derive(Component)]
pub struct BattleModeCamera;

impl Plugin for BattleMode {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameModeState::LoadingBattle)
                .continue_to_state(GameModeState::InBattle),
        )
        .add_collection_to_loading_state::<_, BattleModeAssets>(GameModeState::LoadingBattle)
        .init_resource_after_loading_state::<_, BattleModeAtlases>(GameModeState::LoadingBattle)
        .add_systems(
            OnExit(GameModeState::LoadingBattle),
            BattleMode::spawn_camera,
        )
        .add_systems(
            Update,
            (BattleMode::update).run_if(in_state(GameModeState::InBattle)),
        )
        .add_systems(
            Update,
            ExitTweenValues::<UnvacuumTween>::step_state_when_tweens_completed(
                GameModeState::InDungeon,
            )
            .run_if(in_state(GameModeState::ExitingBattle)),
        )
        .add_systems(
            OnExit(GameModeState::ExitingBattle),
            cleanup_system::<BattleModeEntity>,
        );
    }
}

impl PluginGroup for BattleModePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BattleMode)
            .add(BackgroundTilePlugin)
    }
}

impl BattleMode {
    fn update(
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState<GameModeState>>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Semicolon) {
            next_state.set(GameModeState::ExitingBattle);
        }
    }

    fn spawn_camera(mut commands: Commands) {
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
