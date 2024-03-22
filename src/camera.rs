use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 24.0;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct DevCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cameras);
    }
}

fn spawn_cameras(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, CAMERA_DISTANCE)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        MainCamera,
    ));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, CAMERA_DISTANCE)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        DevCamera,
    ));
}

// press P to get into dev mode with a new camera you can move around
// press spacebar to get back to main camera
fn dev_camera(
    mut main_cam_q: Query<(&mut Camera, &Transform), With<MainCamera>>,
    mut dev_cam_q: Query<(&mut Camera, &mut Transform), With<DevCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        let (mut main, main_trans) = main_cam_q.single_mut();
        main.is_active = !main.is_active;

        let (mut dev, mut dev_trans) = dev_cam_q.single_mut();
        dev.is_active = true;

        // set us into a dev mode game state
    }
}
