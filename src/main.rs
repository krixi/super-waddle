pub mod assets;
pub mod camera;
pub mod enemy;
pub mod input;
pub mod player;

use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use crate::{
    assets::AssetsPlugin, camera::CameraPlugin, enemy::EnemyPlugin, input::InputPlugin,
    player::PlayerPlugin,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Gaming,
    GameOver,
}

fn main() {
    let mut app = App::new();

    // We use a state to coordinate asset loading
    app.init_state::<GameState>();

    // We want pixel-perfect graphics so turn off anti-aliasing
    app.insert_resource(Msaa::Off);

    // Load default plugins first
    app.add_plugins(DefaultPlugins);

    // Then configure the loading state
    app.add_loading_state(
        LoadingState::new(GameState::Loading).continue_to_state(GameState::Gaming),
    );

    // Then custom plugins
    app.add_plugins((
        AssetsPlugin,
        CameraPlugin,
        EnemyPlugin,
        InputPlugin,
        PlayerPlugin,
    ));

    // Run the game loop
    app.run();
}
