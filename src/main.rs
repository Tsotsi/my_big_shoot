use bevy::{prelude::*, transform};

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

// start setup
fn add_main_character(mut commands: Commands) {
    commands
        .spawn()
        .insert(Character)
        .insert(Name("Player1".to_string()));
    commands
        .spawn()
        .insert(Character)
        .insert(Name("Player2".to_string()));
    commands
        .spawn()
        .insert(Character)
        .insert(Name("Player3".to_string()));
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
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. && *logo != Direction::Left {
            *logo = Direction::Left;
        } else if transform.translation.y < -200. && *logo != Direction::Right {
            *logo = Direction::Right;
        } else if transform.translation.x > 200. && *logo != Direction::Up {
            *logo = Direction::Up;
        } else if transform.translation.x < -200. && *logo != Direction::Down {
            *logo = Direction::Down;
        }
    }
}

// systems end
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("brands/icon.png"),
            transform: Transform::from_xyz(64., 0., 0.),
            ..default()
        })
        .insert(Direction::Up);
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_main_character)
            .add_startup_system(setup)
            //.add_system(hello_world)
            .add_system(greet_characters)
            .add_system(sprite_movement);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
