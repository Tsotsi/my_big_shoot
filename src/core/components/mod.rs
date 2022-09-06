use bevy::prelude::{Component, Deref, Vec2, Vec3};
use bevy_inspector_egui::Inspectable;
use std::{collections::HashMap, fmt::Debug};

pub mod character;

#[derive(Component)]
pub struct Name {
    pub value: String,
}

#[derive(Component)]
pub struct ActorData {
    sprite_atlas_path: String,
    sprite_idx: usize,
    current_state: String,
    animations: HashMap<String, Animation>,
}

pub struct Animation {
    duration_per_frame: f32,
    repeating: bool,
    sprite_idx: usize,
}

#[derive(Component, Debug, PartialEq, Eq, Inspectable)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

impl Direction {
    /// Returns `true` if the direction is [`Left`].
    ///
    /// [`Left`]: Direction::Left
    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left)
    }

    /// Returns `true` if the direction is [`Up`].
    ///
    /// [`Up`]: Direction::Up
    pub fn is_up(&self) -> bool {
        matches!(self, Self::Up)
    }

    /// Returns `true` if the direction is [`Down`].
    ///
    /// [`Down`]: Direction::Down
    pub fn is_down(&self) -> bool {
        matches!(self, Self::Down)
    }

    /// Returns `true` if the direction is [`Right`].
    ///
    /// [`Right`]: Direction::Right
    pub fn is_right(&self) -> bool {
        matches!(self, Self::Right)
    }

    pub fn prefix_str<T>(&self, prefix: T) -> String
    where
        T: Into<&'static str> + Debug,
    {
        match self {
            Direction::Up => {
                format!("{}_up", prefix.into())
            }
            Direction::Down => {
                format!("{}_down", prefix.into())
            }
            Direction::Left => {
                format!("{}_left", prefix.into())
            }
            Direction::Right => {
                format!("{}_right", prefix.into())
            }
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::Down
    }
}

#[derive(Component, Default)]
pub struct Lv {}

#[derive(Component, Default)]
pub struct Label(pub String);

#[derive(Component, Default, Deref, Inspectable, Debug)]
pub struct Velocity(pub Vec2);
impl Velocity {
    pub fn sub(&mut self, other: Vec2) -> &mut Self {
        // self.0.x -= other.0.x;
        self.0 -= other;
        self
    }
    pub fn add(&mut self, other: Vec2) -> &mut Self {
        // self.0.x -= other.0.x;
        self.0 += other;
        self
    }
    pub fn set(&mut self, other: Vec2) -> &mut Self {
        // self.0.x -= other.0.x;
        self.0 = other;
        self
    }
}

impl From<&Velocity> for Vec3 {
    fn from(velocity: &Velocity) -> Self {
        Self {
            x: velocity.0.x,
            y: velocity.0.y,
            z: 0f32,
        }
    }
}

#[derive(Component, Default, Inspectable, Debug)]
pub struct Info {
    pub speed: f32,
}

impl From<f32> for Info {
    fn from(speed: f32) -> Self {
        Info { speed: speed }
    }
}
