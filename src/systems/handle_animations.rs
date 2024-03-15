use bevy::{asset::LoadState, prelude::*};

use crate::{
    assets::{Animation, AnimationType},
    components::AnimationData,
    events::AnimationFinishedEvent,
};

pub fn handle_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    a_animations: Res<Assets<Animation>>,
    mut e_animation_finished: EventWriter<AnimationFinishedEvent>,
    q_new_animations: Query<
        (Entity, &Handle<Animation>),
        Or<(
            Added<Handle<Animation>>,
            Changed<Handle<Animation>>,
            Without<AnimationData>,
        )>,
    >,
    mut q_animations: Query<(Entity, &Handle<Animation>, &mut AnimationData)>,
) {
    // Initialize new animations
    for (entity, animation_handle) in q_new_animations.iter() {
        if asset_server.load_state(animation_handle) != LoadState::Loaded {
            // Remove old animation data
            // This can happen if the animation is changed by the user
            // We only detect if a new animation is added or we don't have AnimationData
            // We need to make sure we don't have AnimationData
            commands.entity(entity).remove::<AnimationData>();
            #[cfg(feature = "trace")]
            trace!("Waiting for animation to load: {:?}", animation_handle);
            continue;
        }

        let Some(animation) = a_animations.get(animation_handle) else {
            // Remove old animation data
            // This can happen if the animation is changed by the user
            // We only detect if a new animation is added or we don't have AnimationData
            // We need to make sure we don't have AnimationData
            commands.entity(entity).remove::<AnimationData>();
            warn!(
                "Attempting to get non-existant animation: {:?}",
                animation_handle
            );
            continue;
        };

        commands.entity(entity).insert((
            AnimationData::new(animation.delay),
            animation.frames[0].clone(),
        ));
    }

    // Update existing animations
    for (entity, animation_handle, mut data) in q_animations.iter_mut() {
        if let Some(animation) = a_animations.get(animation_handle) {
            // Check timer finished
            data.timer.tick(time.delta());
            if data.timer.just_finished() {
                #[cfg(feature = "trace")]
                trace!("Animation timer finished: {:?}", animation_handle);
                match animation.animation_type {
                    AnimationType::OneShot => {
                        if data.index == animation.frames.len() - 1 {
                            #[cfg(feature = "trace")]
                            trace!("Animation finished: {:?}", animation_handle);

                            e_animation_finished.send(AnimationFinishedEvent {
                                animation_type: animation.animation_type,
                                entity,
                            });
                        } else {
                            data.index += 1;
                            commands
                                .entity(entity)
                                .insert(animation.frames[data.index].clone());
                        }
                    }
                    AnimationType::Loop => {
                        if data.index == animation.frames.len() - 1 {
                            #[cfg(feature = "trace")]
                            trace!("Animation finished: {:?}", animation_handle);

                            e_animation_finished.send(AnimationFinishedEvent {
                                animation_type: animation.animation_type,
                                entity,
                            });
                        }
                        data.index += 1;
                        data.index %= animation.frames.len();
                        commands
                            .entity(entity)
                            .insert(animation.frames[data.index].clone());
                    }
                }
            }
        }
    }
}
