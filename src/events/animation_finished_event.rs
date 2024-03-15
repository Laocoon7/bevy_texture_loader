use bevy::prelude::*;

use crate::assets::AnimationType;

/// This [`Event`] is raised when an [`Animation`] finishes.
///
/// NOTE: This will fire on `AnimationType::Loop` at the end of the loop, but the loop will continue
#[derive(Event, Reflect, Debug, Clone)]
pub struct AnimationFinishedEvent {
    /// The [`AnimationType`] of this [`Animation`]
    pub animation_type: AnimationType,
    /// The [`Entity`] which caused this event to occur
    pub entity: Entity,
}
