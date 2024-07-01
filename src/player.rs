use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
};

use crate::{
    assets::GameConfig,
    enemy::{EnemySet, PickFlower},
    input::{InputSet, InputState},
    GameState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Loading).load_collection::<PlayerAssets>(),
        )
        .add_systems(OnEnter(GameState::Gaming), spawn_player)
        .add_systems(
            Update,
            (
                move_player.after(InputSet::ReadInput),
                count_picked_flowers.after(EnemySet::Collisions),
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct FlowerCount(pub i32);

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(path = "sprites/goose.png")]
    goose: Handle<Image>,
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands.spawn((
        Player,
        FlowerCount(0),
        SpriteBundle {
            texture: assets.goose.clone(),
            ..default()
        },
    ));
}

fn move_player(
    config: GameConfig,
    time: Res<Time>,
    input: Res<InputState>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let Some(config) = config.get() else {
        return;
    };

    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let direction = input.normalized_direction();
    if direction == Vec2::ZERO {
        return;
    }

    let transform = player.mul_transform(Transform::from_translation(
        (direction * config.player_move_speed * time.delta_seconds()).extend(0.0f32),
    ));

    *player = transform;
}

fn count_picked_flowers(
    mut player: Query<&mut FlowerCount, With<Player>>,
    mut events: EventReader<PickFlower>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };
    for _ in events.read() {
        player.0 += 1;
    }
}
