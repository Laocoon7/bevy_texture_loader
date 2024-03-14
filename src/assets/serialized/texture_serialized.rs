use bevy::{asset::AssetPath, prelude::*, utils::HashMap};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum TextureSerialized {
    Image(AssetPath<'static>),
    Atlas {
        image: AssetPath<'static>,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
        padding: Option<Vec2>,
        offset: Option<Vec2>,
        indices: HashMap<String, usize>,
    },
}
