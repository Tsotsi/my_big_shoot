use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;

#[derive(Component)]
pub struct Character;

#[derive(Component, Default, Inspectable, Debug)]
pub enum MovementState {
    #[default]
    Idle,
    Walk,
    Invalid,
}

impl MovementState {
    /// Returns `true` if the movement state is [`Idle`].
    ///
    /// [`Idle`]: MovementState::Idle
    #[must_use]
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Returns `true` if the movement state is [`Walk`].
    ///
    /// [`Walk`]: MovementState::Walk
    #[must_use]
    pub fn is_walk(&self) -> bool {
        matches!(self, Self::Walk)
    }

    // pub fn turn_to(&mut self, new_state: Self) -> &mut Self {
    //     *self = new_state;
    //     self
    // }
    pub fn turn_to<T>(&mut self, new_state: T) -> &mut Self
    where
        T: Into<Self>,
    {
        *self = new_state.into();
        self
    }
}

impl From<&str> for MovementState {
    fn from(s: &str) -> Self {
        match s {
            "idle" => MovementState::Idle,
            "walk" => MovementState::Walk,
            _ => MovementState::Invalid,
        }
    }
}

impl From<&MovementState> for &str {
    fn from(s: &MovementState) -> Self {
        match s {
            MovementState::Idle => "idle",
            MovementState::Walk => "walk",
            _ => "",
        }
    }
}

impl From<MovementState> for &str {
    fn from(s: MovementState) -> Self {
        match s {
            MovementState::Idle => "idle",
            MovementState::Walk => "walk",
            _ => "",
        }
    }
}
