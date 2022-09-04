use std::collections::HashMap;
use bevy::prelude::Component;

pub mod character;


#[derive(Component)]
pub struct Name{
    pub value: String,
}

#[derive(Component)]
pub struct ActorData{
    sprite_atlas_path: String,
    sprite_idx: usize,
    current_state:String,
    animations: HashMap<String, Animation>,
}

pub struct Animation{
    duration_per_frame: f32,
    repeating:bool,
    sprite_idx:usize,
}
#[derive(Component, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Lv {

}