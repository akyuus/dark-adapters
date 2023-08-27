use crate::model::tweenutils::PreserveQuatRotateYLens;
use crate::modes::dungeon::dungeonmode::DungeonAssets;
use crate::modes::dungeon::model::cell::{GridPosType, GridPosition};
use bevy::math::Vec3;
use bevy::prelude::{default, Commands, Component, Res, SceneBundle};
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::{
    Animator, EaseFunction, EaseMethod, RepeatCount, RepeatStrategy, Tracks, Tween,
};
use serde::Deserialize;
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Component, Deserialize, Clone, Copy, PartialEq)]
pub enum ItemType {
    Polaroid,
    Key,
    Maxwell,
}

pub struct DungeonItem;

impl DungeonItem {
    pub fn spawn(
        commands: &mut Commands,
        item_type: ItemType,
        grid_pos: GridPosition,
        dungeon_assets: &Res<DungeonAssets>,
    ) {
        let scene_handle = match item_type {
            ItemType::Polaroid => dungeon_assets.polaroid.clone(),
            ItemType::Key => dungeon_assets.key.clone(),
            ItemType::Maxwell => dungeon_assets.maxwell.clone(),
        };
        let transform = if item_type == ItemType::Maxwell {
            grid_pos
                .to_transform(GridPosType::Item)
                .with_scale(Vec3::splat(0.01))
        } else {
            grid_pos.to_transform(GridPosType::Item)
        };

        let scene_bundle = SceneBundle {
            scene: scene_handle,
            transform,
            ..default()
        };
        let rotation_tween = Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(2.0),
            PreserveQuatRotateYLens {
                start_quat: transform.rotation,
                start: 0.0,
                end: TAU,
            },
        )
        .with_repeat_count(RepeatCount::Infinite);
        let bounce_tween = Tween::new(
            EaseMethod::EaseFunction(EaseFunction::QuadraticInOut),
            Duration::from_secs_f32(0.8),
            TransformPositionLens {
                start: transform.translation,
                end: transform.translation + 0.1 * Vec3::Y,
            },
        )
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
        .with_repeat_count(RepeatCount::Infinite);
        let track = Tracks::new([rotation_tween, bounce_tween]);
        commands.spawn((scene_bundle, Animator::new(track)));
    }
}
