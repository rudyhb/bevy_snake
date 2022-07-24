use bevy::log::{Level, LogSettings};
use bevy::prelude::*;

use crate::game::GamePlugin;
use crate::game_over::GameOverPlugin;
use crate::resources::setup_resources;

mod constants;
mod game;
mod game_over;
mod resources;
mod utils;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Game,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(LogSettings {
            level: Level::DEBUG,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=debug".to_string(),
        })
        .insert_resource(WindowDescriptor {
            width: 500.0,
            height: 500.0,
            title: "~ snake ~".to_string(),
            resizable: false,
            cursor_visible: false,
            ..default()
        })
        .add_startup_system(setup_resources)
        .insert_resource(ClearColor(constants::BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Game)
        .add_startup_system(setup)
        .add_plugin(GamePlugin)
        .add_plugin(GameOverPlugin)
        // .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
