use bevy::prelude::*;

use crate::constants::*;
use crate::game::OnGameScreen;
use crate::utils::despawn_screen;

pub struct GameOverPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Game,
    GameOver,
} //placeholder

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(game_over_setup))
            .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(game_over))
            .add_system_set(
                SystemSet::on_exit(GameState::GameOver)
                    .with_system(despawn_screen::<OnGameOverScreen>)
                    .with_system(despawn_screen::<OnGameScreen>),
            );
    }
}

// tag component
#[derive(Component)]
struct OnGameOverScreen;

fn game_over_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

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
                    "Game Over",
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
                    margin: Rect::all(Val::Px(50.0)),
                    ..default()
                },
                text: Text::with_section(
                    "Press any key to try again",
                    TextStyle {
                        font: font.clone(),
                        font_size: 46.0,
                        color: TEXT_COLOR,
                    },
                    Default::default(),
                ),
                ..default()
            });
        });
}

fn game_over(interaction_query: Query<&Interaction>, mut game_state: ResMut<State<GameState>>) {
    if let Some(interaction) = interaction_query.iter().next() {
        info!("interaction: {:?}", interaction);
        game_state.set(GameState::Game).unwrap();
    }
}
