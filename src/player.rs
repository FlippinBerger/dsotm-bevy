use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// marker component for the Player
#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 30.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Spawning player");

    // let shape = meshes.add(Capsule3d::default());
    let shape = meshes.add(Sphere::default());

    commands.spawn((
        PbrBundle {
            mesh: shape,
            ..Default::default()
        },
        RigidBody::Dynamic,
        Velocity {
            linvel: Vec3::ZERO,
            angvel: Vec3::ZERO,
        },
        Sleeping::disabled(),
        Ccd::enabled(),
        Collider::ball(0.5),
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

fn jump(mut player_q: Query<&mut Velocity, With<Player>>, keys: Res<ButtonInput<KeyCode>>) {
    let mut vel = player_q.single_mut();
}
