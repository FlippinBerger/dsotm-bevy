use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// movement constants
const PLAYER_VELOCITY_X: f32 = 30.;
const PLAYER_VELOCITY_Y: f32 = 15.0;
const MAX_JUMP_HEIGHT: f32 = 7.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Jump(f32);

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_controller)
            .add_systems(Update, (player_movement, jump, variable_jump, fall));
    }
}

fn spawn_player_controller(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let shape = meshes.add(Sphere::default());

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
        player_t.translation.x -= PLAYER_VELOCITY_X * time.delta_seconds();
    }
    if keys.pressed(KeyCode::KeyD) {
        player_t.translation.x += PLAYER_VELOCITY_X * time.delta_seconds();
    }
}

// jumping

// the kinematic character controller doesn't respend to external forces, so
// need to apply gravity
fn fall(time: Res<Time>, mut q: Query<&mut KinematicCharacterController, Without<Jump>>) {
    if q.is_empty() {
        return;
    }

    let mut player = q.single_mut();

    let fall_speed = (PLAYER_VELOCITY_Y / 1.5) * time.delta_seconds() * -1.;

    match player.translation {
        Some(v) => player.translation = Some(Vec3::new(v.x, fall_speed, v.z)),
        None => player.translation = Some(Vec3::new(0., fall_speed, 0.)),
    }
}

fn jump(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    q: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if q.is_empty() {
        return;
    }

    let (player, output) = q.single();

    if input.pressed(KeyCode::Space) && output.grounded {
        commands.entity(player).insert(Jump(0.));
    }
}

fn variable_jump(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut player, mut jump_height) = q.single_mut();

    // still holding down the jump key
    if input.pressed(KeyCode::Space) {
        let mut movement = PLAYER_VELOCITY_Y * time.delta_seconds();

        if movement + jump_height.0 >= MAX_JUMP_HEIGHT {
            movement = MAX_JUMP_HEIGHT - jump_height.0;
            commands.entity(entity).remove::<Jump>();
        }

        jump_height.0 += movement;

        match player.translation {
            Some(v) => player.translation = Some(Vec3::new(v.x, movement, v.z)),
            None => player.translation = Some(Vec3::new(0., movement, 0.)),
        }
    }

    // jump key released, so let the player start falling
    if input.just_released(KeyCode::Space) {
        commands.entity(entity).remove::<Jump>();
    }
}
