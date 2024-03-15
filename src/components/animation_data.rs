use bevy::prelude::*;

/// A component for holding the timer and index responsible for animations.
#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub struct AnimationData {
    /// Holds the timer for this animation
    pub timer: Timer,
    /// Holds the index for this animation
    pub index: usize,
}

impl AnimationData {
    pub(crate) fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
            index: 0,
        }
    }
}
