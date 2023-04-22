use bevy::prelude::*;
use std::f32::consts::PI;

const QUAD_WIDTH: f32 = 2.0;

pub trait Tile {
    fn transform(&mut self) -> &mut Transform;
    fn set_tile_transform(&mut self, transform: Transform);
}

#[derive(Component, Clone)]
pub struct EmptyTile {}

#[derive(Bundle, Clone)]
pub struct EmptyTileBundle {
    tile: EmptyTile,
    #[bundle]
    transform_bundle: TransformBundle,
}

impl Tile for EmptyTileBundle {
    fn transform(&mut self) -> &mut Transform {
        &mut self.transform_bundle.local
    }

    fn set_tile_transform(&mut self, transform: Transform) {
        self.transform_bundle.local = transform;
    }
}

impl EmptyTileBundle {
    pub fn new() -> Self {
        EmptyTileBundle {
            tile: EmptyTile {},
            transform_bundle: TransformBundle::from_transform(Transform::IDENTITY),
        }
    }
}

#[derive(Component, Clone)]
pub struct BaseTile {}

#[derive(Bundle, Clone)]
pub struct BaseTileBundle {
    tile: BaseTile,
    #[bundle]
    pbr_bundle: PbrBundle,
}

impl BaseTileBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        texture_path: &str,
    ) -> Self {
        let image_handle = asset_server.load(texture_path);
        let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
            QUAD_WIDTH, QUAD_WIDTH,
        ))));
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(image_handle),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });
        let pbr_bundle = PbrBundle {
            mesh: quad_handle,
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
            ..default()
        };
        BaseTileBundle {
            tile: BaseTile {},
            pbr_bundle,
        }
    }
}

impl Tile for BaseTileBundle {
    fn transform(&mut self) -> &mut Transform {
        
        &mut self.pbr_bundle.transform as _
    }

    fn set_tile_transform(&mut self, transform: Transform) {
        self.pbr_bundle.transform = transform;
    }
}
