use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod camera;
mod planet;
mod player;

fn main() {
    println!("Running Dark Side of the Moon");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<()>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(camera::CameraPlugin)
        .add_plugins(planet::PlanetPlugin)
        .add_plugins(player::PlayerPlugin)
        .insert_resource(ClearColor(Color::BISQUE))
        .run();
}
