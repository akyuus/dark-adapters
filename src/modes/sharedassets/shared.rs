use crate::modes::mode_state::GameModeState;
use bevy::app::App;
use bevy::prelude::{AssetServer, Font, Handle, Plugin, Resource};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

#[derive(Resource, AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/pc-9800.ttf")]
    pub ui_font: Handle<Font>,
}

pub struct SharedAssetsPlugin;

impl Plugin for SharedAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameModeState::LoadingSharedAssets)
                .continue_to_state(GameModeState::LoadingDungeon),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameModeState::LoadingSharedAssets);
    }
}
