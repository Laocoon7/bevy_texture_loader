use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};

use crate::assets::{serialized::AnimationSerialized, Animation};

#[derive(Default)]
pub struct AnimationLoader;

impl AssetLoader for AnimationLoader {
    type Asset = Animation;
    type Settings = ();
    type Error = anyhow::Error;

    fn extensions(&self) -> &[&str] {
        &["animation", "animations"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            #[cfg(feature = "trace")]
            trace!("Loading `Animation`");

            // Read the serialized version
            let mut contents = Vec::new();
            reader.read_to_end(&mut contents).await?;
            let animations_serialized = ron::de::from_bytes::<AnimationSerialized>(&contents)?;

            // Get the animation asset we will return
            let animation = match animations_serialized {
                AnimationSerialized::Single(animation_serialized) => {
                    let mut frames = Vec::new();
                    for path in animation_serialized.frames.iter() {
                        frames.push(load_context.load(path));
                    }

                    Animation {
                        animation_type: animation_serialized.animation_type,
                        delay: 1.0 / animation_serialized.frames_per_second,
                        frames,
                    }
                }
                AnimationSerialized::Multi(values) => {
                    // we need to provide a default animation incase the label is not set
                    let mut default_animation = None;

                    for (name, animation_serialized) in values.iter() {
                        #[cfg(feature = "trace")]
                        trace!("Found animation: {}", name);

                        let mut frames = Vec::new();
                        for path in animation_serialized.frames.iter() {
                            frames.push(load_context.load(path));
                        }

                        let animation = Animation {
                            animation_type: animation_serialized.animation_type,
                            delay: 1.0 / animation_serialized.frames_per_second,
                            frames,
                        };

                        // set the default_animation if one is not set
                        if default_animation.is_none() {
                            default_animation = Some(animation.clone());
                        }

                        load_context.add_labeled_asset(name.to_string(), animation);
                    }

                    // return the first or default animation incase the label is not set
                    default_animation.unwrap_or_default()
                }
            };

            Ok(animation)
        })
    }
}
