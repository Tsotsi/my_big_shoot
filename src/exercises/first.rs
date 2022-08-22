use bevy::{prelude::*, transform};
use std::collections::HashMap;
use bevy_egui::{egui, EguiContext, EguiPlugin};

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Character;

#[derive(Component)]
struct Name(String);

fn hello_world() {
    println!("hello world from system!!");
}

const CHARACTER_ASSET_PNG_1: &str = "characters/bonus1.png";
// start setup
fn add_main_character(
    mut commands: Commands,
    texture_atlas_map: Res<HashMap<String, Handle<TextureAtlas>>>,
) {
    commands
        .spawn()
        .insert(Character)
        .insert(Name("Player1".to_string()));
    let mut character1_sheet = SpriteSheetBundle {
        texture_atlas: texture_atlas_map
            .get(&CHARACTER_ASSET_PNG_1.to_string())
            .expect("can't find")
            .to_owned(),
        transform: Transform::from_xyz(64., 0., 0.),
        ..default()
    };
    character1_sheet.sprite.index =15;
    commands
        .spawn_bundle(character1_sheet)
        .insert(Direction::Up);

    let mut character2_sheet = SpriteSheetBundle {
        texture_atlas: texture_atlas_map
            .get(&CHARACTER_ASSET_PNG_1.to_string())
            .expect("can't find")
            .to_owned(),
        transform: Transform::from_xyz(64., 64., 0.),
        ..default()
    };
    character2_sheet.sprite.index =51;
    commands
        .spawn_bundle(character2_sheet)
        .insert(Direction::Up);
}

struct GreetTimer(Timer);
// systems
fn greet_characters(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Character>>,
) {
    // update our timer with the time elapsed since the last update
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

#[derive(Component, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        // match *logo {
        //     Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
        //     Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        //     Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
        //     Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
        // }

        // if transform.translation.y > 200. && *logo != Direction::Left {
        //     *logo = Direction::Left;
        // } else if transform.translation.y < -200. && *logo != Direction::Right {
        //     *logo = Direction::Right;
        // } else if transform.translation.x > 200. && *logo != Direction::Up {
        //     *logo = Direction::Up;
        // } else if transform.translation.x < -200. && *logo != Direction::Down {
        //     *logo = Direction::Down;
        // }
    }
}

#[derive(Component)]
struct SpriteData {
    raw_data: String,
    size: Vec2,
    animation_interval: f32,
    repeating: bool,
}
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn load_sprite(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut sprites: Query<(Entity, &SpriteData)>,
) {
    for (sp_ent, sd) in &sprites {
        commands
            .entity(sp_ent)
            .insert(AnimationTimer(Timer::from_seconds(
                sd.animation_interval,
                sd.repeating,
            )));
        commands.entity(sp_ent).remove::<SpriteData>();
    }
}

// systems end
fn setup(
    mut texture_atlas_map: ResMut<HashMap<String, Handle<TextureAtlas>>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(CHARACTER_ASSET_PNG_1);
    let texture_atlas = TextureAtlas::from_grid_with_padding(texture_handle,
         Vec2::new(26., 34.), 
         12, 8,
         Vec2::new(0., 2.),Vec2::new(0., 2.));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    texture_atlas_map.insert(CHARACTER_ASSET_PNG_1.to_string(), texture_atlas_handle);
    commands.spawn_bundle(Camera2dBundle::default());
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        let h: HashMap<String, Handle<TextureAtlas>> = HashMap::with_capacity(10);
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .insert_resource(h)
            .add_startup_system(setup)
            .add_startup_system(add_main_character.after(setup))
            //.add_system(hello_world)
            .add_system(greet_characters)
            .add_system(sprite_movement);
    }
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}

pub fn first_run(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(HelloPlugin)
        .add_system(ui_example)
        .run();
}