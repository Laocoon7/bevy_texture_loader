use bevy::prelude::*;

use crate::assets::Animation;

/// A [`Bundle`] of components for drawing an animated sprite
///
/// Note:
/// This bundle is identical to [`SpriteBundle`] or [`SpriteSheetBundle`] but where the [`Animation`] is loaded from a `.animation` asset file.
#[derive(Bundle, Clone, Default)]
pub struct AnimationBundle {
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

    /// The animation definition which will convert this object into either a Sprite or SpriteSheet
    pub animation: Handle<Animation>,
}
