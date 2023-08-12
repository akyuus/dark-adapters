use std::f32::consts::PI;
use std::time::Duration;

use crate::model::cell::{initialize_preset_map, GridPosition, TileBundlePresetMap};
use crate::model::grid::{spawn_grid, DungeonAssets, DungeonGrid, RawDungeonData};
use crate::model::tile::{PurpleTileAssets, PurpleTileTextureMap};
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_tweening::lens::{TransformPositionLens, TransformRotationLens};
use bevy_tweening::{Animator, AnimatorState, EaseMethod, RepeatStrategy, Tween, TweeningPlugin};

mod model;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    LoadingAssets,
    Ready,
}

#[derive(Clone, Copy, PartialEq)]
enum MovementState {
    Stationary,
    Walking,
    Rotating,
}

#[derive(Component)]
struct Player {
    movement_state: MovementState,
}

// struct TranslatePlayerEvent(Entity);

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::LoadingAssets).continue_to_state(GameState::Ready),
        )
        .add_collection_to_loading_state::<_, PurpleTileAssets>(GameState::LoadingAssets)
        .add_collection_to_loading_state::<_, DungeonAssets>(GameState::LoadingAssets)
        .init_resource_after_loading_state::<_, PurpleTileTextureMap>(GameState::LoadingAssets)
        .init_resource_after_loading_state::<_, TileBundlePresetMap>(GameState::LoadingAssets)
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TweeningPlugin,
            JsonAssetPlugin::<RawDungeonData>::new(&["dungeon.json"]),
        ))
        .add_systems(
            OnEnter(GameState::Ready),
            (
                initialize_preset_map
                    .before(setup_player)
                    .before(spawn_grid),
                setup_player,
                spawn_grid,
            ),
        )
        .add_systems(
            Update,
            (try_move_player, close_on_esc).run_if(in_state(GameState::Ready)),
        )
        .run();
}

fn setup_player(
    mut commands: Commands,
    raw_dungeon_data: Res<Assets<RawDungeonData>>,
    dungeon_assets: Res<DungeonAssets>,
) {
    // player
    // TODO: bundle this
    let grid_pos: GridPosition = raw_dungeon_data
        .get(&dungeon_assets.raw_dungeon_data)
        .unwrap()
        .player_start_position
        .try_into()
        .unwrap();
    let player_pos = grid_pos.to_player_vec3();
    let target = grid_pos.to_player_vec3() + 2.0 * Vec3::X;
    commands.spawn((
        Player {
            movement_state: MovementState::Stationary,
        },
        Animator::new(Tween::new(
            EaseMethod::Linear,
            Duration::from_secs(1),
            TransformPositionLens {
                start: Vec3::ZERO,
                end: Vec3::new(1., 2., -4.),
            },
        ))
        .with_state(AnimatorState::Paused),
        Camera3dBundle {
            transform: Transform::from_translation(player_pos).looking_at(target, Vec3::Y),
            ..default()
        },
        grid_pos,
    ));
}

fn try_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            Entity,
            &mut Player,
            &mut Transform,
            &mut Animator<Transform>,
            &mut GridPosition,
        ),
        With<Camera>,
    >,
    mut grid_query: Query<&mut DungeonGrid>,
) {
    let (_id, mut player, transform, mut animator, mut grid_pos) = player_query.single_mut();
    let mut dungeon_grid = grid_query.single_mut();
    // animation over?
    if animator.tweenable().duration().as_secs_f32() - animator.tweenable().elapsed().as_secs_f32()
        < f32::EPSILON
        && animator.state == AnimatorState::Playing
        && player.movement_state != MovementState::Stationary
    {
        player.movement_state = MovementState::Stationary;
        animator.state = AnimatorState::Paused;
    }

    // player moving?
    if player.movement_state != MovementState::Stationary {
        return;
    }

    let mut translate_diff = Vec3::ZERO;
    let mut rotate_diff = 0_f32;
    let mut movement_state = MovementState::Stationary;

    //region keyboard input
    if keyboard_input.just_pressed(KeyCode::Up) {
        translate_diff = transform.forward();
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        translate_diff = -transform.forward();
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        rotate_diff = PI / 2.0;
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        rotate_diff = -PI / 2.0;
    }
    //endregion

    if translate_diff.length_squared() > f32::EPSILON {
        // check for collision here
        let (new_pos, collision_occurred) = dungeon_grid.check_collision(&grid_pos, translate_diff);
        if !collision_occurred {
            movement_state = MovementState::Walking;
            animator.set_tweenable(Tween::new(
                EaseMethod::Linear,
                Duration::from_secs_f32(0.3),
                TransformPositionLens {
                    start: transform.translation,
                    end: transform.translation + translate_diff,
                },
            ));
            grid_pos.col = new_pos.col;
            grid_pos.row = new_pos.row;
        } else {
            // hitting wall
            let start = transform.translation;
            let end = transform.translation + 0.3 * translate_diff;
            let collision_tween = Tween::new(
                EaseMethod::Linear,
                Duration::from_secs_f32(0.1),
                TransformPositionLens { start, end },
            )
            .with_repeat_count(2)
            .with_repeat_strategy(RepeatStrategy::MirroredRepeat);
            animator.set_tweenable(collision_tween);
            animator.state = AnimatorState::Playing;
        }
    } else if rotate_diff.abs() > f32::EPSILON {
        movement_state = MovementState::Rotating;
        animator.set_tweenable(Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(0.3),
            TransformRotationLens {
                start: transform.rotation,
                end: transform.rotation * Quat::from_rotation_y(rotate_diff),
            },
        ));
    }
    player.movement_state = movement_state;

    // play animation
    if player.movement_state != MovementState::Stationary {
        animator.state = AnimatorState::Playing;
    }
}
