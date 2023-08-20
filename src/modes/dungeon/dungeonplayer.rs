use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformRotationLens};
use bevy_tweening::{Animator, AnimatorState, EaseMethod, RepeatStrategy, Tween};

use crate::modes::dungeon::model::cell::{GridDirection, GridPosition};
use crate::modes::dungeon::model::grid::DungeonTileLookup;
use crate::modes::dungeon::model::tile::TileType;
use crate::modes::mode_state::GameModeState;

const WALK_ANIMATION_DURATION: f32 = 0.3;
const ROTATE_ANIMATION_DURATION: f32 = 0.3;
const COLLIDE_ANIMATION_DURATION: f32 = 0.07;

#[derive(Clone, Copy, PartialEq, Debug, Component)]
pub enum MovementState {
    Stationary,
    Walking,
    Rotating,
    Colliding,
}

#[derive(Component)]
pub struct DungeonPlayer;

pub fn try_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    dungeon_tile_lookup: Res<DungeonTileLookup>,
    mut next_state: ResMut<NextState<GameModeState>>,
    mut player_query: Query<
        (
            Entity,
            &mut Transform,
            &mut Animator<Transform>,
            &mut GridPosition,
            &mut GridDirection,
            &mut MovementState,
        ),
        With<DungeonPlayer>,
    >,
    tile_type_query: Query<(Entity, &TileType)>,
) {
    let (
        _id,
        transform,
        mut animator,
        mut grid_pos,
        mut grid_direction,
        mut current_movement_state,
    ) = player_query.single_mut();

    let mut translate_player = false;
    let mut rotate_diff = 0_f32;
    let mut direction_to_translate = GridDirection::Forward;
    let mut direction_to_rotate = GridDirection::Right;

    //region keyboard input
    if keyboard_input.pressed(KeyCode::Left) {
        rotate_diff = PI / 2.0;
        direction_to_rotate = GridDirection::Left
    } else if keyboard_input.pressed(KeyCode::Right) {
        rotate_diff = -PI / 2.0;
        direction_to_rotate = GridDirection::Right;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        direction_to_translate = *grid_direction;
        translate_player = true;
    } else if keyboard_input.pressed(KeyCode::Down) {
        direction_to_translate = grid_direction.get_inverse_direction();
        translate_player = true;
    }
    if keyboard_input.just_pressed(KeyCode::Semicolon) {
        next_state.set(GameModeState::LoadingBattle);
        return;
    }
    //endregion

    let check_collision = || {
        let tile_entity = dungeon_tile_lookup.get_tile(*grid_pos, direction_to_translate);
        let tile_type = tile_type_query
            .get_component::<TileType>(tile_entity)
            .unwrap();
        *tile_type != TileType::Empty
    };

    if can_change_state(&animator, *current_movement_state)
        && animator.state == AnimatorState::Playing
        && *current_movement_state != MovementState::Stationary
    {
        if translate_player {
            let collision = check_collision();
            walk_or_collide(
                collision,
                &mut current_movement_state,
                &mut animator,
                &mut grid_pos,
                direction_to_translate,
            );
            return;
        }

        *current_movement_state = MovementState::Stationary;
        animator.state = AnimatorState::Paused;
    }

    // player moving?
    if *current_movement_state != MovementState::Stationary {
        return;
    }

    if translate_player {
        // check for collision here
        let collision = check_collision();
        walk_or_collide(
            collision,
            &mut current_movement_state,
            &mut animator,
            &mut grid_pos,
            direction_to_translate,
        );
    } else if rotate_diff.abs() > f32::EPSILON {
        *current_movement_state = MovementState::Rotating;
        animator.set_tweenable(Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(ROTATE_ANIMATION_DURATION),
            TransformRotationLens {
                start: transform.rotation,
                end: transform.rotation * Quat::from_rotation_y(rotate_diff),
            },
        ));
        // change our direction here
        *grid_direction = grid_direction.get_rotated_direction(direction_to_rotate);
    }

    // play animation
    if *current_movement_state != MovementState::Stationary {
        animator.state = AnimatorState::Playing;
    }
}

// TODO: probably want to separate these out into a struct?
fn walk_or_collide(
    collision: bool,
    movement_state: &mut Mut<MovementState>,
    animator: &mut Mut<Animator<Transform>>,
    grid_pos: &mut Mut<GridPosition>,
    direction: GridDirection,
) {
    // check for collision here
    if !collision {
        // this cannot be done outside of this if block! we could panic because grid positions are unsigned
        let end_grid_pos = grid_pos.translated(direction);
        **movement_state = MovementState::Walking;
        animator.set_tweenable(Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(WALK_ANIMATION_DURATION),
            TransformPositionLens {
                start: grid_pos.to_player_vec3(),
                end: end_grid_pos.to_player_vec3(),
            },
        ));
        // move our grid position here
        **grid_pos = end_grid_pos;
    } else {
        // hitting wall
        let translate_diff: Vec3 = direction.try_into().unwrap();
        let start = grid_pos.to_player_vec3();
        let end = start + 0.18 * translate_diff;
        let collision_tween = Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(COLLIDE_ANIMATION_DURATION),
            TransformPositionLens { start, end },
        )
        .with_repeat_count(2)
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat);
        animator.set_tweenable(collision_tween);
        animator.state = AnimatorState::Playing;
        **movement_state = MovementState::Colliding;
    }
}

fn can_change_state(animator: &Animator<Transform>, movement_state: MovementState) -> bool {
    let duration_minus_elapsed = animator.tweenable().duration().as_secs_f32()
        - animator.tweenable().elapsed().as_secs_f32()
        < f32::EPSILON;
    match movement_state {
        MovementState::Stationary => true, // no animation playing
        MovementState::Walking | MovementState::Rotating => duration_minus_elapsed,
        MovementState::Colliding => {
            // println!("elapsed: {}", animator.tweenable().elapsed().as_secs_f32());
            // println!(
            //     "times_completed: {}",
            //     animator.tweenable().times_completed()
            // );
            // has to be 1 here because otherwise we'll switch states right when the first bounce of the collision happens
            duration_minus_elapsed && animator.tweenable().times_completed() > 1
        }
    }
}

pub struct DungeonPlayerPlugin;

impl Plugin for DungeonPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            try_move_player.run_if(in_state(GameModeState::InDungeon)),
        );
    }
}
