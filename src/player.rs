use bevy::prelude::*;

use crate::{
    assets::{AssetHandles, AssetsSet},
    input::{InputEvent, InputSet},
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

fn move_player(
    mut input_events: EventReader<InputEvent>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    for event in input_events.read() {
        let Some(direction) = event.direction() else {
            continue;
        };

        let transform = player.mul_transform(Transform::from_translation(
            (direction * 10.0f32).extend(0.0f32),
        ));

        *player = transform;
    }
}
