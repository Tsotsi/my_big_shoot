use crate::core::components::{character::MovementState, Direction, Info, Label, Lv, Velocity};
use bevy::prelude::Bundle;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    // lv
    pub lv: Lv,
    // label name
    pub label: Label,
    // direction
    pub direction: Direction,
    // player movement state (relate with animation)
    pub movement_state: MovementState,
    // velocity
    pub velocity: Velocity,
    // info
    pub info: Info,
}
