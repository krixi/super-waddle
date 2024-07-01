use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum AssetsSet {
    Loading,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetHandles>()
            .add_systems(Startup, load_assets.in_set(AssetsSet::Loading));
    }
}

#[derive(Resource, Default)]
pub struct AssetHandles {
    pub goose: Handle<Image>,
    pub spike_flower: Handle<Image>,
}

fn load_assets(asset_server: Res<AssetServer>, mut asset_handles: ResMut<AssetHandles>) {
    asset_handles.goose = asset_server.load("sprites/goose.png");
    asset_handles.spike_flower = asset_server.load("sprites/spike_flower.png");
}
