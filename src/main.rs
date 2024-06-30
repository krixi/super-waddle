pub mod assets;
pub mod camera;
pub mod player;

use bevy::prelude::*;

use crate::{assets::AssetsPlugin, camera::CameraPlugin, player::PlayerPlugin};

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa::Off);
    app.add_plugins((DefaultPlugins, AssetsPlugin, CameraPlugin, PlayerPlugin));

    app.run();
}
