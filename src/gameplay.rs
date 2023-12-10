use crate::*;
use bevy::prelude::*;
use rand::prelude::*;

pub struct GameState;

#[derive(Event)]
pub struct MissedCircleEvent;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Scoring), score_and_spawn_new_circles)
            .add_systems(
                Update,
                move_target_circles.run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, process_events.run_if(in_state(AppState::InGame)))
            .add_systems(Update, game_over.run_if(in_state(AppState::GameOver)));
    }
}

fn score_and_spawn_new_circles(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    colors: Res<ColorResource>,
    mesh: Res<MeshResource>,
    mut circle_query: Query<(Entity, &mut Handle<ColorMaterial>), Without<Interactable>>,
    window_query: Query<&Window>,
    /* mut one_query: Query<(&mut Handle<ColorMaterial>), With<GroupOne>>,
    mut two_query: Query<(&mut Handle<ColorMaterial>), With<GroupTwo>>,
    mut three_query: Query<(&mut Handle<ColorMaterial>), With<GroupThree>>,
    mut four_query: Query<(&mut Handle<ColorMaterial>), With<GroupFour>>,
    mut five_query: Query<(&mut Handle<ColorMaterial>), With<GroupFive>>, */
) {
    let mut rng = thread_rng();

    let window = window_query.single();
    let window_height = window.height();

    let mut color_candidates = colors.to_vec();
    let correct_num = rng.gen_range(0..4);
    let correct_color = color_candidates.remove(correct_num);
    let wrong_num = rng.gen_range(0..3);
    let wrong_color = color_candidates.remove(wrong_num);

    println!("{:?}", color_candidates);

    circle_query.iter_mut().for_each(|(_, mut color)| {
        *color = color_candidates.choose(&mut rng).unwrap().clone();
    });

    let offset: f32 = rng.gen();

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: mesh.0.clone(),
            material: correct_color.clone(),
            transform: Transform::from_translation(Vec3::new(
                -20.,
                rng.gen_range((0.)..(window_height)),
                3.,
            )),
            ..default()
        })
        .insert(Interactable)
        .insert(Correct)
        .insert(Offset { 0: offset });

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: mesh.0.clone(),
            material: wrong_color.clone(),
            transform: Transform::from_translation(Vec3::new(
                -20.,
                rng.gen_range((0.)..(window_height)),
                3.,
            )),
            ..default()
        })
        .insert(Interactable)
        .insert(Incorrect)
        .insert(Offset { 0: offset });

    next_state.set(AppState::InGame);
}

fn move_target_circles(
    mut transform_query: Query<(&mut Transform, &Offset), (With<Interactable>)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds() * 400.;
    transform_query.iter_mut().for_each(|(mut transform, _)| {
        transform.translation.x += delta_time;
    });
}

fn process_events() {}

fn game_over() {}
