use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_inspector_egui::prelude::*;

const QUAD_WIDTH: f32 = 1.0;

#[derive(Reflect, Hash, PartialEq, Eq, Copy, Clone, Debug, Component, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum TileTexture {
    Wall,
    Floor,
    Ceiling,
}

#[derive(AssetCollection, Resource)]
pub struct PurpleTileAssets {
    #[asset(standard_material)]
    #[asset(path = "img/dun/wall1.png")]
    pub wall: Handle<StandardMaterial>,
    #[asset(standard_material)]
    #[asset(path = "img/dun/floor.png")]
    pub floor: Handle<StandardMaterial>,
    #[asset(standard_material)]
    #[asset(path = "img/dun/plainCeiling.png")]
    pub ceiling: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct PurpleTileTextureMap(pub HashMap<TileTexture, Tile>);

impl FromWorld for PurpleTileTextureMap {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let tile_material_handles = cell
            .get_resource_mut::<PurpleTileAssets>()
            .expect("failed to get purple tile asset handles");
        let mut meshes = cell
            .get_resource_mut::<Assets<Mesh>>()
            .expect("failed to get meshes");
        let mesh_handle = meshes.add(Mesh::from(shape::Box::new(
            QUAD_WIDTH,
            QUAD_WIDTH,
            f32::EPSILON,
        )));
        let wall_handle = &tile_material_handles.wall;
        let floor_handle = &tile_material_handles.floor;
        let ceiling_handle = &tile_material_handles.ceiling;
        let wall = Tile {
            tile_type: TileType::Basic,
            pbr_bundle: PbrBundle {
                mesh: mesh_handle.clone(),
                material: wall_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
                ..default()
            },
        };
        let floor = Tile {
            tile_type: TileType::Basic,
            pbr_bundle: PbrBundle {
                mesh: mesh_handle.clone(),
                material: floor_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
                ..default()
            },
        };
        let ceiling = Tile {
            tile_type: TileType::Basic,
            pbr_bundle: PbrBundle {
                mesh: mesh_handle.clone(),
                material: ceiling_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
                ..default()
            },
        };

        let map = HashMap::from([
            (TileTexture::Wall, wall),
            (TileTexture::Floor, floor),
            (TileTexture::Ceiling, ceiling),
        ]);
        PurpleTileTextureMap(map)
    }
}

#[derive(Component, PartialEq, Clone, Debug)]
pub enum TileType {
    Empty, // nothing
    Basic, // just a texture. solid, collideable
}

#[derive(Bundle, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pbr_bundle: PbrBundle,
}

impl Tile {
    pub fn new_empty() -> Self {
        Tile {
            tile_type: TileType::Empty,
            pbr_bundle: PbrBundle::default(),
        }
    }

    pub fn set_tile_transform(&mut self, transform: Transform) {
        self.pbr_bundle.transform = transform;
    }
}

#[cfg(test)]
pub mod test_helpers {
    use super::*;
    use bevy::prelude::App;

    fn make_basic_tile() -> Tile {
        Tile {
            tile_type: TileType::Basic,
            pbr_bundle: Default::default(),
        }
    }
    pub fn setup_test_texture_map(app: &mut App) {
        let mut map: HashMap<TileTexture, Tile> = HashMap::new();
        map.insert(TileTexture::Wall, make_basic_tile());
        map.insert(TileTexture::Floor, make_basic_tile());
        map.insert(TileTexture::Ceiling, make_basic_tile());
        let tile_texture_map = PurpleTileTextureMap(map);
        app.insert_resource(tile_texture_map);
    }
}
