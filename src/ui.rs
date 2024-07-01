use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
};

use crate::{player::FlowerCount, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Loading).load_collection::<UiAssets>(),
        )
        .add_systems(OnEnter(GameState::Gaming), spawn_hud)
        .add_systems(OnExit(GameState::Gaming), cleanup_gameui)
        .add_systems(
            Update,
            update_flower_count.run_if(in_state(GameState::Gaming)),
        );
    }
}

#[derive(AssetCollection, Resource)]
struct UiAssets {
    #[asset(path = "fonts/RobotoMono-Regular.ttf")]
    font: Handle<Font>,
}

// Marker component that makes it easy to clean up this UI when it's no longer needed.
#[derive(Component)]
struct GameUi;

// Marker component for the text bundle that shows the flower count
#[derive(Component)]
struct FlowerCountText;

fn spawn_hud(mut commands: Commands, assets: Res<UiAssets>) {
    commands
        .spawn((
            GameUi,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(2.),
                    left: Val::Percent(2.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                FlowerCountText,
                TextBundle::from_section(
                    "0 Flowers",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 30.,
                        ..default()
                    },
                ),
            ));
        });
}

fn cleanup_gameui(mut commands: Commands, ui: Query<Entity, With<GameUi>>) {
    for entity in &ui {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_flower_count(
    mut ui: Query<&mut Text, With<FlowerCountText>>,
    updates: Query<&FlowerCount, Changed<FlowerCount>>,
) {
    // This will only return something on frames when the flower count is updated.
    let Ok(count) = updates.get_single() else {
        return;
    };

    let Ok(mut ui) = ui.get_single_mut() else {
        error!("unable to update flower count; cannot get UI text");
        return;
    };

    let flower_count_text = format!("{} Flowers", count.0);

    if ui.as_ref().sections[0].value != flower_count_text {
        ui.sections[0].value = flower_count_text;
    }
}
