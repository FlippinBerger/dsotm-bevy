use bevy::prelude::*;

use crate::player::Player;
use crate::state::GameState;

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
fn god_mode_toggle(
    mut main_cam_q: Query<(&mut Camera, &Transform), With<MainCamera>>,
    mut dev_cam_q: Query<(&mut Camera, &mut Transform), With<DevCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Run all of this off pressing the key code.
    if keys.just_pressed(KeyCode::KeyP) {
        // get the 2 cameras and their transforms
        let (mut main, main_trans) = main_cam_q.single_mut();
        let (mut dev, mut dev_trans) = dev_cam_q.single_mut();

        match state.get() {
            GameState::GodMode => {
                // set cameras back together and go back to main camera in Level Mode
                next_state.set(GameState::Level);
                *dev_trans = *main_trans;
            }
            _ => {
                next_state.set(GameState::GodMode);
            }
        }
    }
}

fn dev_cam_move(
    state: Res<State<GameState>>,
    mut cam: Query<&mut Transform, With<DevCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let curr_state = state.get();
    if *curr_state != GameState::GodMode {
        return;
    }

    // impl movement
}

fn main_cam_move(
    state: Res<State<GameState>>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
    player_trans: Query<&Transform, With<Player>>,
) {
    let curr_state = state.get();
    if *curr_state != GameState::Level {
        return;
    }

    let player_t = player_trans.single();
    let mut cam_t = cam.single_mut();

    cam_t.translation.x = player_t.translation.x;
}
