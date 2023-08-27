use std::f32::consts::{PI, TAU};
use std::time::Duration;

use bevy::app::{App, PluginGroup, PluginGroupBuilder};
use bevy::asset::{Assets, Handle};
use bevy::math::Vec3;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    default, AssetServer, Camera3dBundle, Commands, Font, IntoSystemConfigs, OnExit,
    PerspectiveProjection, Plugin, Projection, Res, ResMut, Resource, Scene, SceneBundle,
    Transform,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::{
    Animator, AnimatorState, EaseFunction, EaseMethod, RepeatCount, RepeatStrategy, Tracks, Tween,
};

use crate::model::tweenutils::PreserveQuatRotateYLens;
use crate::modes::dungeon::dungeonplayer::{DungeonPlayer, DungeonPlayerPlugin, MovementState};
use crate::modes::dungeon::model::cell::{
    spawn_dungeon_cell, DungeonCell, GridPosType, GridPosition, TileBundle, TileBundlePreset,
    TileBundlePresetMap,
};
use crate::modes::dungeon::model::grid::{DungeonTileLookup, RawDungeonData};
use crate::modes::dungeon::model::tile::{
    PurpleTexture, PurpleTileAssets, PurpleTileTextureMap, Tile,
};
use crate::modes::mode_state::GameModeState;

struct DungeonMode;

pub struct DungeonModePlugins;

#[derive(Resource, AssetCollection)]
pub struct DungeonAssets {
    #[asset(path = "dungeon_data/test.dungeon.json")]
    pub raw_dungeon_data: Handle<RawDungeonData>,
    #[asset(path = "fonts/pc-9800.ttf")]
    pub ui_font: Handle<Font>,
    #[asset(path = "model/polaroid.gltf#Scene0")]
    pub polaroid: Handle<Scene>,
}

impl DungeonMode {
    fn initialize_preset_map(
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
        let closed = TileBundle::new(
            wall_tile.clone(),
            wall_tile.clone(),
            wall_tile.clone(),
            wall_tile.clone(),
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
        let map = &mut tile_bundle_preset_map.0;
        map.insert(TileBundlePreset::Open, open);
        map.insert(TileBundlePreset::Empty, empty);
        map.insert(TileBundlePreset::Closed, closed);
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

    fn setup_player(
        mut commands: Commands,
        raw_dungeon_data: Res<Assets<RawDungeonData>>,
        dungeon_assets: Res<DungeonAssets>,
    ) {
        // player
        let grid_pos: GridPosition = raw_dungeon_data
            .get(&dungeon_assets.raw_dungeon_data)
            .unwrap()
            .player_start_position
            .try_into()
            .unwrap();
        let start_direction = raw_dungeon_data
            .get(&dungeon_assets.raw_dungeon_data)
            .unwrap()
            .player_start_direction;
        let player_pos = grid_pos.to_vec3(GridPosType::Player);
        let target = grid_pos.to_vec3(GridPosType::Player) + 2.0 * Vec3::X;
        commands.spawn((
            DungeonPlayer,
            Animator::new(Tween::new(
                EaseMethod::Linear,
                Duration::from_secs(1),
                TransformPositionLens {
                    start: Vec3::ZERO,
                    end: Vec3::new(1., 2., -4.),
                },
            ))
            .with_state(AnimatorState::Paused),
            Camera3dBundle {
                transform: Transform::from_translation(player_pos).looking_at(target, Vec3::Y),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: PI / 3.0,
                    ..default()
                }),
                ..default()
            },
            grid_pos,
            start_direction,
            MovementState::Stationary,
        ));
    }

    fn spawn_grid(
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

    fn spawn_item(mut commands: Commands, dungeon_assets: Res<DungeonAssets>) {
        let grid_position: GridPosition = [3, 3].into();
        let transform = grid_position.to_transform(GridPosType::Item);
        let scene_bundle = SceneBundle {
            scene: dungeon_assets.polaroid.clone(),
            transform,
            ..default()
        };
        let rotation_tween = Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(2.0),
            PreserveQuatRotateYLens {
                start_quat: transform.rotation,
                start: 0.0,
                end: TAU,
            },
        )
        .with_repeat_count(RepeatCount::Infinite);
        let bounce_tween = Tween::new(
            EaseMethod::EaseFunction(EaseFunction::QuadraticInOut),
            Duration::from_secs_f32(0.8),
            TransformPositionLens {
                start: transform.translation,
                end: transform.translation + 0.1 * Vec3::Y,
            },
        )
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
        .with_repeat_count(RepeatCount::Infinite);
        let track = Tracks::new([rotation_tween, bounce_tween]);
        commands.spawn((scene_bundle, Animator::new(track)));
    }

    fn unlight_all_materials(mut materials: ResMut<Assets<StandardMaterial>>) {
        for (_, material) in materials.iter_mut() {
            material.unlit = true;
        }
    }
}

impl Plugin for DungeonMode {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameModeState::LoadingDungeon)
                .continue_to_state(GameModeState::InDungeon),
        )
        .add_collection_to_loading_state::<_, PurpleTileAssets>(GameModeState::LoadingDungeon)
        .add_collection_to_loading_state::<_, DungeonAssets>(GameModeState::LoadingDungeon)
        .init_resource_after_loading_state::<_, PurpleTileTextureMap>(GameModeState::LoadingDungeon)
        .init_resource_after_loading_state::<_, TileBundlePresetMap>(GameModeState::LoadingDungeon)
        .init_resource_after_loading_state::<_, DungeonTileLookup>(GameModeState::LoadingDungeon)
        .add_systems(
            OnExit(GameModeState::LoadingDungeon),
            (
                DungeonMode::initialize_preset_map,
                (
                    DungeonMode::unlight_all_materials,
                    DungeonMode::setup_player,
                    DungeonMode::spawn_grid,
                    DungeonMode::spawn_item,
                )
                    .after(DungeonMode::initialize_preset_map),
            ),
        );
    }
}

impl PluginGroup for DungeonModePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(DungeonMode)
            .add(DungeonPlayerPlugin)
    }
}
