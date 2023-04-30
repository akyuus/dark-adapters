use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformRotationLens};
use bevy_tweening::{Animator, AnimatorState, EaseMethod, Tween, TweeningPlugin};

use crate::model::cell::{spawn_dungeon_cell, DungeonCellBundle, TileBundle};
use crate::model::tile::{load_handles, PurpleTexture, Tile};

mod model;

#[derive(Clone, Copy, PartialEq)]
enum MovementState {
    STATIONARY,
    WALKING,
    ROTATING,
}

#[derive(Component)]
struct Player {
    movement_state: MovementState,
}

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(TweeningPlugin)
        .add_startup_system(load_handles)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .add_system(move_player.before(keyboard_input_system))
        .run();
}

/// sets up a scene with textured entities
fn setup(mut commands: Commands) {
    let wall_tile = Tile::from_texture_enum(PurpleTexture::Wall);
    let floor_tile = Tile::from_texture_enum(PurpleTexture::Floor);
    let ceiling_tile = Tile::from_texture_enum(PurpleTexture::Ceiling);

    let test_cell = DungeonCellBundle::new(Transform::from_xyz(0.0, 0.0, 0.0));
    let tile_bundle = TileBundle::new(
        wall_tile.clone(),
        wall_tile.clone(),
        wall_tile,
        Tile::new_empty(),
        ceiling_tile,
        floor_tile,
    );
    spawn_dungeon_cell(test_cell, tile_bundle, &mut commands);

    // player
    commands.spawn((
        Player {
            movement_state: MovementState::STATIONARY,
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
            transform: Transform::from_xyz(0.0, 1.0, 2.0)
                .looking_at(Vec3::new(0.0, 1.0, -1.0), Vec3::Y),
            ..default()
        },
    ));
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform, &mut Animator<Transform>), With<Camera>>,
) {
    let (mut player, transform, mut animator) = query.single_mut();

    if player.movement_state != MovementState::STATIONARY {
        return;
    }

    let mut translate_diff = Vec3::ZERO;
    let mut rotate_diff = 0_f32;
    let mut movement_state = MovementState::STATIONARY;

    if keyboard_input.just_pressed(KeyCode::Up) {
        translate_diff = transform.forward();
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        translate_diff = -transform.forward();
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        rotate_diff = PI / 2.0;
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        rotate_diff = -PI / 2.0;
    }

    if translate_diff.length_squared() > f32::EPSILON {
        movement_state = MovementState::WALKING;
        animator.set_tweenable(Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(0.3),
            TransformPositionLens {
                start: transform.translation,
                end: transform.translation + translate_diff,
            },
        ));
    } else if rotate_diff.abs() > f32::EPSILON {
        movement_state = MovementState::ROTATING;
        animator.set_tweenable(Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(0.3),
            TransformRotationLens {
                start: transform.rotation,
                end: transform.rotation * Quat::from_rotation_y(rotate_diff),
            },
        ));
        // transform.rotate_local_y(rotate_diff); // <-- works
    }
    player.movement_state = movement_state;
}

fn move_player(mut query: Query<(&mut Player, &mut Animator<Transform>)>) {
    let (mut player, mut animator) = query.single_mut();
    if player.movement_state != MovementState::STATIONARY {
        animator.state = AnimatorState::Playing;
    }
    if animator.tweenable().duration().as_secs_f32() - animator.tweenable().elapsed().as_secs_f32()
        < f32::EPSILON
        && animator.state == AnimatorState::Playing
    {
        player.movement_state = MovementState::STATIONARY;
        animator.state = AnimatorState::Paused;
    }
}
