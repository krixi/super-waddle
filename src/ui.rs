use bevy::{app::AppExit, ecs::system::EntityCommands, prelude::*};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
};

use crate::{
    game::GameTimer,
    player::{FlowerCount, Player},
    GameState,
};

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
            (update_flower_count, update_game_timer).run_if(in_state(GameState::Gaming)),
        )
        .add_systems(OnEnter(GameState::GameOver), spawn_game_over_ui)
        .add_systems(OnExit(GameState::GameOver), cleanup_gameui)
        .add_systems(
            Update,
            handle_button_interaction.run_if(in_state(GameState::GameOver)),
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

// Marker component for the text bundle that shows the countdown timer
#[derive(Component)]
struct GameTimerText;

fn spawn_hud(mut commands: Commands, assets: Res<UiAssets>) {
    commands
        .spawn((
            GameUi,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
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

            parent.spawn((
                GameTimerText,
                TextBundle::from_section(
                    "Ready...",
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

fn update_game_timer(mut ui: Query<&mut Text, With<GameTimerText>>, timer: Res<GameTimer>) {
    let Ok(mut ui) = ui.get_single_mut() else {
        error!("unable to get game timer text to update");
        return;
    };

    let timer_text = format!("{:.1}s", timer.remaining_secs());

    if ui.as_ref().sections[0].value != timer_text {
        ui.sections[0].value = timer_text;
    }
}

#[derive(Component)]
pub enum GameOverButtonChoice {
    Retry,
    Exit,
}

fn spawn_game_over_ui(
    mut commands: Commands,
    player: Query<&FlowerCount, With<Player>>,
    assets: Res<UiAssets>,
) {
    let Ok(player) = player.get_single() else {
        error!("unable to get player to display game over ui");
        return;
    };

    let flower_count = player.0;

    commands
        .spawn((
            GameUi,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    top: Val::Percent(25.),
                    left: Val::Percent(50.),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Game Over",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.,
                    ..default()
                },
            ));

            let message = format!("{} flowers collected", flower_count);
            parent.spawn(TextBundle::from_section(
                message,
                TextStyle {
                    font: assets.font.clone(),
                    ..default()
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Two buttons
                    spawn_button(parent)
                        .insert(GameOverButtonChoice::Retry)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Retry",
                                TextStyle {
                                    font: assets.font.clone(),
                                    ..default()
                                },
                            ));
                        });

                    spawn_button(parent)
                        .insert(GameOverButtonChoice::Exit)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Exit",
                                TextStyle {
                                    font: assets.font.clone(),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

// Spawns a button with a default style
fn spawn_button<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn(ButtonBundle {
        style: Style {
            padding: UiRect::new(Val::Px(20.), Val::Px(20.), Val::Px(12.), Val::Px(12.)),
            border: UiRect::all(Val::Px(2.)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        border_color: Color::rgb(0.2, 0.2, 0.2).into(),
        ..default()
    })
}

fn handle_button_interaction(
    buttons: Query<(&Interaction, &GameOverButtonChoice), Changed<Interaction>>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, choice) in &buttons {
        if let Interaction::Pressed = interaction {
            match choice {
                GameOverButtonChoice::Retry => {
                    next.set(GameState::Gaming);
                }
                GameOverButtonChoice::Exit => {
                    exit.send(AppExit);
                }
            }
        }
    }
}
