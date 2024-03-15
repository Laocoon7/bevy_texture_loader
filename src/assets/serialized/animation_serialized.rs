use bevy::{asset::AssetPath, utils::HashMap};
use serde::Deserialize;

use crate::assets::AnimationType;

#[derive(Deserialize, Debug, Clone)]
pub struct SingleAnimationSerialized {
    pub animation_type: AnimationType,
    pub frames_per_second: f32,
    pub frames: Vec<AssetPath<'static>>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum AnimationSerialized {
    Single(SingleAnimationSerialized),
    Multi(HashMap<String, SingleAnimationSerialized>),
}
