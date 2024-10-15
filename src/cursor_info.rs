use bevy::prelude::*;

#[derive(Resource)]
pub struct OffsetedCursorPosition {
    pub x: f32,
    pub y: f32,
}
