use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera);
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
