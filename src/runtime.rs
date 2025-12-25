use crate::hiero::Glyph;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct PIDController {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub setpoint: Glyph, // Changed from f32 to Glyph (The 🧿 target)
    pub integral: f32,
    pub prev_error: f32,
}

#[derive(Component)]
pub struct MetabolicState {
    pub current_usage: f32,
    pub status: Glyph, // This is our "Runtime Only" glyph
}
