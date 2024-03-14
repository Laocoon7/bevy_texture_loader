//! Contains the [`TextureLoaderPlugin`]

/// The module containing the data structs for each [`Asset`] which can be loaded by this plugin
pub mod assets;
/// The module containing the bundles intended to build the objects which interact with this plugin
pub mod bundles;
pub(crate) mod loaders;
pub(crate) mod systems;

mod texture_loader_plugin;
pub use self::texture_loader_plugin::*;

mod texture_loader_schedule;
pub use self::texture_loader_schedule::*;
