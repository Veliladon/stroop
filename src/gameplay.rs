use crate::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource)]
pub struct GameState {
    score: usize,
    remaining_time: Timer,
    timer_expired: bool,
}

#[derive(Clone)]
pub enum CorrectIncorrect {
    Correct,
    Incorrect,
}

#[derive(Clone)]
pub enum WordOrColor {
    Word,
    Color,
}

#[derive(Component)]
struct ColoredWord;

#[derive(Component)]
struct ColoredOrWord;

#[derive(Component)]
struct RemainingTime;

#[derive(Component)]
struct Score;

#[derive(Component)]
struct GameOverText;

pub const CORRECT_OR_INCORRECT: [CorrectIncorrect; 2] =
    [CorrectIncorrect::Correct, CorrectIncorrect::Incorrect];

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

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 100.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the justification of the Text
        .with_background_color(Color::BLACK)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Auto),
            ..default()
        }),
        ColoredWord,
    ));

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the justification of the Text
        .with_background_color(Color::BLACK)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Percent(75.0),
            ..default()
        }),
        ColoredOrWord,
    ));

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 100.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the justification of the Text
        .with_background_color(Color::BLACK)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Percent(5.0),
            ..default()
        }),
        RemainingTime,
    ));

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "0",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 100.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the justification of the Text
        .with_background_color(Color::BLACK)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Percent(20.0),
            ..default()
        }),
        Score,
    ));
}

fn score_and_spawn_new_circles(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    colors: Res<ColorResource>,
    mesh: Res<MeshResource>,
    mut circle_query: Query<(Entity, &mut Handle<ColorMaterial>), Without<Interactable>>,
    window_query: Query<&Window>,
    mut colored_word_query: Query<
        &mut Text,
        (
            With<ColoredWord>,
            Without<ColoredOrWord>,
            Without<RemainingTime>,
        ),
    >,
    mut colored_or_word_query: Query<
        &mut Text,
        (
            With<ColoredOrWord>,
            Without<ColoredWord>,
            Without<RemainingTime>,
        ),
    >,
) {
    let mut rng = thread_rng();

    let window = window_query.single();
    let window_height = window.height();

    let correct_or_incorrect: CorrectIncorrect =
        CORRECT_OR_INCORRECT.choose(&mut rng).unwrap().clone();

    let mut color_candidates = colors.0.to_vec().clone();

    let correct_num = rng.gen_range(0..5);
    let (correct_color, correct_color_name, correct_color_const) =
        color_candidates.remove(correct_num);
    let wrong_num = rng.gen_range(0..4);
    let (wrong_color, wrong_color_name, wrong_color_const) =
        color_candidates.remove(wrong_num).clone();

    let word;
    let word_color;
    let word_or_color;

    match correct_or_incorrect {
        CorrectIncorrect::Correct => {
            word = correct_color_name.clone();
            word_color = wrong_color_const.clone();
            word_or_color = "As Written".to_string();
        }
        CorrectIncorrect::Incorrect => {
            word = wrong_color_name.clone();
            word_color = correct_color_const.clone();
            word_or_color = "As Colored".to_string();
        }
    }

    let mut colored_word = colored_word_query.single_mut();
    colored_word.sections[0].value = word;
    colored_word.sections[0].style.color = word_color;

    let mut colored_or_word = colored_or_word_query.single_mut();
    colored_or_word.sections[0].value = word_or_color;

    let correct_top: bool = rng.gen();
    let correct_y;
    let incorrect_y;
    if correct_top {
        correct_y = rng.gen_range((window_height / 2. + 20.)..(window_height - 160.));
        incorrect_y = rng.gen_range((160.)..(window_height / 2. - 20.));
    } else {
        incorrect_y = rng.gen_range((window_height / 2. + 20.)..(window_height - 160.));
        correct_y = rng.gen_range((160.)..(window_height / 2. - 20.));
    }

    circle_query.iter_mut().for_each(|(_, mut color)| {
        (*color, _, _) = color_candidates.choose(&mut rng).unwrap().clone();
    });

    let offset: f32 = rng.gen();

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: mesh.0.clone(),
            material: correct_color.clone(),
            transform: Transform::from_translation(Vec3::new(-20., correct_y, 3.)),
            ..default()
        })
        .insert(Interactable)
        .insert(Correct)
        .insert(Offset { 0: offset });

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: mesh.0.clone(),
            material: wrong_color.clone(),
            transform: Transform::from_translation(Vec3::new(-20., incorrect_y, 3.)),
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
    mut timer_query: Query<
        &mut Text,
        (
            With<RemainingTime>,
            Without<ColoredWord>,
            Without<ColoredOrWord>,
            Without<Score>,
        ),
    >,

    mut score_query: Query<
        &mut Text,
        (
            Without<RemainingTime>,
            Without<ColoredWord>,
            Without<ColoredOrWord>,
            With<Score>,
        ),
    >,

    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut click_event: EventReader<LeftClickEvent>,
    missed_event: EventReader<MissedCircleEvent>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (correct_entity, correct) = correct_query.single();
    let (incorrect_entity, incorrect) = incorrect_query.single();
    let mut timer_text = timer_query.single_mut();
    let mut score_text = score_query.single_mut();

    game_state.remaining_time.tick(time.delta());
    if game_state.remaining_time.just_finished() {
        game_state.timer_expired = true;
        next_state.set(AppState::GameOver);
    }
    timer_text.sections[0].value =
        format!("{}", game_state.remaining_time.remaining_secs().trunc());

    if !missed_event.is_empty() {
        next_state.set(AppState::GameOver);
    }

    for event in click_event.read() {
        if event.position.distance(correct.translation.xy()) < 21. {
            game_state.score += 1;
            commands.entity(correct_entity).despawn();
            commands.entity(incorrect_entity).despawn();
            score_text.sections[0].value = format!("{}", game_state.score);
            next_state.set(AppState::NextRound);
        }
        if event.position.distance(incorrect.translation.xy()) < 21. {
            next_state.set(AppState::GameOver);
        }
    }
}

fn game_over(
    mut commands: Commands,
    game_state: Res<GameState>,
    text_boxes: Query<(Entity, &Text)>,
    correct_query: Query<(Entity, &Transform), With<Correct>>,
    incorrect_query: Query<(Entity, &Transform), With<Incorrect>>,
) {
    for (text_box, _) in text_boxes.iter() {
        commands.entity(text_box).despawn();
    }

    for (circle, _) in correct_query.iter() {
        commands.entity(circle).despawn();
    }

    for (circle, _) in incorrect_query.iter() {
        commands.entity(circle).despawn();
    }

    let game_over_text;

    if game_state.timer_expired {
        game_over_text = format!(
            "Congratulations! You won!\nScore: {}\n\nPress Space to start or Esc to quit",
            game_state.score
        );
    } else {
        game_over_text = format!(
            "Sorry. You lost!\nScore: {}\n\nPress Space to start or Esc to quit",
            game_state.score
        );
    }

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            game_over_text,
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 100.0,
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
        GameOverText,
    ));
}

fn game_over_input(
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

    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
