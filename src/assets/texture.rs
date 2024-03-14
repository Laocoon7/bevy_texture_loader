use bevy::prelude::*;

/// This [`Asset`] is defined by it's accompanying '.texture' file and is used to easily add [`Image`]s and [`TextureAtlas`]es to objects
#[derive(Asset, Reflect, Debug, Clone)]
pub enum Texture {
    /// Defines a single [`Image`]
    Image(Handle<Image>),
    /// Defines a single [`Image`] inside of a [`TextureAtlas`]
    Atlas {
        /// Holds the [`Handle`] to the parent [`Image`]
        image_handle: Handle<Image>,
        /// Holds the [`Handle`] to the parent [`TextureAtlasLayout`]
        layout_handle: Handle<TextureAtlasLayout>,
        /// Holds the index requested from the [`TextureAtlas`]
        index: usize,
    },
}

impl Default for Texture {
    fn default() -> Self {
        Self::Image(Handle::default())
    }
}

impl Texture {
    /// Retrieve the [`Handle<Image>`]
    pub fn image_handle(&self) -> Handle<Image> {
        match self {
            Texture::Image(image_handle) | Texture::Atlas {image_handle, layout_handle: _, index: _,} => image_handle.clone(),
        }
    }

    /// Retrieve the [`TextureAtlas`] if one is present
    pub fn texture_atlas(&self) -> Option<TextureAtlas> {
        match self {
            Texture::Image(_image_handle) => None,
            Texture::Atlas {
                image_handle: _,
                layout_handle,
                index,
            } => Some(TextureAtlas {
                layout: layout_handle.clone(),
                index: *index,
            }),
        }
    }
}
