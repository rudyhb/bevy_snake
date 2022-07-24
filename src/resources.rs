use bevy::prelude::*;

pub struct EatingSound(pub Handle<AudioSource>);

pub struct GameOverSound(pub Handle<AudioSource>);

pub fn setup_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // sound
    let eating_sound = asset_server.load("sounds/eat.ogg");
    commands.insert_resource(EatingSound(eating_sound));
    let game_over_sound = asset_server.load("sounds/game_over.ogg");
    commands.insert_resource(GameOverSound(game_over_sound));
}