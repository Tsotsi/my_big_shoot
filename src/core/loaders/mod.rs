use std::path::{Path, PathBuf};

use bevy::{
    asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset},
    prelude::{AssetServer, Component, Deref, Handle, Res, Vec2, DerefMut},
    reflect::TypeUuid,
    utils::hashbrown::HashMap,
};
use serde::{Deserialize, Serialize};
pub mod animation_loader;

#[derive(Default)]
pub struct MainConfigLoader;

#[derive(TypeUuid, Deserialize, Serialize, Debug)]
#[uuid = "6b27b1e1-4f61-47a4-bf72-f91aca0209be"]
pub struct MainConfig {
    pub title: String,
    pub size: Vec2,
    pub characters: HashMap<String, CharacterCfg>,
}

/// [characters.main]
// name = "chara_1_2"
// img = "characters/chara1.png"
// sprite_idx = 3
// size = [26,34]
// rows = 8
// columns = 12
// padding = [0,2]
// offset = [0,2]
// animations = ["walk_down","walk_up","walk_left","walk_right"]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CharacterCfg {
    pub name: String,
    pub img: String,
    pub sprite_idx: usize,
    pub size: Vec2,
    pub rows: u32,
    pub columns: u32,
    pub padding: Vec2,
    pub offset: Vec2,
    pub animations: Vec<String>,
}

impl AssetLoader for MainConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        match toml::from_slice::<MainConfig>(bytes) {
            Ok(value) => {
                load_context.set_default_asset(LoadedAsset::new(value));

                Box::pin(async move { Ok(()) })
            }
            Err(err) => Box::pin(async move { anyhow::bail!(err) }),
        }
    }

    fn extensions(&self) -> &[&str] {
        &["cfg.toml"]
    }
}

impl CharacterCfg {
    pub fn gen_animation_file_path(self) -> Vec<String> {
        let path = Path::new(&self.img).parent().expect("no parent dir");
        let mut res: Vec<String> = Vec::new();
        for animation_name in self.animations.iter() {
            res.push(
                path.join(format!(
                    "animations/{}_{}.animation.toml",
                    self.name, animation_name
                ))
                .to_str()
                .unwrap().to_string(),
            );
        }
        res
    }
}


#[derive(Component, Deref, DerefMut)]
pub struct AnimationMap(HashMap<String, Handle<animation_loader::Animation>>);

impl AnimationMap{
    pub fn new() -> Self {
        AnimationMap(HashMap::new())
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationNameMap(HashMap<String, String>);

impl From<CharacterCfg> for AnimationNameMap {
    fn from(cfg: CharacterCfg) -> Self {
        let path = Path::new(&cfg.img).parent().expect("no parent dir");
        let mut res: AnimationNameMap = AnimationNameMap(HashMap::new());
        for animation_name in cfg.animations.iter() {
            res.0.insert(
                animation_name.to_string(),
                path.join(format!(
                    "animations/{}_{}.animation.toml",
                    cfg.name, animation_name
                ))
                .to_str()
                .unwrap()
                .to_string(),
            );
        }
        res
    }
}
