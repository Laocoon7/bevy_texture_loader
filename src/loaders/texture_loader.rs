use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};

use crate::assets::{serialized::TextureSerialized, Texture};

#[derive(Default)]
pub struct TextureLoader;

impl AssetLoader for TextureLoader {
    type Asset = Texture;
    type Settings = ();
    type Error = anyhow::Error;

    fn extensions(&self) -> &[&str] {
        &["texture"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            #[cfg(feature = "trace")]
            println!("Loading `Texture`");

            let mut contents = Vec::new();
            reader.read_to_end(&mut contents).await?;
            let texture_serialized = ron::de::from_bytes::<TextureSerialized>(&contents)?;

            let texture = match texture_serialized {
                TextureSerialized::Image(path) => Texture::Image(load_context.load(path)),
                TextureSerialized::Atlas {
                    image,
                    tile_size,
                    columns,
                    rows,
                    padding,
                    offset,
                    indices,
                } => {
                    let layout =
                        TextureAtlasLayout::from_grid(tile_size, columns, rows, padding, offset);
                    let layout_handle =
                        load_context.add_labeled_asset("layout".to_string(), layout);

                    let image = load_context.load(image);

                    for (name, index) in indices.iter() {
                        #[cfg(feature = "trace")]
                        println!("Found atlas image: {}", name);
                        load_context.add_labeled_asset(
                            name.to_string(),
                            Texture::Atlas {
                                image_handle: image.clone(),
                                layout_handle: layout_handle.clone(),
                                index: *index,
                            },
                        );
                    }

                    Texture::Image(image)
                }
            };

            Ok(texture)
        })
    }
}
