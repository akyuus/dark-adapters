use std::time::Duration;

use bevy::math::Vec3;
use bevy::prelude::{
    default, Commands, Component, Query, Res, ResMut, Resource, SpriteSheetBundle,
    TextureAtlasSprite, Transform, Window, With,
};
use bevy::sprite::Anchor;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::{Animator, EaseMethod, RepeatCount, Tween};

use crate::model::spriteutils::{
    get_bottom_left_of_window, get_top_left_of_window, get_top_right_of_window,
    TextureAtlasSpriteLens,
};
use crate::modes::battle::battlemode::BattleModeEntity;
use crate::modes::battle::battlemoderesources::BattleModeAtlases;

const BACKGROUND_TILE_FRAME_COUNT: usize = 24;
const SCALED_BACKGROUND_TILE_WIDTH: usize = 128; // px, doubled for convenience
const VACUUM_TWEEN_DURATION_SECS: f32 = 1.0;
const SCROLLING_TWEEN_DURATION_SECS: f32 = 0.5;

#[derive(Component)]
pub struct BackgroundTile;

#[derive(Resource, Default)]
pub struct ExitTweenValues {
    pub count: u16,
    pub max: u16,
}

pub fn spawn_background_tiles(
    mut commands: Commands,
    battle_mode_atlases: Res<BattleModeAtlases>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let (top_left_x, top_left_y) = get_top_left_of_window(window);
    let (top_right_x, _) = get_top_right_of_window(window);
    let (_, bottom_left_y) = get_bottom_left_of_window(window);

    let four_width = 4 * SCALED_BACKGROUND_TILE_WIDTH as i32;
    let half_width = SCALED_BACKGROUND_TILE_WIDTH / 2;

    for y in (bottom_left_y..=(top_left_y + four_width)).step_by(half_width) {
        for x in (top_left_x..=(top_right_x + four_width)).step_by(half_width) {
            let start = Vec3::new(x as f32, y as f32, 0.0);

            // shift by 64px
            let end = Vec3::new(
                x as f32 - half_width as f32,
                y as f32 + half_width as f32,
                0.0,
            );

            let vacuum_tween = Tween::new(
                EaseMethod::Linear,
                Duration::from_secs_f32(VACUUM_TWEEN_DURATION_SECS),
                TextureAtlasSpriteLens {
                    start_index: (BACKGROUND_TILE_FRAME_COUNT - 1) as i8,
                    end_index: 0,
                },
            );

            let scrolling_tween = Tween::new(
                EaseMethod::Linear,
                Duration::from_secs_f32(SCROLLING_TWEEN_DURATION_SECS),
                TransformPositionLens { start, end },
            )
            .with_repeat_count(RepeatCount::Infinite);

            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: battle_mode_atlases.background_tile_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: BACKGROUND_TILE_FRAME_COUNT - 1,
                        anchor: Anchor::TopLeft, // makes my life easier
                        ..default()
                    },
                    transform: Transform::from_translation(start).with_scale(Vec3::splat(2.0)),
                    ..default()
                },
                BackgroundTile,
                Animator::new(vacuum_tween),
                Animator::new(scrolling_tween),
                BattleModeEntity,
            ));
        }
    }
}

pub fn unvacuum_background_tiles(
    mut query: Query<&mut Animator<TextureAtlasSprite>, With<BackgroundTile>>,
    mut exit_tween_values: ResMut<ExitTweenValues>,
) {
    let mut count: u16 = 0;
    for mut animator in query.iter_mut() {
        let unvacuum_tween = Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(VACUUM_TWEEN_DURATION_SECS),
            TextureAtlasSpriteLens {
                start_index: 0,
                end_index: (BACKGROUND_TILE_FRAME_COUNT - 1) as i8,
            },
        )
        .with_completed_event(count as u64);
        animator.set_tweenable(unvacuum_tween);
        count += 1;
    }
    exit_tween_values.max = count;
}
