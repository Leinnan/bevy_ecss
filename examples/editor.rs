use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_ecss::{
    prelude::{Class, EcssPlugin, StyleSheet},
    StyleSheetAsset,
};
use bevy_egui::{
    EguiContexts, EguiPlugin,
};

#[derive(Resource)]
struct EditorTextCss {
    text: String,
}

impl Default for EditorTextCss {
    fn default() -> Self {
        Self {
            text: include_str!("../assets/sheets/simple_ui.css").to_string(),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_string()),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<EditorTextCss>()
        .add_plugins(EguiPlugin)
        .add_plugins(EcssPlugin::default())
        .add_systems(Startup, (setup,add_ui.after(setup)))
        .add_systems(
            Update,
            editor_ui.run_if(input_toggle_active(true, KeyCode::F1)),
        )
        .run();
}

fn setup(mut commands: Commands){
    // Camera
    commands.spawn(Camera2dBundle::default());
}


fn add_ui(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<Entity,With<StyleSheet>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(StyleSheet::new(asset_server.load("sheets/simple_ui.css")))
        .insert(Name::new("ui-root"))
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .insert(Name::new("left-border"))
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .insert(Name::new("left-bg"))
                        .with_children(|parent| {
                            // text
                            parent
                                .spawn(
                                    TextBundle::from_section(
                                        "Text Example",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    }),
                                )
                                .insert(Name::new("left-text"));
                        });
                });
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(200.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(Name::new("right-border"))
                .with_children(|parent| {
                    // Title
                    parent
                        .spawn(
                            TextBundle::from_section(
                                "Scrolling list",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 25.,
                                    color: Color::WHITE,
                                },
                            )
                            .with_style(Style {
                                height: Val::Px(25.0),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    ..default()
                                },
                                ..default()
                            }),
                        )
                        .insert(Name::new("right-bg"));
                    // List with hidden overflow
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_self: AlignSelf::Center,
                                width: Val::Percent(100.0),
                                height: Val::Percent(50.0),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .insert(Name::new("right-list"))
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::ColumnReverse,
                                        flex_grow: 1.0,
                                        ..default()
                                    },
                                    background_color: Color::NONE.into(),
                                    ..default()
                                })
                                .insert(Name::new("right-moving-panel"))
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent
                                            .spawn(
                                                TextBundle::from_section(
                                                    format!("Item {i}"),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 20.,
                                                        color: Color::WHITE,
                                                    },
                                                )
                                                .with_style(Style {
                                                    flex_shrink: 0.,
                                                    height: Val::Px(20.),
                                                    margin: UiRect {
                                                        left: Val::Auto,
                                                        right: Val::Auto,
                                                        ..default()
                                                    },
                                                    ..default()
                                                }),
                                            )
                                            .insert(Class::new("big-text"))
                                            .insert(Name::new(format!("right-item-{}", i)));
                                    }
                                });
                        });
                });
            // absolute positioning
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(210.0),
                        bottom: Val::Px(10.0),
                        border: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.4, 0.4, 1.0).into(),
                    ..default()
                })
                .insert(Name::new("mid-blue-border"))
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::rgb(0.8, 0.8, 1.0).into(),
                            ..default()
                        })
                        .insert(Name::new("mid-navy-blue-content"));
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .insert(Name::new("mid-red-last"))
                .insert(Class::new("blue-bg container"))
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                                ..default()
                            },
                            background_color: Color::rgb(1.0, 0.0, 0.0).into(),
                            ..default()
                        })
                        .insert(Name::new("mid-red-last-but-one"))
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(100.0),
                                        height: Val::Px(100.0),
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(20.0),
                                        bottom: Val::Px(20.0),
                                        ..default()
                                    },
                                    background_color: Color::rgb(1.0, 0.3, 0.3).into(),
                                    ..default()
                                })
                                .insert(Name::new("mid-red-center"));
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(100.0),
                                        height: Val::Px(100.0),
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(40.0),
                                        bottom: Val::Px(40.0),
                                        ..default()
                                    },
                                    background_color: Color::rgb(1.0, 0.5, 0.5).into(),
                                    ..default()
                                })
                                .insert(Class::new("blue-bg"))
                                .insert(Name::new("mid-red-top-but-one"));
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(100.0),
                                        height: Val::Px(100.0),
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(60.0),
                                        bottom: Val::Px(60.0),
                                        ..default()
                                    },
                                    background_color: Color::rgb(1.0, 0.7, 0.7).into(),
                                    ..default()
                                })
                                .insert(Name::new("mid-red-top"));
                            // alpha test
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(100.0),
                                        height: Val::Px(100.0),
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(80.0),
                                        bottom: Val::Px(80.0),
                                        ..default()
                                    },
                                    background_color: Color::rgba(1.0, 0.9, 0.9, 0.4).into(),
                                    ..default()
                                })
                                .insert(Class::new("blue-bg"))
                                .insert(Name::new("mid-red-alpha"));
                        });
                });
            // bevy logo (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .insert(Name::new("mid-bevy-logo-bg"))
                .with_children(|parent| {
                    // bevy logo (image)
                    parent
                        .spawn(ImageBundle {
                            style: Style {
                                width: Val::Px(500.0),
                                ..default()
                            },
                            image: asset_server.load("branding/bevy_logo_dark_big.png").into(),
                            ..default()
                        })
                        .insert(Name::new("mid-bevy-logo-image"));
                });
        });
}

fn editor_ui(
    mut contexts: EguiContexts,
    mut editor_text: ResMut<EditorTextCss>,
    mut stylesheets: ResMut<Assets<StyleSheetAsset>>,
    mut stylesheet: Query<&mut StyleSheet>,
) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("Editor")
        // egui::SidePanel::left("left_panel")
        .resizable(true)
        .default_width(250.0)
        .show(ctx, |ui| {
            ui.add_space(10.0);
            // ui.heading(RichText::new("Editor").strong());
            ui.label(egui::RichText::new("Press F1 to toggle UI").small());
            if ui.button("Generate CSS").clicked() {
                let style = StyleSheetAsset::parse("EDITOR", &editor_text.text);
                let handle = stylesheets.add(style);

                let mut style_object = stylesheet.get_single_mut().unwrap();
                style_object.set(handle);
            }
            ui.add_space(15.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut editor_text.text)
                        .code_editor()
                        .hint_text("Enter CSS here"),
                );
            });
        });
}
