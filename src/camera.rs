use bevy::prelude::*;

use crate::player_controller::Player;
use crate::state::GameState;

const CAMERA_DISTANCE: f32 = 24.;
const GOD_MODE_CAMERA_SPEED: f32 = 30.;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct GodCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cameras)
            .add_systems(Update, (god_mode_toggle, god_cam_move, main_cam_move));
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
            camera: Camera {
                is_active: false,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, CAMERA_DISTANCE)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        GodCamera,
    ));
}

// press P to get into dev mode with a new camera you can move around
// press spacebar to get back to main camera
fn god_mode_toggle(
    mut main_cam_q: Query<(&mut Camera, &Transform), With<MainCamera>>,
    mut god_cam_q: Query<(&mut Camera, &mut Transform), (With<GodCamera>, Without<MainCamera>)>,
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Run all of this off pressing the key code.
    if keys.just_pressed(KeyCode::KeyP) {
        // get the 2 cameras and their transforms
        let (mut main, main_t) = main_cam_q.single_mut();
        let (mut god, mut god_t) = god_cam_q.single_mut();

        match state.get() {
            GameState::GodMode => {
                // set cameras back together and go back to main camera in Level Mode
                next_state.set(GameState::Level);
                main.is_active = true;
                god.is_active = false;
            }
            _ => {
                next_state.set(GameState::GodMode);
                main.is_active = false;
                god.is_active = true;
                *god_t = *main_t;
            }
        }
    }
}

fn god_cam_move(
    state: Res<State<GameState>>,
    time: Res<Time>,
    mut cam: Query<&mut Transform, With<GodCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let curr_state = state.get();
    if *curr_state != GameState::GodMode {
        return;
    }

    let mut cam_t = cam.single_mut();

    // impl movement
    if keys.pressed(KeyCode::ArrowLeft) {
        cam_t.translation.x -= GOD_MODE_CAMERA_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::ArrowRight) {
        cam_t.translation.x += GOD_MODE_CAMERA_SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::ArrowDown) {
        cam_t.translation.y -= GOD_MODE_CAMERA_SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::ArrowUp) {
        cam_t.translation.y += GOD_MODE_CAMERA_SPEED * time.delta_seconds();
    }
}

fn main_cam_move(
    state: Res<State<GameState>>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
    player_trans: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let curr_state = state.get();
    if *curr_state != GameState::Level {
        return;
    }

    let player_t = player_trans.single();
    let mut cam_t = cam.single_mut();

    cam_t.translation.x = cam_t.translation.x.lerp(player_t.translation.x, 0.2);

    // let distance = player_t.translation.x - cam_t.translation.x;

    // if distance.abs() > 2.0 {
    //     if distance < 0. {
    //         // implement easing here
    //         // cam_t.translation.x = player_t.translation.x - 2.0;
    //         cam_t.translation.x = cam_t.translation.x.lerp(player_t.translation.x - 2.0, 0.5);
    //     } else {
    //         cam_t.translation.x = cam_t.translation.x.lerp(player_t.translation.x + 2.0, 0.5);
    //     }
    // }
}
