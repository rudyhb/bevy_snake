use bevy::prelude::*;

use crate::constants::*;
use crate::game::OnGameScreen;
use crate::utils::despawn_screen;
use crate::GameState;

const GAME_OVER_STARTUP_DELAY: f32 = 1.0;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(game_over_setup))
            .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(game_over))
            .add_system_set(
                SystemSet::on_exit(GameState::GameOver)
                    .with_system(despawn_screen::<OnGameOverScreen>)
                    .with_system(despawn_screen::<OnGameScreen>),
            )
            .insert_resource(GameOverTimer(Timer::from_seconds(
                GAME_OVER_STARTUP_DELAY,
                false,
            )));
    }
}

// tag component
#[derive(Component)]
struct OnGameOverScreen;

struct GameOverTimer(Timer);

fn game_over_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<GameOverTimer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    timer.0.reset();
    let (title, subtitle) = ("Game Over", "Press any key to try again");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: GRAY_BACKDROP_COLOR.into(),
            ..default()
        })
        .insert(OnGameOverScreen)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(50.0)),
                    ..default()
                },
                text: Text::with_section(
                    title,
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.0,
                        color: TEXT_COLOR,
                    },
                    Default::default(),
                ),
                ..default()
            });

            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(30.0)),
                    ..default()
                },
                text: Text::with_section(
                    subtitle,
                    TextStyle {
                        font: font.clone(),
                        font_size: 36.0,
                        color: TEXT_COLOR,
                    },
                    Default::default(),
                ),
                ..default()
            });
        });
}

fn game_over(
    time: Res<Time>,
    mut timer: ResMut<GameOverTimer>,
    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if timer.0.tick(time.delta()).finished() {
        if mouse_button_input.pressed(MouseButton::Left) {
            info!("mouse click!");
        } else if keyboard_input.get_just_pressed().count() > 0 {
            info!("keyboard clicked!");
        } else {
            return;
        }
        game_state.set(GameState::Game).unwrap();
    }
}
