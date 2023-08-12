use std::f32::consts::PI;

use bevy::prelude::{
    BuildChildren, Bundle, Commands, Component, Quat, ResMut, Resource, SpatialBundle, Transform,
    Vec3,
};
use bevy::utils::HashMap;

use crate::model::tile::{PurpleTexture, PurpleTileTextureMap, Tile};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TileBundlePreset {
    Empty,
    Open,
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

pub fn initialize_preset_map(
    tile_texture_map: ResMut<PurpleTileTextureMap>,
    mut tile_bundle_preset_map: ResMut<TileBundlePresetMap>,
) {
    let wall_tile = tile_texture_map.0.get(&PurpleTexture::Wall).unwrap();
    let floor_tile = tile_texture_map.0.get(&PurpleTexture::Floor).unwrap();
    let ceiling_tile = tile_texture_map.0.get(&PurpleTexture::Ceiling).unwrap();
    //region AUUGGGHHH
    let open = TileBundle::new(
        Tile::new_empty(),
        Tile::new_empty(),
        Tile::new_empty(),
        Tile::new_empty(),
        ceiling_tile.clone(),
        floor_tile.clone(),
    );
    let empty = TileBundle::new(
        Tile::new_empty(),
        Tile::new_empty(),
        Tile::new_empty(),
        Tile::new_empty(),
        Tile::new_empty(),
        Tile::new_empty(),
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
    let map = &mut tile_bundle_preset_map.0;
    map.insert(TileBundlePreset::Open, open);
    map.insert(TileBundlePreset::Empty, empty);
    map.insert(TileBundlePreset::ForwardWall, north_wall);
    map.insert(TileBundlePreset::RightWall, east_wall);
    map.insert(TileBundlePreset::BackWall, south_wall);
    map.insert(TileBundlePreset::LeftWall, west_wall);
    map.insert(TileBundlePreset::ForwardLeftCorner, north_west_corner);
    map.insert(TileBundlePreset::ForwardRightCorner, north_east_corner);
    map.insert(TileBundlePreset::BackRightCorner, south_east_corner);
    map.insert(TileBundlePreset::BackLeftCorner, south_west_corner);
    map.insert(TileBundlePreset::ForwardBackHallway, north_south_hallway);
    map.insert(TileBundlePreset::LeftRightHallway, east_west_hallway);
    map.insert(TileBundlePreset::ForwardHallwayEnd, north_hallway_end);
    map.insert(TileBundlePreset::RightHallwayEnd, east_hallway_end);
    map.insert(TileBundlePreset::BackHallwayEnd, south_hallway_end);
    map.insert(TileBundlePreset::LeftHallwayEnd, west_hallway_end);
    //endregion
}

pub enum TileDirection {
    Left,
    Forward,
    Right,
    Back,
    Top,
    Bottom,
}

impl TileDirection {
    fn get_tile_transform(direction: TileDirection) -> Transform {
        // these tiny offsets are here to prevent z-fighting
        match direction {
            TileDirection::Left => Transform::from_xyz(-0.5000001, 1.0, 0.0)
                .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
            TileDirection::Forward => {
                Transform::from_xyz(0.0, 1.0, -0.5000001).with_rotation(Quat::from_rotation_z(PI))
            }
            TileDirection::Right => Transform::from_xyz(0.5000001, 1.0, 0.0)
                .with_rotation(Quat::from_rotation_y(PI / 2.0)),
            TileDirection::Back => Transform::from_xyz(0.0, 1.0, 0.5000001),
            TileDirection::Top => Transform::from_xyz(0.0, 1.5000001, 0.0)
                .with_rotation(Quat::from_rotation_x(PI / 2.0)),
            TileDirection::Bottom => Transform::from_xyz(0.0, 0.5000001, 0.0)
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

impl GridPosition {
    pub fn to_player_vec3(self) -> Vec3 {
        Vec3::new(self.col as f32, 1.0, self.row as f32)
    }

    pub fn to_cell_vec3(self) -> Vec3 {
        Vec3::new(self.col as f32, 0.0, self.row as f32)
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
        self.spatial_bundle.transform.translation = grid_position.to_cell_vec3();
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
        TileDirection::set_tile_transform(
            &mut left,
            TileDirection::get_tile_transform(TileDirection::Left),
        );
        TileDirection::set_tile_transform(
            &mut forward,
            TileDirection::get_tile_transform(TileDirection::Forward),
        );
        TileDirection::set_tile_transform(
            &mut right,
            TileDirection::get_tile_transform(TileDirection::Right),
        );
        TileDirection::set_tile_transform(
            &mut back,
            TileDirection::get_tile_transform(TileDirection::Back),
        );
        TileDirection::set_tile_transform(
            &mut top,
            TileDirection::get_tile_transform(TileDirection::Top),
        );
        TileDirection::set_tile_transform(
            &mut bottom,
            TileDirection::get_tile_transform(TileDirection::Bottom),
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
    // TODO: ADAPTERS-16
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
