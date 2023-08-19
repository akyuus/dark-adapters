use bevy::app::{App, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, OnExit};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use crate::model::modetraits::Registerable;
use crate::modes::dungeon::dungeonplayer::{setup_player, try_move_player};
use crate::modes::dungeon::model::cell::{initialize_preset_map, TileBundlePresetMap};
use crate::modes::dungeon::model::grid::{spawn_grid, DungeonAssets, DungeonTileLookup};
use crate::modes::dungeon::model::tile::{PurpleTileAssets, PurpleTileTextureMap};
use crate::modes::mode_state::GameModeState;

pub struct DungeonMode;

impl Registerable for DungeonMode {
    fn init(app: &mut App) {
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
                initialize_preset_map
                    .before(setup_player)
                    .before(spawn_grid),
                setup_player,
                spawn_grid,
            ),
        )
        .add_systems(
            Update,
            try_move_player.run_if(in_state(GameModeState::InDungeon)),
        );
    }
}

// TODO: add system to darken screen (probably in ADAPTERS-29)
