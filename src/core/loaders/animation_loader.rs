use bevy::{
    asset::{LoadContext, Error, BoxedFuture, AssetLoader, LoadedAsset}, reflect::TypeUuid, prelude::{Deref, Component, DerefMut}, utils::hashbrown::HashMap
};

#[derive(Default)]
pub struct AnimationLoader;
// Create the animation state component
#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(benimator::State);

#[derive(TypeUuid, Deref, Debug)]
#[uuid = "ae6a74db-f6fa-43c4-ac16-01d13b50e4c6"]
pub struct Animation(pub benimator::Animation);

impl AssetLoader for AnimationLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        match toml::from_slice::<benimator::Animation>(bytes) {
            Ok(value) => {
                let animation: Animation = Animation(value);
                load_context.set_default_asset(LoadedAsset::new(animation));
        
                Box::pin(async move { Ok(()) })
            },
            Err(err) =>  Box::pin(async move { anyhow::bail!(err) })
        }
        
    }

    fn extensions(&self) -> &[&str] {
        &["animation.toml"]
    }
}

