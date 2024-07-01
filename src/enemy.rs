use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::assets::{AssetHandles, AssetsSet};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_flowers.after(AssetsSet::Loading));
    }
}

#[derive(Default, Component)]
pub struct Enemy;

const NUM_FLOWERS: usize = 50;
const WORLD_SIZE: f32 = 1024.;

fn init_flowers(mut commands: Commands, assets: Res<AssetHandles>) {
    let mut rng = thread_rng();

    for _ in 0..NUM_FLOWERS {
        // Get a location at a random offset from 0, 0
        let x = rng.gen_range(-WORLD_SIZE..WORLD_SIZE);
        let y = rng.gen_range(-WORLD_SIZE..WORLD_SIZE);
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
