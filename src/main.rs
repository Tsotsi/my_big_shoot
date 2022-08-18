use bevy::prelude::*;


#[derive(Component)]
struct Position{
    x:f32,y:f32
}

#[derive(Component)]
struct Character;

#[derive(Component)]
struct Name(String);


fn hello_world(){
    println!("hello world from system!!");
}

// start setup
fn add_main_character(mut commands : Commands){
    commands.spawn().insert(Character)
    .insert(Name("Player1".to_string()));
    commands.spawn().insert(Character)
    .insert(Name("Player2".to_string()));
    commands.spawn().insert(Character)
    .insert(Name("Player3".to_string()));
}

fn greet_characters(query:Query<&Name, With<Character>>){
    for name in query.iter(){
        println!("hello {}!", name.0);
    }
}

fn main() {
    App::new()
    .add_startup_system(add_main_character)
    .add_system(hello_world)
    .add_system(greet_characters)
    .run();    
}

