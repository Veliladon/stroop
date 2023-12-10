mod components;
mod input;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::prelude::*;

pub use crate::input::*;

use crate::components::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const NUMBER_ENTITIES: usize = 30_000;
pub const COLOR_SELECTION: [Color; 5] = [
    Color::RED,
    Color::YELLOW,
    Color::GREEN,
    Color::BLUE,
    Color::PURPLE,
];
pub const CIRCLE_RADIUS: f32 = 20.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
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
        .add_systems(Startup, setup)
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

    for _ in 0..NUMBER_ENTITIES {
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
    }
}

fn move_circles(
    mut velocity_query: Query<(&mut Transform, &Velocity), With<Velocity>>,
    time: Res<Time>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let window_width = window.width() + (CIRCLE_RADIUS * 2.);

    let delta_time = time.delta_seconds() * 400.;
    velocity_query
        .iter_mut()
        .for_each(|(mut transform, velocity)| {
            transform.translation.x += velocity.x * delta_time;
            transform.translation.x = transform.translation.x % window_width
        });
}
