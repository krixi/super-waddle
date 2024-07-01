use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    ecs::system::SystemParam,
    prelude::*,
    utils::{thiserror::Error, BoxedFuture},
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
};
use serde::{Deserialize, Serialize};

use crate::GameState;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GameConfigFile>()
            .init_asset_loader::<GameConfigLoader>()
            .configure_loading_state(
                LoadingStateConfig::new(GameState::Loading)
                    .load_collection::<GameAssetCollection>(),
            );
    }
}

#[derive(AssetCollection, Resource, Default)]
pub struct GameAssetCollection {
    #[asset(path = "game_config.json")]
    pub game_config: Handle<GameConfigFile>,
}

// This is our serializable config file, which we're making as a custom asset.
#[derive(Default, Debug, Clone, Serialize, Deserialize, TypePath, Asset)]
#[serde(deny_unknown_fields)]
pub struct GameConfigFile {
    pub player_move_speed: f32,
    pub world_size: f32,
    pub num_flowers: u32,
}

// This simplifies loading the config data into a system
#[derive(SystemParam)]
pub struct GameConfig<'w> {
    pub handles: Res<'w, GameAssetCollection>,
    pub assets: Res<'w, Assets<GameConfigFile>>,
}

impl<'w> GameConfig<'w> {
    pub fn get(&self) -> Option<&GameConfigFile> {
        self.assets.get(&self.handles.game_config)
    }
}

// This leverages the 'thiserror' package to bind deserialization errors to this enum type.
#[derive(Debug, Error)]
pub enum GameConfigLoadError {
    #[error("failed to parse")]
    ParseError(#[from] serde_json::Error),
    #[error("reading asset")]
    ReadError(#[from] std::io::Error),
}

// This is the asset loader implementation that leverages serde_json to parse the file
#[derive(Default)]
pub struct GameConfigLoader;

impl AssetLoader for GameConfigLoader {
    type Asset = GameConfigFile;
    type Settings = ();
    type Error = GameConfigLoadError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        (): &'a Self::Settings,
        _: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            match serde_json::from_slice::<GameConfigFile>(&bytes) {
                Ok(cfg) => Ok(cfg),
                Err(err) => {
                    error!("Unable to parse game config: {:?}", err);
                    Err(err.into())
                }
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
