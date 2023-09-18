use std::f32::consts::PI;

use bevy::prelude::{
    BuildChildren, Bundle, Commands, Component, Entity, Quat, ResMut, Resource, SpatialBundle,
    Transform, Vec3,
};
use bevy::utils::HashMap;
use bevy_mod_picking::prelude::RaycastPickTarget;
use bevy_mod_picking::PickableBundle;
use serde::{Deserialize, Serialize};

use crate::modes::dungeon::model::grid::DungeonTileLookup;
use crate::modes::dungeon::model::tile::Tile;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TileBundlePreset {
    Empty,
    Open,
    Closed, // gay baby jail
    ForwardWall,
    RightWall,
    BackWall,
    LeftWall,
    ForwardLeftCorner,
    ForwardRightCorner,
    BackRightCorner,
    BackLeftCorner,
    ForwardBackHallway,
    LeftRightHallway,
    ForwardHallwayEnd,
    RightHallwayEnd,
    BackHallwayEnd,
    LeftHallwayEnd,
}

#[derive(Resource)]
pub struct TileBundlePresetMap(pub HashMap<TileBundlePreset, TileBundle>);

impl Default for TileBundlePresetMap {
    fn default() -> Self {
        TileBundlePresetMap(HashMap::new())
    }
}

#[derive(Hash, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Component, Debug)]
pub enum GridDirection {
    Left = 0,
    Forward = 1,
    Right = 2,
    Back = 3,
    Top = 4,
    Bottom = 5,
}

impl GridDirection {
    fn get_tile_transform(direction: GridDirection) -> Transform {
        // these tiny offsets are here to prevent z-fighting
        match direction {
            GridDirection::Left => Transform::from_xyz(-0.5000001, 1.0, 0.0)
                .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
            GridDirection::Forward => {
                Transform::from_xyz(0.0, 1.0, -0.5000001).with_rotation(Quat::from_rotation_z(PI))
            }
            GridDirection::Right => Transform::from_xyz(0.5000001, 1.0, 0.0)
                .with_rotation(Quat::from_rotation_y(PI / 2.0)),
            GridDirection::Back => Transform::from_xyz(0.0, 1.0, 0.5000001),
            GridDirection::Top => Transform::from_xyz(0.0, 1.5000001, 0.0)
                .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            GridDirection::Bottom => Transform::from_xyz(0.0, 0.5000001, 0.0)
                .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
        }
    }
    fn set_tile_transform(tile: &mut Tile, transform: Transform) {
        tile.set_tile_transform(transform);
    }

    pub fn rotated(self, rotate_dir: GridDirection) -> Self {
        match rotate_dir {
            GridDirection::Left => {
                let num_self = self as i8;
                let new_dir: GridDirection = ((num_self + 3) % 4).try_into().unwrap();
                new_dir
            }
            GridDirection::Right => {
                let num_self = self as i8;
                let new_dir: GridDirection = ((num_self + 1) % 4).try_into().unwrap();
                new_dir
            }
            _ => self,
        }
    }

    pub fn get_inverse_direction(self) -> Self {
        match self {
            GridDirection::Left => GridDirection::Right,
            GridDirection::Forward => GridDirection::Back,
            GridDirection::Right => GridDirection::Left,
            GridDirection::Back => GridDirection::Forward,
            GridDirection::Top => GridDirection::Bottom,
            GridDirection::Bottom => GridDirection::Top,
        }
    }
}

impl From<i8> for GridDirection {
    fn from(value: i8) -> Self {
        match value {
            0 => GridDirection::Left,
            1 => GridDirection::Forward,
            2 => GridDirection::Right,
            3 => GridDirection::Back,
            4 => GridDirection::Top,
            5 => GridDirection::Bottom,
            _ => GridDirection::Forward, // this shouldn't ever happen
        }
    }
}

impl From<GridDirection> for Vec3 {
    fn from(val: GridDirection) -> Self {
        match val {
            GridDirection::Left => Vec3::new(-1.0, 0.0, 0.0),
            GridDirection::Forward => Vec3::new(0.0, 0.0, -1.0),
            GridDirection::Right => Vec3::new(1.0, 0.0, 0.0),
            GridDirection::Back => Vec3::new(0.0, 0.0, 1.0),
            GridDirection::Top => Vec3::new(0.0, 1.0, 0.0),
            GridDirection::Bottom => Vec3::new(0.0, -1.0, 0.0),
        }
    }
}

#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub enum DungeonCellType {
    Basic,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct GridPosition {
    pub row: usize,
    pub col: usize,
}

/// Different things can be placed on a grid. This enum describes them.
pub enum GridPosType {
    Player,
    Cell,
    Item,
}

impl GridPosition {
    ///
    pub fn to_transform(self, grid_pos_type: GridPosType) -> Transform {
        match grid_pos_type {
            GridPosType::Player => Transform::from_translation(self.to_vec3(grid_pos_type)),
            GridPosType::Cell => Transform::from_translation(self.to_vec3(grid_pos_type)),
            GridPosType::Item => Transform::from_translation(self.to_vec3(grid_pos_type))
                .with_scale(Vec3::splat(0.3))
                .with_rotation(Quat::from_rotation_z(PI / 7.0)),
        }
    }

    pub fn to_vec3(self, grid_pos_type: GridPosType) -> Vec3 {
        match grid_pos_type {
            GridPosType::Player => Vec3::new(self.col as f32, 1.0, self.row as f32),
            GridPosType::Cell => Vec3::new(self.col as f32, 0.0, self.row as f32),
            GridPosType::Item => Vec3::new(self.col as f32, 0.75, self.row as f32),
        }
    }

    /// this can panic! make sure to only use this when you know a collision won't occur
    pub fn translated(self, direction: GridDirection) -> Self {
        match direction {
            GridDirection::Left => GridPosition {
                row: self.row,
                col: self.col - 1,
            },
            GridDirection::Forward => GridPosition {
                row: self.row - 1,
                col: self.col,
            },
            GridDirection::Right => GridPosition {
                row: self.row,
                col: self.col + 1,
            },
            GridDirection::Back => GridPosition {
                row: self.row + 1,
                col: self.col,
            },
            _ => self,
        }
    }
}

impl From<[u8; 2]> for GridPosition {
    fn from(value: [u8; 2]) -> Self {
        GridPosition {
            row: value[0] as usize,
            col: value[1] as usize,
        }
    }
}

#[derive(Bundle)]
pub struct DungeonCell {
    pub cell_type: DungeonCellType,
    spatial_bundle: SpatialBundle,
    pub tile_bundle: TileBundle,
    pub grid_position: GridPosition,
}

impl DungeonCell {
    pub fn from_tile_bundle(bundle: TileBundle) -> Self {
        // positions are handled by grid
        DungeonCell {
            cell_type: DungeonCellType::Basic,
            spatial_bundle: SpatialBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
            tile_bundle: bundle,
            grid_position: GridPosition { row: 0, col: 0 },
        }
    }

    pub fn set_position(&mut self, grid_position: GridPosition) {
        self.spatial_bundle.transform.translation = grid_position.to_vec3(GridPosType::Cell);
        self.grid_position = grid_position;
    }
}

impl Clone for DungeonCell {
    fn clone(&self) -> Self {
        DungeonCell {
            cell_type: self.cell_type,
            spatial_bundle: SpatialBundle::default(),
            tile_bundle: self.tile_bundle.clone(),
            grid_position: self.grid_position,
        }
    }
}

#[derive(Bundle, Clone)]
pub struct TileBundle {
    pub left: Tile,
    pub forward: Tile,
    pub right: Tile,
    pub back: Tile,
    pub top: Tile,
    pub bottom: Tile,
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
        GridDirection::set_tile_transform(
            &mut left,
            GridDirection::get_tile_transform(GridDirection::Left),
        );
        GridDirection::set_tile_transform(
            &mut forward,
            GridDirection::get_tile_transform(GridDirection::Forward),
        );
        GridDirection::set_tile_transform(
            &mut right,
            GridDirection::get_tile_transform(GridDirection::Right),
        );
        GridDirection::set_tile_transform(
            &mut back,
            GridDirection::get_tile_transform(GridDirection::Back),
        );
        GridDirection::set_tile_transform(
            &mut top,
            GridDirection::get_tile_transform(GridDirection::Top),
        );
        GridDirection::set_tile_transform(
            &mut bottom,
            GridDirection::get_tile_transform(GridDirection::Bottom),
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
    mut cell: DungeonCell,
    grid_position: GridPosition,
    commands: &mut Commands,
    dungeon_tile_lookup: &mut ResMut<DungeonTileLookup>,
) {
    let mut insert_into_lookup_closure = |direction: GridDirection, entity: Entity| {
        dungeon_tile_lookup.insert_tile(grid_position, direction, entity);
    };

    cell.set_position(grid_position);
    let left = commands
        .spawn((
            cell.tile_bundle.left,
            PickableBundle::default(),
            RaycastPickTarget::default(),
        ))
        .id();
    insert_into_lookup_closure(GridDirection::Left, left);

    let forward = commands
        .spawn((
            cell.tile_bundle.forward,
            PickableBundle::default(),
            RaycastPickTarget::default(),
        ))
        .id();
    insert_into_lookup_closure(GridDirection::Forward, forward);

    let right = commands
        .spawn((
            cell.tile_bundle.right,
            PickableBundle::default(),
            RaycastPickTarget::default(),
        ))
        .id();
    insert_into_lookup_closure(GridDirection::Right, right);

    let back = commands
        .spawn((
            cell.tile_bundle.back,
            PickableBundle::default(),
            RaycastPickTarget::default(),
        ))
        .id();
    insert_into_lookup_closure(GridDirection::Back, back);

    let top = commands
        .spawn((
            cell.tile_bundle.top,
            PickableBundle::default(),
            RaycastPickTarget::default(),
        ))
        .id();
    insert_into_lookup_closure(GridDirection::Top, top);

    let bottom = commands
        .spawn((
            cell.tile_bundle.bottom,
            PickableBundle::default(),
            RaycastPickTarget::default(),
        ))
        .id();
    insert_into_lookup_closure(GridDirection::Bottom, bottom);

    let cell_id = commands
        .spawn((
            cell.cell_type,
            cell.spatial_bundle,
            PickableBundle::default(),
            RaycastPickTarget::default(),
        ))
        .id();
    commands
        .entity(cell_id)
        .push_children(&[left, forward, right, back, top, bottom]);
}

pub mod test_helpers {
    use super::*;
    use crate::modes::dungeon::model::tile::test_helpers::setup_test_texture_map;
    use bevy::prelude::App;

    pub fn setup_test_tile_preset_map(app: &mut App) {
        setup_test_texture_map(app);
        let tile_preset_map = TileBundlePresetMap::default();
        app.insert_resource(tile_preset_map);
    }
}
