use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Bundle)]
pub struct PlatformBundle {
    pbr: PbrBundle,
    rb: RigidBody,
    col: Collider,
}

impl PlatformBundle {
    pub fn new(shape: Handle<Mesh>, translation: Vec3, scale: Vec3) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: shape,
                transform: Transform {
                    translation,
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            rb: RigidBody::Fixed,
            // TODO figure out how to calculate collider sizes
            col: Collider::cuboid(0.5, 0.5, 0.5),
        }
    }
}
