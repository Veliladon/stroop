use bevy::prelude::*;

#[derive(Component, Clone, PartialEq, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
