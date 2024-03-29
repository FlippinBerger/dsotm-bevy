use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const PLAYER_SPEED: f32 = 30.;

#[derive(Component)]
pub struct Player;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_controller)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player_controller(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let shape = meshes.add(Capsule3d::default());

    commands.spawn((
        PbrBundle {
            mesh: shape,
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::ball(0.5),
        KinematicCharacterController::default(),
        Player,
    ));
}

fn player_movement(
    mut player_q: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut player_t = player_q.single_mut();
    if keys.pressed(KeyCode::KeyA) {
        player_t.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::KeyD) {
        player_t.translation.x += PLAYER_SPEED * time.delta_seconds();
    }
}
