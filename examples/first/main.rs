// use crate::plugins::preload_res;
use bevy::asset::AssetStage;
use bevy::prelude::*;
use bevy::window;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use big_shoot::core::components::character::Label;
use big_shoot::core::loaders::animation_loader::Animation;
use big_shoot::core::loaders::MainConfig;
use big_shoot::core::loaders::animation_loader::AnimationState;
use std::collections::HashMap;
use std::fs;

use big_shoot::core::components::character;
use big_shoot::core::loaders::{animation_loader::AnimationLoader, MainConfigLoader};
use big_shoot::core::plugins::preload_res;
use big_shoot::core::GameState;

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn spawn_main_character(
    mut commands: Commands,
    main_cfg:Res<MainConfig>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    texture_atlas_map: Res<HashMap<String, Handle<TextureAtlas>>>,
) {
    let animation_handle: Handle<Animation> =
        asset_server.load("characters/animations/chara_1_1_walk_down.animation.toml");
        match animation_handle.id{
            bevy::asset::HandleId::Id(uuid, id) => println!("uuid: {}, id: {}", uuid,id),
            bevy::asset::HandleId::AssetPathId(p) => println!("assetpath: {:?}", p),
        }
    let mut main_sheet_bundle = SpriteSheetBundle {
        texture_atlas: texture_atlas_map.get(&main_cfg.characters["main"].img).expect(format!("can't found main texture-atlas: {}", &main_cfg.characters["main"].img).as_str()).clone(),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..Default::default()
    };
    // main_sheet_bundle.sprite.index = main_cfg.characters["main"].sprite_idx;
    commands.spawn_bundle(main_sheet_bundle)
    .insert(Label(main_cfg.characters["main"].name.clone()))
    .insert(animation_handle)
    .insert(AnimationState::default());
    
}


fn animate(
    time: Res<Time>,
    animations: Res<Assets<Animation>>,
    mut query: Query<(
        &mut AnimationState,
        &mut TextureAtlasSprite,
        &Handle<Animation>,
    )>,
) {
    for (mut player, mut texture, handle) in query.iter_mut() {
        // Get the animation from handle (or skip this entity if not yet loaded)
        let animation = match animations.get(handle) {
            Some(anim) => anim,
            None => continue,
        };

        // Update the state
        player.update(animation, time.delta());

        // Update the texture atlas
        texture.index = player.frame_index();
    }
}

fn save_animation(ipt: Res<Input<KeyCode>>,
    animations: Res<Assets<Animation>>, query: Query<(&Handle<Animation>, &Label)>){
    
    if ipt.just_released(KeyCode::LWin) {
        for (handle, label) in query.iter()  {
            match animations.get(handle) {
                Some(anim) => {
                    fs::write(format!("assets/cache/{}.animation.toml", label.0), toml::to_string(&anim.0).expect("serialize animation failed"))
                },
                None => continue,
            };
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("game is going setup");
    let _: Handle<MainConfig> = asset_server.load("configs/main.cfg.toml");
    // commands.spawn_bundle(Camera2dBundle::default());
}

pub fn run() {
    let r = &fs::read("assets/configs/main.cfg.toml").expect("msg");
    let main_cfg: MainConfig = toml::from_slice(r).expect("msg");
    println!("cfg: {:?}", main_cfg);
    App::new()
        .insert_resource(WindowDescriptor {
            title: main_cfg.title.clone(),
            width: main_cfg.size.x,
            height: main_cfg.size.y,
            ..Default::default()
        })
        .insert_resource(main_cfg)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(preload_res::PreloadResPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_asset::<Animation>()
        .add_asset::<MainConfig>()
        .init_asset_loader::<AnimationLoader>()
        .init_asset_loader::<MainConfigLoader>()
        .insert_resource(GameState::Normal)
        .add_startup_system(setup)
        .add_startup_system(spawn_main_character)
        // .add_startup_system_to_stage(StartupStage::PostStartup, spawn_main_character)
        .add_system(ui_example)
        .add_system(animate)
        .add_system(save_animation)
        .run();
}
fn main() {
    run();
}
