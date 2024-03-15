use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assets::Texture;

/// Defines the run type of the [`Animation`]
#[derive(Serialize, Deserialize, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AnimationType {
    #[default]
    /// Run the [`Animation`] once
    OneShot,
    /// Run the [`Animation`] in a loop
    Loop,
}

/// This [`Asset`] is defined by it's accompanying '.animation' file and is used to easily add animations to objects
#[derive(Asset, Reflect, Debug, Default, Clone)]
pub struct Animation {
    /// Defines the [`AnimationType`]
    pub animation_type: AnimationType,
    /// Defines the amount of time between each frame
    pub delay: f32,
    /// Defines each frame with a [`Texture`]
    pub frames: Vec<Handle<Texture>>,
}
