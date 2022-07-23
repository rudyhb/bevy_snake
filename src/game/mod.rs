use bevy::prelude::*;

use crate::constants::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[derive(Default)]
pub struct EatEvent;

#[derive(Default)]
pub struct DeathEvent;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeBody;

#[derive(Component)]
pub struct SnakeSegment;

struct EatingSound(Handle<AudioSource>);
struct GameOverSound(Handle<AudioSource>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // sound
    let eating_sound = asset_server.load("sounds/eat.ogg");
    commands.insert_resource(EatingSound(eating_sound));
    let game_over_sound = asset_server.load("sounds/game_over.ogg");
    commands.insert_resource(GameOverSound(game_over_sound));

    // snake
    commands
        .spawn()
        .insert(SnakeHead)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 1.0),
                ..default()
            },
            ..default()
        });
}

fn check_eat(mut eat_event: EventReader<EatEvent>, audio: Res<Audio>, sound: Res<EatingSound>) {
    for _ in eat_event.iter() {
        audio.play(sound.0.clone());
    }
}

fn check_death(
    mut death_event: EventReader<DeathEvent>,
    audio: Res<Audio>,
    sound: Res<GameOverSound>,
    mut snake_query: Query<&mut Sprite, With<SnakeSegment>>,
) {
    if death_event.iter().count() > 0 {
        audio.play(sound.0.clone());
        for mut segment in snake_query.iter_mut() {
            segment.color = DEAD_SNAKE_COLOR;
        }
    }
}
