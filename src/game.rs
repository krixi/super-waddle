use bevy::prelude::*;

use crate::{assets::GameConfig, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), init_game_timer)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_objects)
            .add_systems(Update, tick_game_timer.run_if(in_state(GameState::Gaming)));
    }
}

#[derive(Debug, Deref, DerefMut, Resource)]
pub struct GameTimer(Timer);

impl GameTimer {
    fn new(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, TimerMode::Once))
    }
}

fn init_game_timer(mut commands: Commands, config: GameConfig) {
    let Some(config) = config.get() else {
        return;
    };
    commands.insert_resource(GameTimer::new(config.game_time_seconds));
}

fn tick_game_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut next: ResMut<NextState<GameState>>,
) {
    if timer.tick(time.delta()).just_finished() {
        commands.remove_resource::<GameTimer>();
        next.set(GameState::GameOver);
    }
}

#[derive(Default, Debug, Component)]
pub struct GameObject;

fn cleanup_game_objects(mut commands: Commands, objects: Query<Entity, With<GameObject>>) {
    for entity in &objects {
        commands.entity(entity).despawn_recursive();
    }
}
