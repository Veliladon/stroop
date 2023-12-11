use crate::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource)]
pub struct GameState {
    score: usize,
    remaining_time: Timer,
    timer_expired: bool,
}

#[derive(Event)]
pub struct MissedCircleEvent;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MissedCircleEvent>()
            .add_systems(OnEnter(AppState::GameStart), setup_game)
            .add_systems(OnEnter(AppState::NextRound), score_and_spawn_new_circles)
            .add_systems(
                Update,
                move_target_circles.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                process_events_and_timers.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::GameOver), game_over)
            .add_systems(Update, game_over_input.run_if(in_state(AppState::GameOver)));
    }
}

fn setup_game(mut commands: Commands<'_, '_>, mut next_state: ResMut<NextState<AppState>>) {
    let game_state = GameState {
        score: 0,
        remaining_time: Timer::from_seconds(60., TimerMode::Once),
        timer_expired: false,
    };
    commands.insert_resource(game_state);
    next_state.set(AppState::NextRound);
}

fn score_and_spawn_new_circles(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    colors: Res<ColorResource>,
    mesh: Res<MeshResource>,
    mut circle_query: Query<(Entity, &mut Handle<ColorMaterial>), Without<Interactable>>,
    window_query: Query<&Window>,
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

    println!("Correct: {:?}", correct_color);

    next_state.set(AppState::InGame);
}

fn move_target_circles(
    mut transform_query: Query<(&mut Transform, &Offset), With<Interactable>>,
    time: Res<Time>,
    mut missed_circle_event: EventWriter<MissedCircleEvent>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let window_width = window.width() + 40.;
    let delta_time = time.delta_seconds() * 200.;
    transform_query.iter_mut().for_each(|(mut transform, _)| {
        transform.translation.x += delta_time;
        if transform.translation.x >= window_width {
            missed_circle_event.send(MissedCircleEvent);
        }
    });
}

fn process_events_and_timers(
    correct_query: Query<(Entity, &Transform), With<Correct>>,
    incorrect_query: Query<(Entity, &Transform), With<Incorrect>>,

    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut click_event: EventReader<LeftClickEvent>,
    missed_event: EventReader<MissedCircleEvent>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (correct_entity, correct) = correct_query.single();
    let (incorrect_entity, incorrect) = incorrect_query.single();

    game_state.remaining_time.tick(time.delta());
    if game_state.remaining_time.just_finished() {
        game_state.timer_expired = true;
        next_state.set(AppState::GameOver);
    }

    if !missed_event.is_empty() {
        println!("You Missed!");
        next_state.set(AppState::GameOver);
    }

    for event in click_event.read() {
        if event.position.distance(correct.translation.xy()) < 21. {
            game_state.score += 1;
            println!("Score!");
            commands.entity(correct_entity).despawn();
            commands.entity(incorrect_entity).despawn();
            next_state.set(AppState::NextRound);
        }
        if event.position.distance(incorrect.translation.xy()) < 21. {
            println!("Wrong!");
            commands.entity(correct_entity).despawn();
            commands.entity(incorrect_entity).despawn();
            next_state.set(AppState::GameOver);
        }
    }
}

fn game_over(game_state: Res<GameState>) {
    if game_state.timer_expired {
        println!("Congratulations! You won!");
    } else {
        println!("Sorry. You lost!");
    }
}

fn game_over_input(
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::Return) {
        println!("Starting Game");
        next_state.set(AppState::GameStart)
    }

    if keyboard_input.pressed(KeyCode::I) {
        next_state.set(AppState::Instructions);
    }

    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
