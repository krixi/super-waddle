pub mod assets;
pub mod camera;
pub mod enemy;
pub mod input;
pub mod player;

use bevy::prelude::*;

use crate::{
    assets::AssetsPlugin, camera::CameraPlugin, enemy::EnemyPlugin, input::InputPlugin,
    player::PlayerPlugin,
};

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa::Off);
    app.add_plugins((
        AssetsPlugin,
        CameraPlugin,
        DefaultPlugins,
        EnemyPlugin,
        InputPlugin,
        PlayerPlugin,
    ));

    app.run();
}
