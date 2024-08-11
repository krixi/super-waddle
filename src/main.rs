#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod assets;
mod camera;
mod enemy;
mod game;
mod input;
mod player;
mod ui;

use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

use crate::{
    assets::AssetsPlugin, camera::CameraPlugin, enemy::EnemyPlugin, game::GamePlugin,
    input::InputPlugin, player::PlayerPlugin, ui::UiPlugin,
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
        GamePlugin,
        InputPlugin,
        PlayerPlugin,
        UiPlugin,
    ));

    // Run the game loop
    app.run();
}
