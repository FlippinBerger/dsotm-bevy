use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let shape = meshes.add(Cuboid::default());

    commands.spawn((
        PbrBundle {
            mesh: shape,
            transform: Transform {
                translation: Vec3::new(0., -2.0, 0.),
                scale: Vec3::new(30., 1., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        RigidBody::Fixed,
    ));
}
