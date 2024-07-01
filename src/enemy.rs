use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
};
use rand::{thread_rng, Rng};

use crate::{assets::GameConfig, GameState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Loading).load_collection::<EnemyAssets>(),
        )
        .add_systems(OnEnter(GameState::Gaming), init_flowers);
    }
}

#[derive(Default, Component)]
pub struct Enemy;

#[derive(AssetCollection, Resource)]
pub struct EnemyAssets {
    #[asset(path = "sprites/spike_flower.png")]
    spike_flower: Handle<Image>,
}

fn init_flowers(mut commands: Commands, config: GameConfig, assets: Res<EnemyAssets>) {
    let Some(config) = config.get() else {
        error!("unable to get config file; not spawning flowers");
        return;
    };

    let mut rng = thread_rng();

    let world_size = config.world_size;

    info!(
        "Initializing {} flowers in world size {}x{}",
        config.num_flowers, world_size, world_size
    );

    for _ in 0..config.num_flowers {
        // Get a location at a random offset from 0, 0
        let x = rng.gen_range(-world_size..world_size);
        let y = rng.gen_range(-world_size..world_size);
        let angle = rng.gen_range(0.0f32..TAU);
        let rot = Quat::from_axis_angle(Vec3::Z, angle);

        commands.spawn((
            Enemy,
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(x, y, 0.)).with_rotation(rot),
                texture: assets.spike_flower.clone(),
                ..default()
            },
        ));
    }
}
