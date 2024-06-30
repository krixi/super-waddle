use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetHandles>()
            .add_event::<LoadingComplete>()
            .add_systems(Startup, load_assets);
    }
}

#[derive(Default, Event)]
pub struct LoadingComplete;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct AssetHandles {
    pub goose: Handle<Image>,
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut asset_handles: ResMut<AssetHandles>,
    mut events: EventWriter<LoadingComplete>,
) {
    asset_handles.goose = asset_server.load("sprites/goose.png");
    events.send_default();
}
