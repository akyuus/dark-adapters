use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use bevy::math::Vec3;
use bevy::prelude::{BuildChildren, Bundle, Commands, Component, Entity, Resource, SpatialBundle};
use bevy::utils::HashMap;
use lazy_static::lazy_static;

use crate::model::cell::{spawn_dungeon_cell, DungeonCell, GridPosition, TileBundlePreset};
use crate::model::tile::TileType;

pub const GRID_WIDTH: usize = 2;
pub const GRID_HEIGHT: usize = 1;

#[derive(Component)]
pub struct DungeonGrid {
    grid: Vec<Vec<DungeonCell>>,
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
    #[bundle]
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

pub fn spawn_grid(mut grid: Vec<Vec<DungeonCell>>, commands: &mut Commands) {
    let mut row_ids = Vec::new();
    let mut cell_grid = Vec::new();
    for (i, row) in grid.drain(0..).enumerate() {
        let mut cell_row = Vec::new();
        for (j, mut cell) in row.into_iter().enumerate() {
            cell.set_position(Vec3::new(j as f32, i as f32, 1.0));
            cell.grid_position = GridPosition { row: i, col: j };
            let cloned_cell = spawn_dungeon_cell(cell, commands);
            cell_row.push(cloned_cell);
        }
        cell_grid.push(cell_row);
    }
    let grid_id = commands.spawn(DungeonGridBundle::new(cell_grid)).id();
    commands.entity(grid_id).push_children(&row_ids);
}
