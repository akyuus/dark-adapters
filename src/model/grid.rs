use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::AssetCollection;
use serde::{Deserialize, Serialize};

use crate::model::cell::{
    spawn_dungeon_cell, DungeonCell, GridDirection, GridPosition, TileBundlePreset,
    TileBundlePresetMap,
};

#[derive(Serialize, Deserialize, TypePath, TypeUuid)]
#[uuid = "ad582585-3550-465f-a2cc-8be5ed4c540a"]
pub struct RawDungeonData {
    dungeon_grid: Vec<Vec<u8>>,
    pub player_start_position: [u8; 2],
    pub player_start_direction: GridDirection,
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

#[derive(Resource)]
pub struct DungeonTileLookup(Vec<Vec<HashMap<GridDirection, Entity>>>);

impl DungeonTileLookup {
    pub fn get_tile(&self, grid_position: GridPosition, direction: GridDirection) -> Entity {
        let (row, col) = (grid_position.row, grid_position.col);
        // this only works because of copy which is fine
        *self.0[row][col].get(&direction).unwrap()
    }

    pub fn insert_tile(
        &mut self,
        grid_position: GridPosition,
        direction: GridDirection,
        entity: Entity,
    ) {
        let (row, col) = (grid_position.row, grid_position.col);
        self.0[row][col].insert(direction, entity);
    }

    pub fn resize(&mut self, dungeon_grid: &Vec<Vec<u8>>) {
        let table = &mut self.0;
        let num_rows = dungeon_grid.len();
        let num_cols = dungeon_grid[0].len();
        // this is a little janky, but the 0th row will only have one hashmap if we don't do this
        for _ in 0..num_cols - 1 {
            table[0].push(HashMap::new());
        }
        table.resize_with(num_rows, || {
            let mut row = vec![];
            for _ in 0..num_cols {
                row.push(HashMap::new());
            }
            row
        });
    }
}

impl Default for DungeonTileLookup {
    fn default() -> Self {
        DungeonTileLookup(vec![vec![HashMap::new()]])
    }
}

#[derive(Resource, AssetCollection)]
pub struct DungeonAssets {
    #[asset(path = "dungeon_data/test.dungeon.json")]
    pub raw_dungeon_data: Handle<RawDungeonData>,
}

pub fn spawn_grid(
    dungeon_asset: ResMut<DungeonAssets>,
    grid_asset: Res<Assets<RawDungeonData>>,
    tile_bundle_map: Res<TileBundlePresetMap>,
    mut dungeon_tile_lookup: ResMut<DungeonTileLookup>,
    mut commands: Commands,
) {
    let grid_handle = &dungeon_asset.raw_dungeon_data;
    let raw_dungeon_grid = grid_asset
        .get(grid_handle)
        .expect("failed to get raw dungeon grid out of assets");

    // first we need to resize the lookup resource
    dungeon_tile_lookup.resize(&raw_dungeon_grid.dungeon_grid);
    let num_rows = raw_dungeon_grid.dungeon_grid.len();
    for (i, row) in raw_dungeon_grid.dungeon_grid.iter().enumerate() {
        // panic if this isn't a square
        if num_rows != row.len() {
            panic!("failed to spawn dungeon because it is not a square");
        }

        for j in 0..row.len() {
            let preset = raw_dungeon_grid.determine_preset(i as i32, j as i32);
            let grid_position = GridPosition { row: i, col: j };
            let cell =
                DungeonCell::from_tile_bundle(tile_bundle_map.0.get(&preset).unwrap().clone());
            spawn_dungeon_cell(cell, grid_position, &mut commands, &mut dungeon_tile_lookup);
        }
    }
}
