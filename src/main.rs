mod components;
mod gameplay;
mod input;

use bevy::app::AppExit;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
//use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::render_resource::encase::rts_array::Length;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use rand::prelude::*;

pub use crate::components::*;
pub use crate::gameplay::*;
pub use crate::input::*;

//use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const NUMBER_ENTITIES: usize = 10_000;
pub const COLOR_SELECTION: [Color; 5] = [
    Color::RED,
    Color::YELLOW,
    Color::GREEN,
    Color::BLUE,
    Color::PURPLE,
];
pub const CIRCLE_RADIUS: f32 = 20.;

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct ColorResource([(Handle<ColorMaterial>, String, Color); 5]);

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct MeshResource(Mesh2dHandle);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    GameStart,
    InGame,
    NextRound,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<AppState>()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), /* .set(LogPlugin {
                                                                    level: bevy::log::Level::INFO,
                                                                    filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                                                                }),*/
        )
        /* .add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))*/
        .add_plugins(InputPlugin)
        .add_plugins(GameplayPlugin)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Menu), menu_setup)
        .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
        /*      .add_systems(
            Update,
            instructions.run_if(in_state(AppState::Instructions)),
        )*/
        .add_systems(Update, move_circles)
        //.add_plugins(WorldInspectorPlugin::new())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window>,
) {
    let mut rng = rand::thread_rng();

    let window = window_query.single();

    let window_height = window.height();
    let window_width = window.width() + (CIRCLE_RADIUS * 2.);

    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(window_width / 2., window_height / 2., 1000.),
            ..default()
        },
        ..default()
    });

    let mesh: Mesh2dHandle = meshes.add(shape::Circle::new(CIRCLE_RADIUS).into()).into();
    let mesh_resource = MeshResource { 0: mesh.clone() };

    commands.insert_resource(mesh_resource);

    /* let mesh: Mesh2dHandle = meshes
    .add(
        (shape::Circle {
            radius: 20.,
            vertices: 16,
        })
        .into(),
    )
    .into(); */

    let material_red = materials.add(ColorMaterial::from(Color::RED));
    let material_yellow = materials.add(ColorMaterial::from(Color::YELLOW));
    let material_green = materials.add(ColorMaterial::from(Color::GREEN));
    let material_blue = materials.add(ColorMaterial::from(Color::BLUE));
    let material_purple = materials.add(ColorMaterial::from(Color::PURPLE));

    let color_handles = [
        material_red.clone(),
        material_yellow.clone(),
        material_green.clone(),
        material_blue.clone(),
        material_purple.clone(),
    ];

    let color_resource = [
        (material_red.clone(), "RED".to_string(), Color::RED),
        (material_yellow.clone(), "YELLOW".to_string(), Color::YELLOW),
        (material_green.clone(), "GREEN".to_string(), Color::GREEN),
        (material_blue.clone(), "BLUE".to_string(), Color::BLUE),
        (material_purple.clone(), "PURPLE".to_string(), Color::PURPLE),
    ];

    let color_resource = ColorResource { 0: color_resource };

    commands.insert_resource(color_resource);

    for color in 0..color_handles.length() {
        let circle_color = &color_handles[color].clone();
        for _ in 0..NUMBER_ENTITIES / color_handles.length() {
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: circle_color.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        rng.gen_range((0.)..(window_width)),
                        rng.gen_range((0.)..(window_height)),
                        1. + rand::random::<f32>(),
                    )),
                    ..default()
                })
                .insert(Velocity {
                    x: rng.gen(),
                    y: 0.,
                });
        }
    }
}

fn menu_setup(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Welcome to Stroop!\nPress Space or Enter to Start\nPress Esc to Exit",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the justification of the Text
        .with_background_color(Color::BLACK)
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Auto),
            ..default()
        }),
        MenuText,
    ));
}

fn menu(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
    text_boxes: Query<(Entity, &Text)>,
) {
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::Return) {
        for (text_box, _) in text_boxes.iter() {
            commands.entity(text_box).despawn();
        }
        next_state.set(AppState::GameStart)
    }

    /* if keyboard_input.pressed(KeyCode::I) {
        next_state.set(AppState::Instructions);
    } */

    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

/* fn instructions(mut next_state: ResMut<NextState<AppState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::Return) {
        next_state.set(AppState::GameStart)
    }
    if keyboard_input.pressed(KeyCode::Escape) {
        next_state.set(AppState::Menu)
    }
} */

fn move_circles(
    mut transform_query: Query<
        (&mut Transform, &Velocity),
        (With<Velocity>, Without<Interactable>),
    >,
    time: Res<Time>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let window_width = window.width() + (CIRCLE_RADIUS * 2.);

    let delta_time = time.delta_seconds() * 400.;
    transform_query
        .iter_mut()
        .for_each(|(mut transform, velocity)| {
            transform.translation.x += velocity.x * delta_time;
            transform.translation.x = transform.translation.x % window_width
        });
}
