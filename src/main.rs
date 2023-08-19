use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_tweening::{component_animator_system, TweeningPlugin};

use crate::model::modetraits::RegisterTarget;
use crate::modes::battle::battlemode::BattleMode;
use crate::modes::dungeon::dungeonmode::DungeonMode;
use crate::modes::dungeon::model::grid::RawDungeonData;
use crate::modes::mode_state::GameModeState;

mod model;
mod modes;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_state::<GameModeState>()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
            TweeningPlugin,
            JsonAssetPlugin::<RawDungeonData>::new(&["dungeon.json"]),
        ))
        .add_systems(Update, component_animator_system::<TextureAtlasSprite>)
        .add_systems(Update, close_on_esc)
        .register::<BattleMode>()
        .register::<DungeonMode>()
        .run();
}
