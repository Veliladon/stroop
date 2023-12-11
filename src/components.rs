use bevy::prelude::*;

#[derive(Component, Clone, PartialEq, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/* #[derive(Component, Clone, PartialEq, Debug)]
pub struct CircleColor(Handle<ColorMaterial>); */

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct MenuText;

#[derive(Component)]
pub struct Offset(pub f32);

#[derive(Component)]
pub struct Correct;

#[derive(Component)]
pub struct Incorrect;

#[derive(Component)]
pub struct Interactable;

#[derive(Component)]
pub struct Instructions;
