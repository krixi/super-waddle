use bevy::prelude::*;

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera)
            .add_systems(PostUpdate, follow_player);
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: Color::rgb(0.3f32, 0.3f32, 0.3f32).into(),
                ..default()
            },
            ..default()
        },
        VisibilityBundle::default(),
    ));
}

fn follow_player(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let Ok(mut camera_transform) = camera.get_single_mut() else {
        return;
    };
    let Ok(player) = player.get_single() else {
        return;
    };
    let player = player.translation;
    if camera_transform.as_ref().translation == player {
        return;
    }
    camera_transform.translation = player;
}
