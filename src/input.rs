use bevy::prelude::*;

pub struct InputPlugin;

#[derive(Event)]
pub struct LeftClickEvent {
    pub position: Vec2,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cursor_system)
        .run();
    }
}

fn cursor_system(
    btn: Res<Input<MouseButton>>,
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
}