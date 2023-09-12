use bevy::prelude::*;
use bevy::text::TextSettings;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_tweening::{component_animator_system, TweeningPlugin};
use bevy_ui_navigation::DefaultNavigationPlugins;

use crate::modes::battle::battlemode::BattleModePlugins;
use crate::modes::dungeon::dungeonmode::DungeonModePlugins;
use crate::modes::dungeon::model::grid::RawDungeonData;
use crate::modes::mode_state::GameModeState;
use crate::modes::pause::pausemode::PauseModePlugins;
use crate::modes::sharedassets::shared::SharedAssetsPlugin;

mod model;
mod modes;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(TextSettings {
            allow_dynamic_font_size: true,
            ..default()
        })
        .add_state::<GameModeState>()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dark Adapters".into(),
                        resolution: [640., 360.].into(),
                        ..default()
                    }),
                    ..default()
                })
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
            TweeningPlugin,
            JsonAssetPlugin::<RawDungeonData>::new(&["dungeon.json"]),
            DefaultNavigationPlugins,
        ))
        .add_systems(Update, component_animator_system::<TextureAtlasSprite>)
        .add_plugins(DungeonModePlugins)
        .add_plugins(BattleModePlugins)
        .add_plugins(PauseModePlugins)
        .add_plugins(SharedAssetsPlugin)
        .run();
}
