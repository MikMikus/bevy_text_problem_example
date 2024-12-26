use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_pancam::{PanCam, PanCamPlugin};

#[derive(Resource, Default)]
struct TextDisplayMode(pub TextMode);

#[derive(Default, PartialEq, Clone, Copy, Debug)]
enum TextMode {
    #[default]
    Text2dMultiSections,
    Text2dOneSection,
    Hidden,
    None,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PanCamPlugin,
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .init_resource::<TextDisplayMode>()
        .add_systems(Startup, setup)
        .add_systems(Update, (despawn_text, spawn_text, change_text_display_mode))
        .run();
}

const COLS_NUM: usize = 48;
const ROWS_NUM: usize = 27;

const LABEL_BODY_A: f32 = 100.;
const LABEL_BODY_R: f32 = 50.;
const MARGIN: f32 = 10.;

const FONT_SIZE: f32 = 24.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 2d camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                (LABEL_BODY_A + LABEL_BODY_R * 2. + MARGIN) * (COLS_NUM + 1) as f32 * 0.5,
                (LABEL_BODY_R * 2. + MARGIN) * ROWS_NUM as f32 * 0.5,
                0.,
            ),
            projection: OrthographicProjection {
                scale: 8.,
                ..Camera2dBundle::default().projection
            },
            ..Camera2dBundle::default()
        },
        PanCam {
            min_scale: 1.0,
            ..PanCam::default()
        },
    ));

    // labels body
    let shape_mesh_handel = Mesh2dHandle(meshes.add(Capsule2d::new(LABEL_BODY_R, LABEL_BODY_A)));
    for i in 1..=COLS_NUM {
        let x_pos = (LABEL_BODY_A + LABEL_BODY_R * 2. + MARGIN) * i as f32;
        for j in 1..=ROWS_NUM {
            let y_pos = (LABEL_BODY_R * 2. + MARGIN) * j as f32;
            let color = Color::hsl(360. * i as f32 / j as f32, 0.95, 0.7);

            commands.spawn(MaterialMesh2dBundle {
                mesh: shape_mesh_handel.clone(),
                material: materials.add(color),
                transform: Transform::from_xyz(x_pos, y_pos, 1.0)
                    .with_rotation(Quat::from_rotation_z(1.571)),
                ..default()
            });
        }
    }
}

fn change_text_display_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut display_mode_res: ResMut<TextDisplayMode>,
) {
    if keys.just_pressed(KeyCode::Space) {
        display_mode_res.0 = match display_mode_res.0 {
            TextMode::Text2dMultiSections => TextMode::Text2dOneSection,
            TextMode::Text2dOneSection => TextMode::Hidden,
            TextMode::Hidden => TextMode::None,
            TextMode::None => TextMode::Text2dMultiSections,
        };

        println!("Change to text mode: {:?}", display_mode_res.0);
    }
}

fn despawn_text(
    mut commands: Commands,
    display_mode_res: Res<TextDisplayMode>,
    text_query: Query<Entity, With<Text>>,
    mut last_mode: Local<TextMode>,
) {
    if display_mode_res.0 != *last_mode {
        for entity in text_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        *last_mode = display_mode_res.0;
    }
}

fn spawn_text(
    mut commands: Commands,
    display_mode_res: Res<TextDisplayMode>,
    body_query: Query<(Entity, Option<&Children>), With<Mesh2dHandle>>,
    text_query: Query<Entity, With<Text>>,
) {
    if display_mode_res.0 != TextMode::None {
        for (body_entity, children) in body_query.iter() {
            // only for labels without text
            if !children.is_some_and(|ch| ch.iter().any(|c| text_query.get(*c).is_ok())) {
                if let Some(text_entity) = match display_mode_res.0 {
                    TextMode::Text2dMultiSections => Some(TextBuilder::spawn_text_multi_sections(
                        &mut commands,
                        &["Lorem", "\nipsum", "\ndolor"],
                    )),
                    TextMode::Text2dOneSection => Some(TextBuilder::spawn_text(
                        &mut commands,
                        "Lorem\nipsum\ndolor",
                    )),
                    TextMode::Hidden => Some(TextBuilder::spawn_hidden_text(&mut commands, &[""])),
                    TextMode::None => None,
                } {
                    commands
                        .entity(text_entity)
                        .insert(
                            Transform::from_xyz(0., 0., 2.)
                                .with_rotation(Quat::from_rotation_z(-1.571)),
                        )
                        .set_parent(body_entity);
                }
            }
        }
    }
}

struct TextBuilder;

impl TextBuilder {
    pub fn spawn_text_multi_sections(commands: &mut Commands, text_sections: &[&str]) -> Entity {
        let sections: Vec<TextSection> = text_sections
            .iter()
            .map(|t| TextSection {
                value: t.to_string(),
                style: TextStyle {
                    font_size: FONT_SIZE,
                    ..TextStyle::default()
                },
            })
            .collect();
        commands
            .spawn(Text2dBundle {
                text: Text::from_sections(sections).with_justify(JustifyText::Center),
                ..default()
            })
            .id()
    }

    pub fn spawn_text(commands: &mut Commands, text: &str) -> Entity {
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font_size: FONT_SIZE,
                        ..TextStyle::default()
                    },
                )
                .with_justify(JustifyText::Center),
                ..default()
            })
            .id()
    }

    pub fn spawn_hidden_text(commands: &mut Commands, text_sections: &[&str]) -> Entity {
        let text_entity = Self::spawn_text_multi_sections(commands, text_sections);
        commands.entity(text_entity).insert(Visibility::Hidden).id()
    }
}