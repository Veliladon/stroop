use crate::*;
use bevy::prelude::*;

pub struct InputPlugin;

#[derive(Event)]
pub struct LeftClickEvent {
    pub position: Vec2,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, cursor_system.run_if(in_state(AppState::InGame)))
            .add_event::<LeftClickEvent>();
    }
}

fn cursor_system(
    mut next_state: ResMut<NextState<AppState>>,
    btn: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    mut left_click: EventWriter<LeftClickEvent>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.get_single().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            left_click.send(LeftClickEvent {
                position: (world_position.x, world_position.y).into(),
            });

            info!(
                "Clicked! World coords: {}/{}",
                world_position.x, world_position.y
            );
        }
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Menu);
    }
}
