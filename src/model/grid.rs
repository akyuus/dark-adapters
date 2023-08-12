use crate::model::cell::{
    spawn_dungeon_cell, DungeonCell, GridPosition, TileBundlePreset, TileBundlePresetMap,
};
use crate::model::tile::TileType;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy_asset_loader::prelude::AssetCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, TypePath, TypeUuid, Resource)]
#[uuid = "ad582585-3550-465f-a2cc-8be5ed4c540a"]
pub struct RawDungeonData {
    dungeon_grid: Vec<Vec<u8>>,
    pub player_start_position: [u8; 2],
}

impl RawDungeonData {
    fn determine_preset(&self, i: i32, j: i32) -> TileBundlePreset {
        // We can determine which preset to use by examining the tiles in each cardinal direction.
        // Right    -> +X
        // Left     -> -X
        // Forward  -> -Z
        // Back     -> +Z
        if !self.cell_exists(i, j) {
            return TileBundlePreset::Empty;
        }
        let right = self.cell_exists(i, j + 1);
        let left = self.cell_exists(i, j - 1);
        let forward = self.cell_exists(i - 1, j);
        let back = self.cell_exists(i + 1, j);
        match (right, left, forward, back) {
            (true, true, true, true) => TileBundlePreset::Open,
            (true, true, false, true) => TileBundlePreset::ForwardWall,
            (false, true, true, true) => TileBundlePreset::RightWall,
            (true, true, true, false) => TileBundlePreset::BackWall,
            (true, false, true, true) => TileBundlePreset::LeftWall,
            (true, false, false, true) => TileBundlePreset::ForwardLeftCorner,
            (false, true, false, true) => TileBundlePreset::ForwardRightCorner,
            (false, true, true, false) => TileBundlePreset::BackRightCorner,
            (true, false, true, false) => TileBundlePreset::BackLeftCorner,
            (false, false, true, true) => TileBundlePreset::ForwardBackHallway,
            (true, true, false, false) => TileBundlePreset::LeftRightHallway,
            (false, false, false, true) => TileBundlePreset::ForwardHallwayEnd,
            (false, true, false, false) => TileBundlePreset::RightHallwayEnd,
            (false, false, true, false) => TileBundlePreset::BackHallwayEnd,
            (true, false, false, false) => TileBundlePreset::LeftHallwayEnd,
            (false, false, false, false) => TileBundlePreset::Empty, // should never be hit
        }
    }

    fn cell_exists(&self, i: i32, j: i32) -> bool {
        if i < 0 || j < 0 {
            return false;
        }

        let cell_option = self
            .dungeon_grid
            .get(i as usize)
            .and_then(|row| row.get(j as usize));
        cell_option.map_or(false, |val| *val > 0u8)
    }
}

#[derive(Resource, AssetCollection)]
pub struct DungeonAssets {
    #[asset(path = "dungeon_data/test.dungeon.json")]
    pub raw_dungeon_data: Handle<RawDungeonData>,
}

#[derive(Component)]
pub struct DungeonGrid {
    pub grid: Vec<Vec<DungeonCell>>,
}

impl DungeonGrid {
    pub fn check_collision(
        &mut self,
        position: &GridPosition,
        direction: Vec3,
    ) -> (GridPosition, bool) {
        let cell = &self.grid[position.row][position.col];
        if direction.x > 0.5 && cell.tile_bundle.right.tile_type == TileType::Empty {
            return (
                GridPosition {
                    row: position.row,
                    col: position.col + 1,
                },
                false,
            );
        } else if direction.x < -0.5 && cell.tile_bundle.left.tile_type == TileType::Empty {
            return (
                GridPosition {
                    row: position.row,
                    col: position.col - 1,
                },
                false,
            );
        } else if direction.z > 0.5 && cell.tile_bundle.back.tile_type == TileType::Empty {
            return (
                GridPosition {
                    row: position.row + 1,
                    col: position.col,
                },
                false,
            );
        } else if direction.z < -0.5 && cell.tile_bundle.forward.tile_type == TileType::Empty {
            return (
                GridPosition {
                    row: position.row - 1,
                    col: position.col,
                },
                false,
            );
        }
        (*position, true)
    }
}

#[derive(Bundle)]
pub struct DungeonGridBundle {
    dungeon_grid: DungeonGrid,
    spatial_bundle: SpatialBundle,
}

impl DungeonGridBundle {
    pub fn new(grid: Vec<Vec<DungeonCell>>) -> Self {
        DungeonGridBundle {
            dungeon_grid: DungeonGrid { grid },
            spatial_bundle: SpatialBundle::default(),
        }
    }
}

pub fn spawn_grid(
    dungeon_asset: ResMut<DungeonAssets>,
    grid_asset: Res<Assets<RawDungeonData>>,
    tile_bundle_map: Res<TileBundlePresetMap>,
    mut commands: Commands,
) {
    let row_ids = Vec::new();
    let mut cell_grid = Vec::new();

    let grid_handle = &dungeon_asset.raw_dungeon_data;
    let raw_dungeon_grid = grid_asset
        .get(grid_handle)
        .expect("failed to get raw dungeon grid out of assets");
    for (i, row) in raw_dungeon_grid.dungeon_grid.iter().enumerate() {
        let mut cell_row = Vec::new();
        for j in 0..row.len() {
            let preset = raw_dungeon_grid.determine_preset(i as i32, j as i32);
            let mut cell =
                DungeonCell::from_tile_bundle(tile_bundle_map.0.get(&preset).unwrap().clone());
            cell.set_position(GridPosition { row: i, col: j });
            let cloned_cell = spawn_dungeon_cell(cell, &mut commands);
            cell_row.push(cloned_cell);
        }
        cell_grid.push(cell_row);
    }
    let grid_id = commands.spawn(DungeonGridBundle::new(cell_grid)).id();
    commands.entity(grid_id).push_children(&row_ids);
}
