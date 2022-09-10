use bevy::{
    prelude::*,
};
use std::collections::HashMap;

pub const CHARACTER_ASSET_PNG_1: &str = "characters/chara1.png";

pub struct PreloadResPlugin;

// pub const PRELOAD_RES_PLUGIN_LABEL:&str = "PRELOAD_RES_PLUGIN";

pub enum PreloadResPluginLabel{
    LoadSpriteTexture,
}
impl SystemLabel for PreloadResPluginLabel{
    fn as_str(&self) ->  &'static str {
        match self {
            PreloadResPluginLabel::LoadSpriteTexture=>"PreloadResPluginLabel::LoadSpriteTexture"
        }
    }
}

impl Plugin for PreloadResPlugin {
    fn build(&self, app: &mut App) {
        let h: HashMap<String, Handle<TextureAtlas>> = HashMap::with_capacity(10);
        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        // app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        //     .insert_resource(h)
        //     .add_startup_system(setup)
        //     .add_startup_system(add_main_character.after(setup))
        //     //.add_system(hello_world)
        //     .add_system(greet_characters)
        //     .add_system(sprite_movement);
        app.insert_resource(h)
            .add_startup_system(preload_sprites_res.label(PreloadResPluginLabel::LoadSpriteTexture));
    }
}

fn preload_sprites_res(
    mut texture_atlas_map: ResMut<HashMap<String, Handle<TextureAtlas>>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(CHARACTER_ASSET_PNG_1);
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(26., 34.),
        12,
        8,
        Vec2::new(0., 2.),
        Vec2::new(0., 2.),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    texture_atlas_map.insert(CHARACTER_ASSET_PNG_1.to_string(), texture_atlas_handle);

}

