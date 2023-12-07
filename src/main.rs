mod components;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::prelude::*;

use crate::components::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const NUMBER_ENTITIES: usize = 10_000;
pub const COLOR_SELECTION: [Color; 5] = [
    Color::RED,
    Color::YELLOW,
    Color::GREEN,
    Color::BLUE,
    Color::PURPLE,
];
pub const CIRCLE_RADIUS: f32 = 10.;

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

    commands.spawn(Camera2dBundle::default());
    for _ in 0..NUMBER_ENTITIES {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(CIRCLE_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(
                    COLOR_SELECTION
                        .choose(&mut rng)
                        .unwrap_or(&Color::BLACK)
                        .clone(),
                )),
                transform: Transform::from_translation(Vec3::new(
                    -750.,
                    rng.gen_range((-360.)..(360.)),
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
) {
    let delta_time = time.delta_seconds() * 400.;
    velocity_query
        .par_iter_mut()
        .for_each(|(mut transform, velocity)| {
            transform.translation.x += velocity.x * delta_time;
            if transform.translation.x >= 750. {
                transform.translation.x = -transform.translation.x;
            }
        });
}
