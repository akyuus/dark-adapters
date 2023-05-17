use bevy::prelude::*;
use bevy::utils::HashMap;
use lazy_static::lazy_static;
use std::f32::consts::PI;

use bevy_rapier3d::geometry::{ActiveCollisionTypes, Collider, CollisionGroups, Group};
use bevy_rapier3d::prelude::RigidBody;
use std::sync::Mutex;

const QUAD_WIDTH: f32 = 1.0;
pub const BASIC_TILE_GROUP: Group = Group::GROUP_2;
pub const EMPTY_TILE_GROUP: Group = Group::GROUP_3;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum PurpleTexture {
    Wall,
    Floor,
    Ceiling,
}

lazy_static! {
    static ref TILE_MESH: Mutex<Handle<Mesh>> = Mutex::new(Handle::default());
    static ref PURPLE_TEXTURE_PATHS: HashMap<PurpleTexture, &'static str> = {
        let mut m = HashMap::new();
        m.insert(PurpleTexture::Wall, "img/dun/wall1.png");
        m.insert(PurpleTexture::Floor, "img/dun/floor.png");
        m.insert(PurpleTexture::Ceiling, "img/dun/plainCeiling.png");
        m
    };
    static ref PURPLE_TEXTURE_HANDLES: Mutex<HashMap<PurpleTexture, Handle<Image>>> =
        Mutex::new(HashMap::new());
    static ref PURPLE_MATERIALS: Mutex<HashMap<PurpleTexture, Handle<StandardMaterial>>> =
        Mutex::new(HashMap::new());
    pub static ref BASIC_COLLISION_GROUP: CollisionGroups =
        CollisionGroups::new(BASIC_TILE_GROUP, Group::GROUP_1);
    static ref EMPTY_COLLISION_GROUP: CollisionGroups =
        CollisionGroups::new(EMPTY_TILE_GROUP, Group::NONE);
}

#[derive(Component, PartialEq, Clone, Debug)]
pub enum TileType {
    Empty, // nothing
    Basic, // just a texture. solid, collideable
}

#[derive(Bundle, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    collider: Collider,
    collision_groups: CollisionGroups,
    active_collision_types: ActiveCollisionTypes,
    #[bundle]
    pbr_bundle: PbrBundle,
    rigid_body: RigidBody,
}

pub fn load_handles(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("here nyow");
    let mut tile_mesh = TILE_MESH.lock().unwrap();
    *tile_mesh = meshes.add(Mesh::from(shape::Box::new(
        QUAD_WIDTH,
        QUAD_WIDTH,
        f32::EPSILON,
    )));
    for (&pt, &path) in PURPLE_TEXTURE_PATHS.iter() {
        let image: Handle<Image> = asset_server.load(path);
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(image.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });
        PURPLE_MATERIALS
            .lock()
            .unwrap()
            .insert(pt, material.clone());
        PURPLE_TEXTURE_HANDLES
            .lock()
            .unwrap()
            .insert(pt, image.clone());
    }
}

impl Tile {
    pub fn new(
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        texture_path: &str,
    ) -> Self {
        let image_handle = asset_server.load(texture_path);
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(image_handle),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });
        let pbr_bundle = PbrBundle {
            mesh: TILE_MESH.lock().unwrap().clone(),
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
            ..default()
        };
        Tile {
            tile_type: TileType::Basic,
            pbr_bundle,
            collider: Collider::cuboid(QUAD_WIDTH, QUAD_WIDTH, f32::EPSILON),
            collision_groups: BASIC_COLLISION_GROUP.clone(),
            ..default()
        }
    }

    pub fn from_texture_enum(purple_texture: PurpleTexture) -> Self {
        Tile {
            tile_type: TileType::Basic,
            pbr_bundle: PbrBundle {
                mesh: TILE_MESH.lock().unwrap().clone(),
                material: PURPLE_MATERIALS
                    .lock()
                    .unwrap()
                    .get(&purple_texture)
                    .unwrap()
                    .clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
                ..default()
            },
            collider: Collider::cuboid(QUAD_WIDTH, QUAD_WIDTH, 0.01),
            collision_groups: BASIC_COLLISION_GROUP.clone(),
            active_collision_types: ActiveCollisionTypes::empty(),
            rigid_body: RigidBody::Fixed,
        }
    }

    pub fn new_empty() -> Self {
        Tile {
            tile_type: TileType::Empty,
            pbr_bundle: PbrBundle::default(),
            collider: Collider::cuboid(QUAD_WIDTH, QUAD_WIDTH, f32::EPSILON),
            collision_groups: EMPTY_COLLISION_GROUP.clone(),
            ..default()
        }
    }
    pub fn transform(&mut self) -> &mut Transform {
        &mut self.pbr_bundle.transform as _
    }

    pub fn set_tile_transform(&mut self, transform: Transform) {
        self.pbr_bundle.transform = transform;
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            tile_type: TileType::Basic,
            pbr_bundle: PbrBundle {
                mesh: TILE_MESH.lock().unwrap().clone(),
                material: PURPLE_MATERIALS
                    .lock()
                    .unwrap()
                    .get(&PurpleTexture::Wall)
                    .unwrap()
                    .clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_x(-PI / 2.0)),
                ..default()
            },
            collider: Collider::cuboid(QUAD_WIDTH, QUAD_WIDTH, f32::EPSILON),
            collision_groups: BASIC_COLLISION_GROUP.clone(),
            active_collision_types: ActiveCollisionTypes::empty(),
            rigid_body: RigidBody::Fixed,
        }
    }
}
