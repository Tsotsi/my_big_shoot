// use crate::plugins::preload_res;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use std::collections::HashMap;

use super::super::plugins::preload_res;
use super::super::components::character;

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn spawn_main_character(mut commands: Commands, texture_atlas_map: Res<HashMap<String, Handle<TextureAtlas>>>){
    commands
        .spawn()
        .insert(character::Character)
        .insert(character::Name{
            value:"Player1".to_string(),
        });
    let mut character1_sheet = SpriteSheetBundle {
        texture_atlas: texture_atlas_map
            .get(&preload_res::CHARACTER_ASSET_PNG_1.to_string())
            .expect("can't find")
            .to_owned(),
        transform: Transform::from_xyz(64., 0., 0.),
        ..default()
    };
    character1_sheet.sprite.index =15;
    commands
        .spawn_bundle(character1_sheet)
        .insert(character::Direction::Up);
}

pub fn run(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(preload_res::PreloadResPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_main_character)
        .add_system(ui_example)
        .run();
}