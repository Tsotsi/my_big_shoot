// use crate::plugins::preload_res;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::LogSettings;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use std::collections::HashMap;
use std::fs;

use big_shoot::core::components::character::MovementState;
use big_shoot::core::components::{Info, Label, Velocity};
use big_shoot::core::loaders::animation_loader::Animation;
use big_shoot::core::loaders::animation_loader::AnimationState;
use big_shoot::core::loaders::MainConfig;
use big_shoot::core::loaders::{animation_loader::AnimationLoader, MainConfigLoader};
use big_shoot::core::loaders::{AnimationMap, AnimationNameMap};
use big_shoot::core::plugins::animation_control::AnimationControlPlugin;
use big_shoot::core::plugins::player::PlayerBundle;
use big_shoot::core::plugins::preload_res;
use big_shoot::core::states::GameState;

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn spawn_main_character(
    mut commands: Commands,
    main_cfg: Res<MainConfig>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    texture_atlas_map: Res<HashMap<String, Handle<TextureAtlas>>>,
) {
    let animation_name_map = AnimationNameMap::from(main_cfg.characters["main"].clone());
    let mut animation_map = AnimationMap::new();
    for (name, cfg_path) in animation_name_map.iter() {
        animation_map.insert(
            name.clone(),
            asset_server.load::<Animation, &std::string::String>(&cfg_path),
        );
    }

    let mut main_sheet_bundle = SpriteSheetBundle {
        texture_atlas: texture_atlas_map
            .get(&main_cfg.characters["main"].img)
            .expect(
                format!(
                    "can't found main texture-atlas: {}",
                    &main_cfg.characters["main"].img
                )
                .as_str(),
            )
            .clone(),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..Default::default()
    };
    // main_sheet_bundle.sprite.index = main_cfg.characters["main"].sprite_idx;
    commands
        .spawn_bundle(main_sheet_bundle)
        .insert(animation_map)
        .insert(AnimationState::default())
        // .insert(big_shoot::core::components::Direction::default())
        // .insert(Label(main_cfg.characters["main"].name.clone()))
        .insert(LookAt)
        .insert_bundle(PlayerBundle {
            label: Label(main_cfg.characters["main"].name.clone()),
            info: Info::from(240.0f32),
            ..Default::default()
        });
}

fn save_animation(
    ipt: Res<Input<KeyCode>>,
    animations: Res<Assets<Animation>>,
    query: Query<(&Handle<Animation>, &Label)>,
) {
    if ipt.just_released(KeyCode::LWin) {
        for (handle, label) in query.iter() {
            if let Some(anim) = animations.get(handle) {
                let s = format!("assets/cache/{}.animation.toml", label.0);
                info!(target:"app_dev",save=%s);
                let _ = fs::write(
                    s,
                    toml::to_string(&anim.0).expect("serialize animation failed"),
                );
            }
        }
    }
}

#[derive(Component, Default)]
struct MainCamera;
#[derive(Component)]
struct LookAt;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!(target:"app_dev","game is going setup");
    // let _: Handle<MainConfig> = asset_server.load("configs/main.cfg.toml");
    // commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera::default());
}

fn smooth_camera_system( mut main_camera_query: Query<(&mut Transform, &Camera2d), With<MainCamera>>, look_at_query: Query<&Transform, (With<LookAt>, Without<MainCamera>)>){
    let (mut main_camera_tranform, main_camera) = main_camera_query.single_mut();
    for (look_at) in look_at_query.iter(){
        main_camera_tranform.translation = look_at.translation;
        break
    }

}

fn handle_player_input(
    game_state: Res<GameState>,
    time: Res<Time>,
    mut commands: Commands,
    ipt: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut big_shoot::core::components::Direction,
        Option<&mut Velocity>,
        Option<&Info>,
        Option<&mut MovementState>,
    )>,
) {
    if game_state.is_paused() {
        return;
    }
    for (mut d, v, i, m) in query.iter_mut() {
        let mut dir = Vec2::ZERO;

        if ipt.pressed(KeyCode::A) {
            dir.x += -1.0f32;
        }
        if ipt.pressed(KeyCode::D) {
            dir.x += 1.0f32;
        }
        if ipt.pressed(KeyCode::S) {
            dir.y += -1.0f32;
        }
        if ipt.pressed(KeyCode::W) {
            dir.y += 1.0f32;
        }
        if dir != Vec2::ZERO {
            if dir.y < 0f32 {
                *d = big_shoot::core::components::Direction::Down;
            } else if dir.y > 0f32 {
                *d = big_shoot::core::components::Direction::Up;
            }
            if dir.x < 0f32 {
                *d = big_shoot::core::components::Direction::Left;
            } else if dir.x > 0f32 {
                *d = big_shoot::core::components::Direction::Right;
            }
        }

        if let Some(mut movement_state) = m {
            if dir != Vec2::ZERO {
                movement_state.turn_to("walk");
            } else {
                movement_state.turn_to("idle");
            }
            // println!("handle_player_input!!! got movement_state. dir: {:?}, movement_state: {:?}", dir, movement_state);
        }

        // try move
        if let (Some(mut velocity), Some(info)) = (v, i) {
            velocity.set(info.speed * dir.normalize_or_zero() * time.delta_seconds());
        }
    }
}

fn player_move_system(
    game_state: Res<GameState>,
    mut query: Query<(&mut Transform, &MovementState, &Velocity)>,
) {
    if game_state.is_paused() {
        return;
    }
    for (mut transform, movement_state, velocity) in query.iter_mut() {
        // transform.translation += Vec3::new(0f32, 0f32, 0f32);
        if !movement_state.is_walk() {
            continue;
        }
        transform.translation += Vec3::from(velocity);
    }
}
#[cfg(debug_assertions)]
fn log_setting(app: &mut App) {
    app.insert_resource(LogSettings {
        filter: "info,app_dev=debug,wgpu_core=warn,wgpu_hal=warn,minewars=debug".into(),
        level: bevy::log::Level::DEBUG,
    });
}
#[cfg(not(debug_assertions))]
fn log_setting(app: &mut App) {
    app.insert_resource(LogSettings {
        filter: "warn".into(),
        level: bevy::log::Level::WARN,
    });
}

pub fn run() {
    let r = &fs::read("assets/configs/main.cfg.toml").expect("msg");
    let main_cfg: MainConfig = toml::from_slice(r).expect("msg");
    println!("app_dev: {:?}", main_cfg);
    let mut app = App::new();
    log_setting(&mut app);
    app.insert_resource(WindowDescriptor {
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
    .add_plugin(AnimationControlPlugin::new())
    // log diagnostic
    // .add_plugin(LogDiagnosticsPlugin::default())
    // .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_asset::<Animation>()
    .add_asset::<MainConfig>()
    .init_asset_loader::<AnimationLoader>()
    .init_asset_loader::<MainConfigLoader>()
    .insert_resource(GameState::default())
    .add_startup_system(setup)
    .add_startup_system(
        spawn_main_character.after(preload_res::PreloadResPluginLabel::LoadSpriteTexture),
    )
    // .add_startup_system_to_stage(StartupStage::PostStartup, spawn_main_character)
    .add_system(smooth_camera_system)
    .add_system(ui_example)
    .add_system(save_animation)
    .add_system(handle_player_input)
    .add_system(player_move_system)
    .register_inspectable::<MovementState>()
    .register_inspectable::<Info>()
    .register_inspectable::<Velocity>()
    .register_inspectable::<big_shoot::core::components::Direction>()
    .run();
}
fn main() {
    run();
}
