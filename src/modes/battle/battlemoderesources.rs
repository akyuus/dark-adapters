use bevy::asset::{Assets, Handle};
use bevy::math::Vec2;
use bevy::prelude::{AssetServer, FromWorld, Image, Resource, TextureAtlas, World};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct BattleModeAssets {
    #[asset(path = "img/enc/animations/tile_fade.png")]
    pub background_tile_image: Handle<Image>,
}

#[derive(Resource)]
pub struct BattleModeAtlases {
    pub background_tile_atlas: Handle<TextureAtlas>,
}

impl FromWorld for BattleModeAtlases {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let battle_mode_assets = cell
            .get_resource::<BattleModeAssets>()
            .expect("could not get battle mode assets");

        let mut texture_atlases = cell
            .get_resource_mut::<Assets<TextureAtlas>>()
            .expect("could not get texture atlases");

        let background_tile_atlas = TextureAtlas::from_grid(
            battle_mode_assets.background_tile_image.clone(),
            64.0 * Vec2::ONE,
            6,
            4,
            None,
            None,
        );

        let handle = texture_atlases.add(background_tile_atlas);
        BattleModeAtlases {
            background_tile_atlas: handle,
        }
    }
}
