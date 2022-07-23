use bevy::log::{Level, LogSettings};
use bevy::prelude::*;

use crate::game::GamePlugin;

mod constants;
mod game;
mod game_over;
mod utils;

fn main() {
    App::new()
        .insert_resource(LogSettings {
            level: Level::DEBUG,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
        })
        .insert_resource(WindowDescriptor {
            width: 500.0,
            height: 500.0,
            title: "~ snake ~".to_string(),
            resizable: false,
            cursor_visible: false,
            ..default()
        })
        .insert_resource(ClearColor(constants::BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(GamePlugin)
        .add_system(bevy::input::system::exit_on_esc_system) //TODO: remove
        .run();
}

fn setup(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
