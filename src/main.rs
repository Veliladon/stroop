mod components;
mod gameplay;
mod input;

use bevy::app::AppExit;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::render_resource::encase::rts_array::Length;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::prelude::*;

pub use crate::components::*;
pub use crate::gameplay::*;
pub use crate::input::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
pub struct ColorResource([Handle<ColorMaterial>; 5]);

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct MeshResource(Mesh2dHandle);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    Instructions,
    InGame,
    Scoring,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<AppState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                }),
        )
        .add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))
        .add_plugins(InputPlugin)
        .add_plugins(GameplayPlugin)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Menu), menu_setup)
        .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
        .add_systems(
            Update,
            instructions.run_if(in_state(AppState::Instructions)),
        )
        .add_systems(Update, move_circles)
        .add_plugins(WorldInspectorPlugin::new())
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
        material_red,
        material_yellow,
        material_green,
        material_blue,
        material_purple,
    ];

    let color_resource = ColorResource {
        0: color_handles.clone(),
    };

    commands.insert_resource(color_resource);

    for color in 0..color_handles.length() {
        let circle_color = &color_handles[color].clone();
        for _ in 0..NUMBER_ENTITIES / color_handles.length() {
            let entity = commands
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
                })
                .id();
            match color {
                0 => commands.entity(entity).insert(GroupOne),
                1 => commands.entity(entity).insert(GroupTwo),
                2 => commands.entity(entity).insert(GroupThree),
                3 => commands.entity(entity).insert(GroupFour),
                4 => commands.entity(entity).insert(GroupFive),
                _ => unreachable!("What color are you?"),
            };
        }
    }

    /*for _ in 0..NUMBER_ENTITIES {
        commands
                .spawn(MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: color_handles.choose(&mut rng).unwrap().clone(),
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
    } */
}

fn menu_setup() {}

fn menu(
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::Return) {
        next_state.set(AppState::Scoring)
    }

    if keyboard_input.pressed(KeyCode::I) {
        next_state.set(AppState::Instructions);
    }

    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn instructions(mut next_state: ResMut<NextState<AppState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::Return) {
        next_state.set(AppState::InGame)
    }
    if keyboard_input.pressed(KeyCode::Escape) {
        next_state.set(AppState::Menu)
    }
}

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
