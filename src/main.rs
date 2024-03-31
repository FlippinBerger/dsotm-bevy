use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod camera;
mod collectable;
mod level;
mod player;
mod player_controller;
mod state;

fn main() {
    println!("Running Dark Side of the Moon");

    App::new()
        .init_state::<state::GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<()>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(camera::CameraPlugin)
        .add_plugins(level::planet::PlanetPlugin)
        // .add_plugins(player::PlayerPlugin)
        .add_plugins(player_controller::PlayerControllerPlugin)
        .insert_resource(ClearColor(Color::BISQUE))
        .run();
}
