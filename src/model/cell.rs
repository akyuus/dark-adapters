use crate::model::tile::Tile;
use bevy::math::Vec3;
use bevy::prelude::{BuildChildren, Bundle, Commands, Component, Quat, SpatialBundle, Transform};
use std::f32::consts::PI;

pub enum TileDirection {
    LEFT,
    FORWARD,
    RIGHT,
    BACK,
    TOP,
    BOTTOM,
}

impl TileDirection {
    fn get_tile_transform(direction: TileDirection) -> Transform {
        // these tiny offsets are here to prevent z-fighting
        match direction {
            TileDirection::LEFT => Transform::from_xyz(-1.0000001, 1.0, -1.0)
                .with_rotation(Quat::from_rotation_y(PI / 2.0)),
            TileDirection::FORWARD => Transform::from_xyz(0.0, 1.0, -2.0000001),
            TileDirection::RIGHT => Transform::from_xyz(1.0000001, 1.0, -1.0)
                .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
            TileDirection::BACK => Transform::from_xyz(0.0, 1.0, 0.0000001),
            TileDirection::TOP => Transform::from_xyz(0.0, 2.00000001, -1.0)
                .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            TileDirection::BOTTOM => Transform::from_xyz(0.0, -0.00000001, -1.0)
                .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
        }
    }
    fn set_tile_transform(tile: &mut Tile, transform: Transform) {
        tile.set_tile_transform(transform);
    }
}

#[derive(Component, Clone)]
pub struct DungeonCell {}

#[derive(Bundle)]
pub struct DungeonCellBundle {
    cell: DungeonCell,
    #[bundle]
    spatial_bundle: SpatialBundle,
}

impl DungeonCellBundle {
    pub fn new(transform: Transform) -> Self {
        DungeonCellBundle {
            cell: DungeonCell {},
            spatial_bundle: SpatialBundle::from_transform(transform),
        }
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    #[bundle]
    left: Tile,
    #[bundle]
    forward: Tile,
    #[bundle]
    right: Tile,
    #[bundle]
    back: Tile,
    #[bundle]
    top: Tile,
    #[bundle]
    bottom: Tile,
}

impl TileBundle {
    pub fn new(
        mut left: Tile,
        mut forward: Tile,
        mut right: Tile,
        mut back: Tile,
        mut top: Tile,
        mut bottom: Tile,
    ) -> Self {
        TileDirection::set_tile_transform(
            &mut left,
            TileDirection::get_tile_transform(TileDirection::LEFT),
        );
        TileDirection::set_tile_transform(
            &mut forward,
            TileDirection::get_tile_transform(TileDirection::FORWARD),
        );
        TileDirection::set_tile_transform(
            &mut right,
            TileDirection::get_tile_transform(TileDirection::RIGHT),
        );
        TileDirection::set_tile_transform(
            &mut back,
            TileDirection::get_tile_transform(TileDirection::BACK),
        );
        TileDirection::set_tile_transform(
            &mut top,
            TileDirection::get_tile_transform(TileDirection::TOP),
        );
        TileDirection::set_tile_transform(
            &mut bottom,
            TileDirection::get_tile_transform(TileDirection::BOTTOM),
        );

        TileBundle {
            left,
            right,
            forward,
            back,
            top,
            bottom,
        }
    }
}

pub fn spawn_dungeon_cell(
    cell_bundle: DungeonCellBundle,
    tile_bundle: TileBundle,
    commands: &mut Commands,
) {
    let left = commands.spawn(tile_bundle.left).id();
    let forward = commands.spawn(tile_bundle.forward).id();
    let right = commands.spawn(tile_bundle.right).id();
    let back = commands.spawn(tile_bundle.back).id();
    let top = commands.spawn(tile_bundle.top).id();
    let bottom = commands.spawn(tile_bundle.bottom).id();
    let c_bundle = commands.spawn(cell_bundle).id();
    commands
        .entity(c_bundle)
        .push_children(&[left, forward, right, back, top, bottom]);
}
