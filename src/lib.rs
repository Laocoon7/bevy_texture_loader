//! Contains the [`TextureLoaderPlugin`]

/// The module containing the data structs for each [`Asset`] which can be loaded by this plugin
pub mod assets;
/// The module containing the bundles intended to build the objects which interact with this plugin
pub mod bundles;
/// The module containing the components used by this plugin
pub mod components;
/// The module containing the events used by this plugin
pub mod events;
pub(crate) mod loaders;
pub(crate) mod systems;

mod texture_loader_plugin;
pub use self::texture_loader_plugin::*;

mod texture_loader_schedule;
pub use self::texture_loader_schedule::*;

/// Prelude for `BevyTextureLoader`
pub mod prelude {
    #[cfg(feature = "animation")]
    pub use crate::assets::{Animation, AnimationType};
    #[cfg(feature = "animation")]
    pub use crate::bundles::AnimationBundle;
    #[cfg(feature = "animation")]
    pub use crate::events::AnimationFinishedEvent;

    pub use crate::assets::Texture;
    pub use crate::bundles::TextureBundle;

    pub use crate::TextureLoaderPlugin;
}
