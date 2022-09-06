use bevy::prelude::*;

use crate::core::components::Direction;
use crate::core::components::character::MovementState;
use crate::core::loaders::{
    animation_loader::{Animation, AnimationState},
    AnimationMap,
};
use crate::core::states::GameState;

pub struct AnimationControlPlugin;
pub enum AnimationControlPluginLabel {
    WalkAnimateSystem,
}

impl SystemLabel for AnimationControlPluginLabel {
    fn as_str(&self) -> &'static str {
        match self {
            AnimationControlPluginLabel::WalkAnimateSystem => {
                "AnimationControlPluginLabel::WalkAnimateSystem"
            }
        }
    }
}
impl AnimationControlPlugin{
    pub fn new() -> Self {
        AnimationControlPlugin{}
    }
}
impl Plugin for AnimationControlPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(animate_system.label(AnimationControlPluginLabel::WalkAnimateSystem));
    }
}

#[derive(Debug, Component, Deref)]
pub struct AnimationClipName(String);

impl From<AnimationClipName> for String {
    fn from(ac: AnimationClipName) -> Self {
        ac.0
    }
}

pub fn animate_system(
    time: Res<Time>,
    animations: Res<Assets<Animation>>,
    game_state: Res<GameState>,
    mut query: Query<(
        &MovementState,
        &mut AnimationState,
        &mut TextureAtlasSprite,
        &Direction,
        &AnimationMap,
    )>,
) {
    if game_state.is_paused() {
        return;
    }
    for (clip_name, mut player, mut texture, dir, animation_map) in query.iter_mut() {
        // Get the animation from handle (or skip this entity if not yet loaded)
        match animation_map.get(&dir.prefix_str(clip_name)) {
            Some(v) => {
                let animation = match animations.get(v) {
                    Some(anim) => anim,
                    None => continue,
                };
                // Update the state
                player.update(animation, time.delta());

                // Update the texture atlas
                texture.index = player.frame_index();
            }
            None => continue,
        }
    }
}
