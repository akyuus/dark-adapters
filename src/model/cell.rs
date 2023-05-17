use std::f32::consts::PI;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use bevy::prelude::{
    BuildChildren, Bundle, Commands, Component, Entity, Quat, SpatialBundle, Transform, Vec3,
};
use bevy::utils::HashMap;
use lazy_static::lazy_static;

use crate::model::tile::{PurpleTexture, Tile};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TileBundlePreset {
    Open,
    NorthWall,
    EastWall,
    SouthWall,
    WestWall,
    NorthWestCorner,
    NorthEastCorner,
    SouthEastCorner,
    SouthWestCorner,
    NorthSouthHallway,
    EastWestHallway,
    NorthHallwayEnd,
    EastHallwayEnd,
    SouthHallwayEnd,
    WestHallwayEnd,
}

lazy_static! {
    pub static ref TILE_BUNDLE_PRESET_MAP: HashMap<TileBundlePreset, TileBundle> = {
        let wall_tile = Tile::from_texture_enum(PurpleTexture::Wall);
        let floor_tile = Tile::from_texture_enum(PurpleTexture::Floor);
        let ceiling_tile = Tile::from_texture_enum(PurpleTexture::Ceiling);
        let open = TileBundle::new(
            Tile::new_empty(),
            Tile::new_empty(),
            Tile::new_empty(),
            Tile::new_empty(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let north_wall = TileBundle::new(
            Tile::new_empty(),
            wall_tile.clone(),
            Tile::new_empty(),
            Tile::new_empty(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let east_wall = TileBundle::new(
            Tile::new_empty(),
            Tile::new_empty(),
            wall_tile.clone(),
            Tile::new_empty(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let south_wall = TileBundle::new(
            Tile::new_empty(),
            Tile::new_empty(),
            Tile::new_empty(),
            wall_tile.clone(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let west_wall = TileBundle::new(
            wall_tile.clone(),
            Tile::new_empty(),
            Tile::new_empty(),
            Tile::new_empty(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let north_west_corner = TileBundle::new(
            wall_tile.clone(),
            wall_tile.clone(),
            Tile::new_empty(),
            Tile::new_empty(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let north_east_corner = TileBundle::new(
            Tile::new_empty(),
            wall_tile.clone(),
            wall_tile.clone(),
            Tile::new_empty(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let south_east_corner = TileBundle::new(
            Tile::new_empty(),
            Tile::new_empty(),
            wall_tile.clone(),
            wall_tile.clone(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let south_west_corner = TileBundle::new(
            wall_tile.clone(),
            Tile::new_empty(),
            Tile::new_empty(),
            wall_tile.clone(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let north_south_hallway = TileBundle::new(
            wall_tile.clone(),
            Tile::new_empty(),
            wall_tile.clone(),
            Tile::new_empty(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let east_west_hallway = TileBundle::new(
            Tile::new_empty(),
            wall_tile.clone(),
            Tile::new_empty(),
            wall_tile.clone(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let north_hallway_end = TileBundle::new(
            wall_tile.clone(),
            wall_tile.clone(),
            wall_tile.clone(),
            Tile::new_empty(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let east_hallway_end = TileBundle::new(
            Tile::new_empty(),
            wall_tile.clone(),
            wall_tile.clone(),
            wall_tile.clone(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let south_hallway_end = TileBundle::new(
            wall_tile.clone(),
            Tile::new_empty(),
            wall_tile.clone(),
            wall_tile.clone(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let west_hallway_end = TileBundle::new(
            wall_tile.clone(),
            wall_tile.clone(),
            Tile::new_empty(),
            wall_tile.clone(),
            ceiling_tile.clone(),
            floor_tile.clone(),
        );
        let mut m = HashMap::new();
        m.insert(TileBundlePreset::Open, open);
        m.insert(TileBundlePreset::NorthWall, north_wall);
        m.insert(TileBundlePreset::EastWall, east_wall);
        m.insert(TileBundlePreset::SouthWall, south_wall);
        m.insert(TileBundlePreset::WestWall, west_wall);
        m.insert(TileBundlePreset::NorthWestCorner, north_west_corner);
        m.insert(TileBundlePreset::NorthEastCorner, north_east_corner);
        m.insert(TileBundlePreset::SouthEastCorner, south_east_corner);
        m.insert(TileBundlePreset::SouthWestCorner, south_west_corner);
        m.insert(TileBundlePreset::NorthSouthHallway, north_south_hallway);
        m.insert(TileBundlePreset::EastWestHallway, east_west_hallway);
        m.insert(TileBundlePreset::NorthHallwayEnd, north_hallway_end);
        m.insert(TileBundlePreset::EastHallwayEnd, east_hallway_end);
        m.insert(TileBundlePreset::SouthHallwayEnd, south_hallway_end);
        m.insert(TileBundlePreset::WestHallwayEnd, west_hallway_end);
        m
    };
}

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
            TileDirection::LEFT => Transform::from_xyz(-0.5000001, 1.0, 0.0)
                .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
            TileDirection::FORWARD => {
                Transform::from_xyz(0.0, 1.0, -0.5000001).with_rotation(Quat::from_rotation_z(PI))
            }
            TileDirection::RIGHT => Transform::from_xyz(0.5000001, 1.0, 0.0)
                .with_rotation(Quat::from_rotation_y(PI / 2.0)),
            TileDirection::BACK => Transform::from_xyz(0.0, 1.0, 0.5000001),
            TileDirection::TOP => Transform::from_xyz(0.0, 1.5000001, 0.0)
                .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            TileDirection::BOTTOM => Transform::from_xyz(0.0, 0.5000001, 0.0)
                .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
        }
    }
    fn set_tile_transform(tile: &mut Tile, transform: Transform) {
        tile.set_tile_transform(transform);
    }
}

#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub enum DungeonCellType {
    Basic,
}

#[derive(Component, Copy, Clone)]
pub struct GridPosition {
    pub row: usize,
    pub col: usize,
}

#[derive(Bundle)]
pub struct DungeonCell {
    pub cell_type: DungeonCellType,
    #[bundle]
    spatial_bundle: SpatialBundle,
    #[bundle]
    pub tile_bundle: TileBundle,
    pub grid_position: GridPosition,
}

impl DungeonCell {
    pub fn from_preset(preset: TileBundlePreset) -> Self {
        // positions are handled by grid
        DungeonCell {
            cell_type: DungeonCellType::Basic,
            spatial_bundle: SpatialBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
            tile_bundle: TILE_BUNDLE_PRESET_MAP.get(&preset).unwrap().clone(),
            grid_position: GridPosition { row: 0, col: 0 },
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.spatial_bundle.transform.translation = position;
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
    #[bundle]
    pub left: Tile,
    #[bundle]
    pub forward: Tile,
    #[bundle]
    pub right: Tile,
    #[bundle]
    pub back: Tile,
    #[bundle]
    pub top: Tile,
    #[bundle]
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

pub fn spawn_dungeon_cell(cell: DungeonCell, commands: &mut Commands) -> DungeonCell {
    let cloned_cell = cell.clone();
    let left = commands.spawn(cell.tile_bundle.left).id();
    let forward = commands.spawn(cell.tile_bundle.forward).id();
    let right = commands.spawn(cell.tile_bundle.right).id();
    let back = commands.spawn(cell.tile_bundle.back).id();
    let top = commands.spawn(cell.tile_bundle.top).id();
    let bottom = commands.spawn(cell.tile_bundle.bottom).id();
    let cell_id = commands.spawn((cell.cell_type, cell.spatial_bundle)).id();
    commands
        .entity(cell_id)
        .push_children(&[left, forward, right, back, top, bottom]);
    cloned_cell
}
