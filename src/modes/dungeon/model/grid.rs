use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::utils::HashMap;
use serde::Deserialize;

use crate::modes::dungeon::model::cell::{GridDirection, GridPosition, TileBundlePreset};
use crate::modes::dungeon::model::items::ItemType;

#[derive(Deserialize)]
pub struct RawDungeonItemData {
    pub item_type: ItemType,
    pub item_position: [u8; 2],
}

#[derive(Deserialize, TypePath, TypeUuid)]
#[uuid = "ad582585-3550-465f-a2cc-8be5ed4c540a"]
pub struct RawDungeonData {
    pub dungeon_grid: Vec<Vec<u8>>,
    pub player_start_position: [u8; 2],
    pub player_start_direction: GridDirection,
    pub items: Vec<RawDungeonItemData>,
}

impl RawDungeonData {
    pub fn determine_preset(&self, i: i32, j: i32) -> TileBundlePreset {
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
            (false, false, false, false) => TileBundlePreset::Closed, // should never be hit
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

#[cfg(test)]
pub mod test_helpers {
    use super::*;
    use bevy::prelude::App;

    pub fn setup_dungeon_tile_lookup(app: &mut App) {
        app.insert_resource(DungeonTileLookup::default());
    }
}
