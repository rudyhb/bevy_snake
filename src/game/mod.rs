use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::prelude::random;

use crate::constants::*;
use crate::game::input::{InputDirection, InputPlugin, InputValue};

const ARENA_WIDTH: u32 = 15;
const ARENA_HEIGHT: u32 = 15;
const TIME_STEP: f64 = 0.150;

mod input;

pub struct GamePlugin;

// tag component
#[derive(Component)]
pub struct OnGameScreen;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set(SystemSet::new().with_system(setup))
            .add_plugin(InputPlugin)
            .add_event::<EatEvent>()
            .add_event::<DeathEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP))
                    .with_system(fruit_spawner)
                    .with_system(snake_movement)
                    .with_system(snake_eating.after(snake_movement))
                    .with_system(check_eat.after(snake_eating))
                    .with_system(check_death.after(snake_movement)),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(position_translation)
                    .with_system(size_scaling),
            )
            .insert_resource(SnakeSegments::default())
            .insert_resource(LastTailPosition::default())
            .insert_resource(GameIsOver::default());
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
pub struct SnakePart;

#[derive(Component)]
struct Fruit;

#[derive(Default, Deref, DerefMut)]
struct SnakeSegments(Vec<Entity>);

struct LastTailPosition(Position);

#[derive(Default)]
struct GameIsOver(bool);

impl Default for LastTailPosition {
    fn default() -> Self {
        Self(Position { x: 0, y: 0 })
    }
}

struct EatingSound(Handle<AudioSource>);

struct GameOverSound(Handle<AudioSource>);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut segments: ResMut<SnakeSegments>,
) {
    info!("setting up game");
    // sound
    let eating_sound = asset_server.load("sounds/eat.ogg");
    commands.insert_resource(EatingSound(eating_sound));
    let game_over_sound = asset_server.load("sounds/game_over.ogg");
    commands.insert_resource(GameOverSound(game_over_sound));

    // snake
    *segments = SnakeSegments(vec![
        commands
            .spawn()
            .insert(OnGameScreen)
            .insert(SnakeHead)
            .insert(SnakePart)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(1.0))
            .id(),
        spawn_segment(&mut commands, Position { x: 2, y: 3 }),
    ]);
}

fn snake_movement(
    mut input: ResMut<InputValue>,
    segments: ResMut<SnakeSegments>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut death_writer: EventWriter<DeathEvent>,
    mut game_is_over: ResMut<GameIsOver>,
) {
    debug!("snake movement");
    if segments.0.is_empty() || game_is_over.0 {
        return;
    }
    let old_positions = segments
        .0
        .iter()
        .map(|e| *positions.get(*e).unwrap())
        .collect::<Vec<Position>>();
    let mut position = positions
        .get_mut(*segments.0.iter().next().unwrap())
        .unwrap();
    match input.next() {
        InputDirection::Up => {
            position.y += 1;
        }
        InputDirection::Down => {
            position.y -= 1;
        }
        InputDirection::Left => {
            position.x -= 1;
        }
        InputDirection::Right => {
            position.x += 1;
        }
    }
    if position.x < 0
        || position.y < 0
        || position.x as u32 >= ARENA_WIDTH
        || position.y as u32 >= ARENA_HEIGHT
        || old_positions.contains(&position)
    {
        death_writer.send(DeathEvent);
        game_is_over.0 = true;
    }
    old_positions
        .iter()
        .zip(segments.iter().skip(1))
        .for_each(|(pos, segment)| {
            *positions.get_mut(*segment).unwrap() = *pos;
        });
    *last_tail_position = LastTailPosition(*old_positions.last().unwrap());
}

fn snake_eating(
    mut commands: Commands,
    mut eating_writer: EventWriter<EatEvent>,
    fruit_position: Query<(Entity, &Position), With<Fruit>>,
    head_position: Query<&Position, With<SnakeHead>>,
) {
    let head_pos = head_position.single();
    if let Ok((ent, fruit_pos)) = fruit_position.get_single() {
        if head_pos == fruit_pos {
            commands.entity(ent).despawn();
            eating_writer.send(EatEvent);
        }
    }
}

fn check_eat(
    mut eat_event: EventReader<EatEvent>,
    mut commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    audio: Res<Audio>,
    sound: Res<EatingSound>,
) {
    for _ in eat_event.iter() {
        segments.push(spawn_segment(&mut commands, last_tail_position.0));
        audio.play(sound.0.clone());
    }
}

fn check_death(
    mut death_event: EventReader<DeathEvent>,
    audio: Res<Audio>,
    sound: Res<GameOverSound>,
    mut snake_query: Query<&mut Sprite, With<SnakePart>>,
) {
    if death_event.iter().count() > 0 {
        audio.play(sound.0.clone());
        for mut segment in snake_query.iter_mut() {
            segment.color = DEAD_SNAKE_COLOR;
        }
    }
}

fn spawn_segment(commands: &mut Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(OnGameScreen)
        .insert(SnakePart)
        .insert(SnakeBody)
        .insert(position)
        .insert(Size::square(0.95))
        .id()
}

fn fruit_spawner(
    existing: Query<&Fruit>,
    mut commands: Commands,
    snake_positions: Query<&Position, With<SnakePart>>,
) {
    if existing.iter().count() == 0 {
        let pos: Position = loop {
            let pos = Position {
                x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
            };
            if snake_positions.iter().all(|p| *p != pos) {
                break pos;
            }
        };
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: FRUIT_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(OnGameScreen)
            .insert(Fruit)
            .insert(pos)
            .insert(Size::square(0.6));
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(
    windows: Res<Windows>,
    mut q: Query<(&Position, &mut Transform, Option<&SnakePart>)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform, maybe_snake_part) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            if maybe_snake_part.is_some() { 1.0 } else { 0.0 },
        );
    }
}
