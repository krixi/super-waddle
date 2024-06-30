use bevy::prelude::*;

use crate::assets::{AssetHandles, LoadingComplete};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum PlayerSet {
    Move,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Default, Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetHandles>,
    mut loading_complete: EventReader<LoadingComplete>,
) {
    for _ in loading_complete.read() {
        commands.spawn((
            Player,
            SpriteBundle {
                texture: assets.goose.clone(),
                ..default()
            },
        ));
    }
}
