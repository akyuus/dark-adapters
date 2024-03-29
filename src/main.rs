use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
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
use crate::utils::utilresources::WindowScaleFactor;
use crate::utils::utilsystems::{resize_sprite_system, resize_text_system, update_scale_factor};

mod modes;
mod utils;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(TextSettings {
            allow_dynamic_font_size: true,
            ..default()
        })
        .insert_resource(WindowScaleFactor(2.0))
        .add_state::<GameModeState>()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dark Adapters".into(),
                        resolution: [1280., 720.].into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
            TweeningPlugin,
            JsonAssetPlugin::<RawDungeonData>::new(&["dungeon.json"]),
            DefaultNavigationPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            // WorldInspectorPlugin::new(),
        ))
        .add_systems(PreUpdate, update_scale_factor)
        .add_systems(
            Update,
            (
                component_animator_system::<TextureAtlasSprite>,
                resize_sprite_system,
                resize_text_system,
            ),
        )
        .add_plugins(DungeonModePlugins)
        .add_plugins(BattleModePlugins)
        .add_plugins(PauseModePlugins)
        .add_plugins(SharedAssetsPlugin)
        .run();
}
