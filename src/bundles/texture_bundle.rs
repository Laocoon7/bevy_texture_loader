use bevy::prelude::*;

use crate::assets::Texture;

/// A [`Bundle`] of components for drawing a single sprite from an [`Image`] or [`TextureAtlas`]
///
/// Note:
/// This bundle is identical to [`SpriteBundle`] or [`SpriteSheetBundle`] but where the [`Texture`] is loaded from a `.texture` asset file.
#[derive(Bundle, Clone, Default)]
pub struct TextureBundle {
    /// Specifies the rendering properties of the sprite, such as color tint and flip.
    pub sprite: Sprite,
    /// The local transform of the sprite, relative to its parent.
    pub transform: Transform,
    /// The absolute transform of the sprite. This should generally not be written to directly.
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,

    /// The texture definition which will convert this object into either a Sprite or SpriteSheet
    pub texture: Handle<Texture>,
}
