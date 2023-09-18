use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformRotationLens};
use bevy_tweening::{Animator, AnimatorState, EaseMethod, RepeatStrategy, Tween};

use crate::modes::dungeon::model::cell::{GridDirection, GridPosType, GridPosition};
use crate::modes::dungeon::model::grid::DungeonTileLookup;
use crate::modes::dungeon::model::tile::TileType;
use crate::modes::mode_state::GameModeState;

const WALK_ANIMATION_DURATION: f32 = 0.3;
const ROTATE_ANIMATION_DURATION: f32 = 0.3;
const COLLIDE_ANIMATION_DURATION: f32 = 0.07;
const RUN_SPEED_MULTIPLIER: f32 = 1.3;

#[derive(Clone, Copy, PartialEq, Debug, Component)]
pub enum DungeonPlayerMovementState {
    Stationary,
    Walking,
    Running,
    Rotating,
    Colliding,
}

#[derive(Component)]
pub struct DungeonPlayer;

#[derive(Component)]
pub struct SpeedMultiplier(pub f32);

#[derive(Bundle)]
pub struct DungeonPlayerBundle {
    pub dungeon_player: DungeonPlayer,
    pub animator_transform: Animator<Transform>,
    pub camera: Camera3dBundle,
    pub grid_pos: GridPosition,
    pub start_direction: GridDirection,
    pub movement_state: DungeonPlayerMovementState,
    pub speed_multiplier: SpeedMultiplier,
}

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
            &mut DungeonPlayerMovementState,
            &mut SpeedMultiplier,
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
        mut speed_multiplier,
    ) = player_query.single_mut();

    if !can_change_state(&animator, *current_movement_state) {
        return;
    }

    let mut translate_player = false;
    let mut rotate_diff = 0_f32;
    let mut direction_to_translate = GridDirection::Forward;
    let mut direction_to_rotate = GridDirection::Right;
    let shift_pressed = keyboard_input.pressed(KeyCode::ShiftLeft);

    // if shift is pressed, then we can't rotate. the speed multiplier is applied and we always
    // translate
    //region keyboard input
    if keyboard_input.pressed(KeyCode::Left) {
        if shift_pressed {
            direction_to_translate = grid_direction.rotated(GridDirection::Left);
            translate_player = true;
        } else {
            rotate_diff = PI / 2.0;
            direction_to_rotate = GridDirection::Left
        }
    } else if keyboard_input.pressed(KeyCode::Right) {
        if shift_pressed {
            direction_to_translate = grid_direction.rotated(GridDirection::Right);
            translate_player = true;
        } else {
            rotate_diff = -PI / 2.0;
            direction_to_rotate = GridDirection::Right;
        }
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

    let collision = {
        let tile_entity = dungeon_tile_lookup.get_tile(*grid_pos, direction_to_translate);
        let tile_type = tile_type_query
            .get_component::<TileType>(tile_entity)
            .unwrap();
        *tile_type != TileType::Empty
    };
    let new_multiplier = if shift_pressed {
        2.0
    } else {
        RUN_SPEED_MULTIPLIER
    };
    speed_multiplier.0 = new_multiplier;

    if translate_player {
        move_or_collide(
            collision,
            &mut current_movement_state,
            &mut animator,
            &mut grid_pos,
            direction_to_translate,
            new_multiplier,
        );
        return;
    }

    if rotate_diff.abs() > f32::EPSILON {
        *current_movement_state = DungeonPlayerMovementState::Rotating;
        animator.set_tweenable(Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(ROTATE_ANIMATION_DURATION),
            TransformRotationLens {
                start: transform.rotation,
                end: transform.rotation * Quat::from_rotation_y(rotate_diff),
            },
        ));
        animator.state = AnimatorState::Playing;
        // change our direction here
        *grid_direction = grid_direction.rotated(direction_to_rotate);
        return;
    }

    // not translating or rotating, so we are now stationary
    *current_movement_state = DungeonPlayerMovementState::Stationary;
}

// TODO: probably want to separate these out into a struct?
fn move_or_collide(
    collision: bool,
    movement_state: &mut Mut<DungeonPlayerMovementState>,
    animator: &mut Mut<Animator<Transform>>,
    grid_pos: &mut Mut<GridPosition>,
    direction: GridDirection,
    speed_multiplier: f32,
) {
    // check for collision here
    if collision {
        let translate_diff: Vec3 = direction.try_into().unwrap();
        let start = grid_pos.to_vec3(GridPosType::Player);
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
        **movement_state = DungeonPlayerMovementState::Colliding;
        return;
    }
    let end_grid_pos = grid_pos.translated(direction);

    **movement_state = if speed_multiplier > 1.5 {
        DungeonPlayerMovementState::Running
    } else {
        DungeonPlayerMovementState::Walking
    };

    animator.set_tweenable(Tween::new(
        EaseMethod::Linear,
        Duration::from_secs_f32(WALK_ANIMATION_DURATION / speed_multiplier),
        TransformPositionLens {
            start: grid_pos.to_vec3(GridPosType::Player),
            end: end_grid_pos.to_vec3(GridPosType::Player),
        },
    ));
    animator.state = AnimatorState::Playing;
    // move our grid position here
    **grid_pos = end_grid_pos;
}

/// Helper function to check if we can change the player's current movement state.
fn can_change_state(
    animator: &Animator<Transform>,
    movement_state: DungeonPlayerMovementState,
) -> bool {
    let finished_animation = animator.tweenable().duration().as_secs_f32()
        - animator.tweenable().elapsed().as_secs_f32()
        < f32::EPSILON;
    match movement_state {
        DungeonPlayerMovementState::Stationary => true, // no animation playing
        DungeonPlayerMovementState::Walking
        | DungeonPlayerMovementState::Rotating
        | DungeonPlayerMovementState::Running => finished_animation,
        DungeonPlayerMovementState::Colliding => {
            // has to be 1 here because otherwise we'll switch states right when the first bounce of the collision happens
            finished_animation && animator.tweenable().times_completed() > 1
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

#[cfg(test)]
mod test {
    use std::time::Duration;

    use bevy::prelude::*;
    use bevy_tweening::lens::TransformPositionLens;
    use bevy_tweening::RepeatStrategy::MirroredRepeat;
    use bevy_tweening::{Animator, AnimatorState, EaseMethod, Tween};

    use crate::modes::dungeon::dungeonmode::test_helpers::setup_test_dungeon_assets;
    use crate::modes::dungeon::dungeonmode::DungeonMode;
    use crate::modes::dungeon::dungeonplayer::{
        can_change_state, try_move_player, DungeonPlayerMovementState,
    };
    use crate::modes::dungeon::model::cell::test_helpers::setup_test_tile_preset_map;
    use crate::modes::dungeon::model::cell::GridDirection;
    use crate::modes::dungeon::model::grid::test_helpers::setup_dungeon_tile_lookup;
    use crate::modes::dungeon::model::grid::RawDungeonData;
    use crate::modes::mode_state::GameModeState;

    fn setup(raw_dungeon_data: Option<RawDungeonData>) -> App {
        let default_data = RawDungeonData {
            dungeon_grid: vec![vec![1, 1], vec![1, 1]],
            player_start_position: [0, 0],
            player_start_direction: GridDirection::Forward,
            items: vec![],
        };
        let mut app = App::new();
        setup_test_tile_preset_map(&mut app);
        setup_dungeon_tile_lookup(&mut app);
        setup_test_dungeon_assets(&mut app, raw_dungeon_data.unwrap_or(default_data));
        app.add_state::<GameModeState>();
        let input = Input::<KeyCode>::default();
        app.insert_resource(input);
        app.add_systems(
            Startup,
            (
                DungeonMode::initialize_preset_map,
                (DungeonMode::spawn_grid, DungeonMode::setup_player)
                    .after(DungeonMode::initialize_preset_map),
            ),
        )
        .add_systems(Update, try_move_player);
        app
    }

    #[test]
    fn should_collide() {
        let mut app = setup(None);
        let mut input = app.world.get_resource_mut::<Input<KeyCode>>().unwrap();
        input.press(KeyCode::Up);
        app.update();
        let (movement_state, animator) = app
            .world
            .query::<(&DungeonPlayerMovementState, &Animator<Transform>)>()
            .single(&app.world);
        assert_eq!(*movement_state, DungeonPlayerMovementState::Colliding);
        assert_eq!(animator.state, AnimatorState::Playing);
    }

    #[test]
    fn should_rotate_left() {
        let mut app = setup(None);
        let mut input = app.world.get_resource_mut::<Input<KeyCode>>().unwrap();
        input.press(KeyCode::Left);
        app.update();
        let (movement_state, animator) = app
            .world
            .query::<(&DungeonPlayerMovementState, &Animator<Transform>)>()
            .single(&app.world);
        assert_eq!(*movement_state, DungeonPlayerMovementState::Rotating);
        assert_eq!(animator.state, AnimatorState::Playing);
    }

    #[test]
    fn should_rotate_right() {
        let mut app = setup(None);
        let mut input = app.world.get_resource_mut::<Input<KeyCode>>().unwrap();
        input.press(KeyCode::Right);
        app.update();
        let (movement_state, animator) = app
            .world
            .query::<(&DungeonPlayerMovementState, &Animator<Transform>)>()
            .single(&app.world);
        assert_eq!(*movement_state, DungeonPlayerMovementState::Rotating);
        assert_eq!(animator.state, AnimatorState::Playing);
    }

    #[test]
    fn should_walk() {
        let raw_dungeon_data = RawDungeonData {
            dungeon_grid: vec![vec![1, 1], vec![1, 1]],
            player_start_position: [0, 0],
            player_start_direction: GridDirection::Right,
            items: vec![],
        };
        let mut app = setup(Some(raw_dungeon_data));
        let mut input = app.world.get_resource_mut::<Input<KeyCode>>().unwrap();
        input.press(KeyCode::Up);
        app.update();
        let (movement_state, animator) = app
            .world
            .query::<(&DungeonPlayerMovementState, &Animator<Transform>)>()
            .single(&app.world);
        assert_eq!(*movement_state, DungeonPlayerMovementState::Walking);
        assert_eq!(animator.state, AnimatorState::Playing);
    }

    #[test]
    fn should_walk_backward() {
        let raw_dungeon_data = RawDungeonData {
            dungeon_grid: vec![vec![1, 1], vec![1, 1]],
            player_start_position: [0, 1],
            player_start_direction: GridDirection::Right,
            items: vec![],
        };
        let mut app = setup(Some(raw_dungeon_data));
        let mut input = app.world.get_resource_mut::<Input<KeyCode>>().unwrap();
        input.press(KeyCode::Down);
        app.update();
        let (movement_state, animator) = app
            .world
            .query::<(&DungeonPlayerMovementState, &Animator<Transform>)>()
            .single(&app.world);
        assert_eq!(*movement_state, DungeonPlayerMovementState::Walking);
        assert_eq!(animator.state, AnimatorState::Playing);
    }

    #[test]
    fn should_run_forward() {
        let raw_dungeon_data = RawDungeonData {
            dungeon_grid: vec![vec![1, 1], vec![1, 1]],
            player_start_position: [0, 0],
            player_start_direction: GridDirection::Right,
            items: vec![],
        };
        let mut app = setup(Some(raw_dungeon_data));
        let mut input = app.world.get_resource_mut::<Input<KeyCode>>().unwrap();
        input.press(KeyCode::Up);
        input.press(KeyCode::ShiftLeft);
        app.update();
        let (movement_state, animator) = app
            .world
            .query::<(&DungeonPlayerMovementState, &Animator<Transform>)>()
            .single(&app.world);
        assert_eq!(*movement_state, DungeonPlayerMovementState::Running);
        assert_eq!(animator.state, AnimatorState::Playing);
    }

    #[test]
    fn should_run_left() {
        let raw_dungeon_data = RawDungeonData {
            dungeon_grid: vec![vec![1, 1], vec![1, 1]],
            player_start_position: [1, 0],
            player_start_direction: GridDirection::Right,
            items: vec![],
        };
        let mut app = setup(Some(raw_dungeon_data));
        let mut input = app.world.get_resource_mut::<Input<KeyCode>>().unwrap();
        input.press(KeyCode::Left);
        input.press(KeyCode::ShiftLeft);
        app.update();
        let (movement_state, animator) = app
            .world
            .query::<(&DungeonPlayerMovementState, &Animator<Transform>)>()
            .single(&app.world);
        assert_eq!(*movement_state, DungeonPlayerMovementState::Running);
        assert_eq!(animator.state, AnimatorState::Playing);
    }

    #[test]
    fn should_run_right() {
        let raw_dungeon_data = RawDungeonData {
            dungeon_grid: vec![vec![1, 1], vec![1, 1]],
            player_start_position: [0, 0],
            player_start_direction: GridDirection::Right,
            items: vec![],
        };
        let mut app = setup(Some(raw_dungeon_data));
        let mut input = app.world.get_resource_mut::<Input<KeyCode>>().unwrap();
        input.press(KeyCode::Right);
        input.press(KeyCode::ShiftLeft);
        app.update();
        let (movement_state, animator) = app
            .world
            .query::<(&DungeonPlayerMovementState, &Animator<Transform>)>()
            .single(&app.world);
        assert_eq!(*movement_state, DungeonPlayerMovementState::Running);
        assert_eq!(animator.state, AnimatorState::Playing);
    }

    #[test]
    fn can_change_state_works() {
        let raw_dungeon_data = RawDungeonData {
            dungeon_grid: vec![vec![1, 1], vec![1, 1]],
            player_start_position: [1, 0],
            player_start_direction: GridDirection::Right,
            items: vec![],
        };
        let mut app = setup(Some(raw_dungeon_data));
        app.update();
        let mut animator = app
            .world
            .query::<&mut Animator<Transform>>()
            .single_mut(&mut app.world);
        assert!(
            can_change_state(&animator, DungeonPlayerMovementState::Stationary),
            "should be able to change state when stationary"
        );
        animator.set_tweenable(Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(1.0),
            TransformPositionLens {
                start: Default::default(),
                end: Default::default(),
            },
        ));
        animator
            .tweenable_mut()
            .set_elapsed(Duration::from_secs_f32(0.5));
        assert!(
            !can_change_state(&animator, DungeonPlayerMovementState::Walking),
            "should return false when animator in progress and walking"
        );
        assert!(
            !can_change_state(&animator, DungeonPlayerMovementState::Running),
            "should return false when animator in progress and running"
        );
        assert!(
            !can_change_state(&animator, DungeonPlayerMovementState::Rotating),
            "should return false when animator in progress and rotating"
        );
        animator
            .tweenable_mut()
            .set_elapsed(Duration::from_secs_f32(1.5));
        assert!(
            can_change_state(&animator, DungeonPlayerMovementState::Walking),
            "should return true when animator is finished and walking"
        );
        assert!(
            can_change_state(&animator, DungeonPlayerMovementState::Running),
            "should return true when animator is finished and running"
        );
        assert!(
            can_change_state(&animator, DungeonPlayerMovementState::Rotating),
            "should return true when animator is finished and rotating"
        );

        // colliding
        animator.set_tweenable(
            Tween::new(
                EaseMethod::Linear,
                Duration::from_secs_f32(1.0),
                TransformPositionLens {
                    start: Default::default(),
                    end: Default::default(),
                },
            )
            .with_repeat_count(2)
            .with_repeat_strategy(MirroredRepeat),
        );

        animator
            .tweenable_mut()
            .set_elapsed(Duration::from_secs_f32(0.5));
        assert!(
            !can_change_state(&animator, DungeonPlayerMovementState::Colliding),
            "should return false if animator in progress and colliding"
        );

        animator
            .tweenable_mut()
            .set_elapsed(Duration::from_secs_f32(1.5));

        assert!(
            !can_change_state(&animator, DungeonPlayerMovementState::Colliding),
            "should return false if animator hasn't completed twice when colliding"
        );

        animator
            .tweenable_mut()
            .set_elapsed(Duration::from_secs_f32(2.5));

        assert_eq!(animator.tweenable().times_completed(), 2);
        assert!(
            can_change_state(&animator, DungeonPlayerMovementState::Colliding),
            "should return true if animator has completed twice when colliding"
        );
    }
}
