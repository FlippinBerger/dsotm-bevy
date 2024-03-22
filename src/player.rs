use bevy::prelude::*;

// marker component for the Player
#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Spawning player");

    let shape = meshes.add(Capsule3d::default());

    commands.spawn((
        PbrBundle {
            mesh: shape,
            ..Default::default()
        },
        Player,
    ));
}
