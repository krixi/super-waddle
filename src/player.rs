use bevy::prelude::*;

use crate::{
    assets::{AssetHandles, AssetsSet},
    input::{InputSet, InputState},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player.after(AssetsSet::Loading))
            .add_systems(Update, move_player.after(InputSet::ReadInput));
    }
}

#[derive(Default, Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, assets: Res<AssetHandles>) {
    commands.spawn((
        Player,
        SpriteBundle {
            texture: assets.goose.clone(),
            ..default()
        },
    ));
}

const PLAYER_MOVE_SPEED: f32 = 25.;

fn move_player(
    time: Res<Time>,
    input: Res<InputState>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let direction = input.normalized_direction();
    if direction == Vec2::ZERO {
        return;
    }

    let transform = player.mul_transform(Transform::from_translation(
        (direction * PLAYER_MOVE_SPEED * time.delta_seconds()).extend(0.0f32),
    ));

    *player = transform;
}
