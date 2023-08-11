use std::f32::consts::PI;
use std::fs;
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformRotationLens};
use bevy_tweening::{Animator, AnimatorState, EaseMethod, RepeatStrategy, Tween, TweeningPlugin};

use crate::model::cell::GridPosition;
use crate::model::grid::{spawn_grid, DungeonGrid, RawDungeonGrid};
use crate::model::tile::load_handles;

mod model;

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
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TweeningPlugin,
        ))
        .add_systems(Startup, (load_handles, setup.after(load_handles)))
        .add_systems(Update, try_move_player)
        .run();
}

fn setup(mut commands: Commands) {
    let dungeon_json: String = fs::read_to_string("assets/dungeon_data/dungeon.json").unwrap();
    let raw_dungeon: RawDungeonGrid = serde_json::from_str(&dungeon_json).unwrap();
    let grid = DungeonGrid::from_raw(raw_dungeon);
    spawn_grid(grid, &mut commands);

    // player
    // TODO: bundle this
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
            transform: Transform::from_xyz(0.0, 1.0, 1.0)
                .looking_at(Vec3::new(2.0, 1.0, 1.0), Vec3::Y),
            ..default()
        },
        GridPosition { row: 1, col: 0 },
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
