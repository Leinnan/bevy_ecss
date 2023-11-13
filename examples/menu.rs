use std::marker::PhantomData;

use bevy::prelude::*;
use tomt_bevycss::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    PauseMenu,
}

#[derive(Debug, Copy, Clone, Default, Resource, Reflect)]
enum MainMenuSelection {
    #[default]
    None,
    NewGame,
    HighScores,
    Options,
    ExitGame,
}
impl std::fmt::Display for MainMenuSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::None => "NONE",
                Self::NewGame => "New Game",
                Self::HighScores => "High Scores",
                Self::Options => "Options",
                Self::ExitGame => "Exit Game",
            }
        )
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Component)]
pub struct SpawnedBy<T>(PhantomData<T>);

const CLASS_MAIN_MENU: &str = "main-menu";

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, BevyCssPlugin::default()))
        .register_type::<Class>()
        .register_type::<StyleSheet>();

    app.add_systems(Startup, spawn_camera);

    app.add_state::<GameState>()
        .register_type::<MainMenuSelection>()
        .add_systems(OnEnter(GameState::MainMenu), enter_main_menu)
        .add_systems(OnExit(GameState::MainMenu), exit_main_menu);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn enter_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.init_resource::<MainMenuSelection>();

    // UI root entity (CSS attached here)
    commands
        .spawn((
            Name::new("root_ui"),
            SpawnedBy::<MainMenuSelection>::default(),
            StyleSheet::new(asset_server.load("sheets/menu.css")),
            Class::new(CLASS_MAIN_MENU),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let mut spawn_btn = |action: MainMenuSelection| {
                        parent
                            .spawn(ButtonBundle::default())
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    action.to_string(),
                                    TextStyle::default(),
                                ));
                            });
                    };

                    spawn_btn(MainMenuSelection::NewGame);
                    spawn_btn(MainMenuSelection::HighScores);
                    spawn_btn(MainMenuSelection::Options);
                    spawn_btn(MainMenuSelection::ExitGame);
                });
        });
}

fn exit_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<SpawnedBy<MainMenuSelection>>>,
) {
    commands.remove_resource::<MainMenuSelection>();

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
