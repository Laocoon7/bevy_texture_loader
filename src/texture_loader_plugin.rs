use bevy::{app::MainScheduleOrder, prelude::*};

use crate::{
    assets::{Animation, Texture},
    loaders::{AnimationLoader, TextureLoader},
    prelude::AnimationFinishedEvent,
    systems::{convert_texture_to_sprite, handle_animations},
    TextureLoaderSchedule, TextureLoaderSet,
};

/// This plugin provides functionality to define sprites in '.texture' files
pub struct TextureLoaderPlugin;

impl Plugin for TextureLoaderPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "trace")]
        println!("bevy_sprite_loader::sprite_loader_plugin\tInstalling SpriteLoaderPlugin");

        app.add_schedule(TextureLoaderSchedule::base_schedule());
        let mut main_schedule_order = app.world.resource_mut::<MainScheduleOrder>();
        main_schedule_order.insert_after(Update, TextureLoaderSchedule);

        #[cfg(feature = "animation")]
        app.init_asset::<Animation>();
        #[cfg(feature = "animation")]
        app.init_asset_loader::<AnimationLoader>();
        #[cfg(feature = "animation")]
        app.add_event::<AnimationFinishedEvent>();

        app.init_asset::<Texture>();
        app.init_asset_loader::<TextureLoader>();

        #[cfg(feature = "animation")]
        app.add_systems(
            TextureLoaderSchedule,
            handle_animations.in_set(TextureLoaderSet::Animate),
        );

        app.add_systems(
            TextureLoaderSchedule,
            convert_texture_to_sprite.in_set(TextureLoaderSet::Convert),
        );
    }
}
