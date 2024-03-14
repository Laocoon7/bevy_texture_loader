use bevy::{asset::LoadState, prelude::*};

use crate::assets::Texture;

// TODO: Can this be event based?
pub fn convert_texture_to_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    a_textures: Res<Assets<Texture>>,
    mut q_textures: Query<(
        Entity,
        &Handle<Texture>,
        Option<&mut Handle<Image>>,
        Option<&mut TextureAtlas>,
    )>,
) {
    for (entity, handle, maybe_old_image_handle, maybe_old_atlas) in q_textures.iter_mut() {
        if asset_server.load_state(handle) != LoadState::Loaded {
            #[cfg(feature = "trace")]
            println!("Waiting for texture to load: {:?}", handle);
            continue;
        }

        let Some(texture) = a_textures.get(handle) else {
            warn!("Attempting to get non-existant texture: {:?}", handle);
            continue;
        };

        #[cfg(feature = "trace")]
        println!("Converting `Handle<Texture>` into a valid image");

        if let Some(mut image_handle) = maybe_old_image_handle {
            // update old [`Handle<Image>`]
            *image_handle = texture.image_handle();
        } else {
            // add new [`Handle<Image>`]
            commands.entity(entity).insert(texture.image_handle());
        }

        if let Some(new_texture_atlas) = texture.texture_atlas() {
            if let Some(mut atlas) = maybe_old_atlas {
                // update old [`TextureAtlas`]
                *atlas = new_texture_atlas;
            } else {
                // add new [`TextureAtlas`]
                commands.entity(entity).insert(new_texture_atlas);
            }
        } else {
            // remove any old [`TextureAtlas`]
            commands.entity(entity).remove::<TextureAtlas>();
        }

        // remove _this_ [`Handle<Texture>`] as we are done processing it
        commands.entity(entity).remove::<Handle<Texture>>();
    }
}
