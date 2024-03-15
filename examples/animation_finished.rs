//! Example for testing the crate as it is built

use bevy::{prelude::*, render::camera::ScalingMode, utils::HashMap};
use bevy_texture_loader::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, TextureLoaderPlugin));

    app.init_resource::<LoadedAnimations>();

    app.add_systems(Startup, setup);
    app.add_systems(Update, switch_animation_on_finish);

    app.run();
}

fn setup(mut commands: Commands, loaded_animations: Res<LoadedAnimations>) {
    info!("Spawning camera");
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: Color::BLACK.into(),
            ..Default::default()
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(3.0),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec2::ZERO.extend(5.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    info!("Spawning animation");
    commands.spawn((
        AnimationBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            },
            animation: loaded_animations
                .get_handle_for(ExampleAnimation::OneShotted)
                .unwrap(),
            ..Default::default()
        },
        NextAnimation(ExampleAnimation::Looped),
    ));
}

#[derive(Resource)]
struct LoadedAnimations(HashMap<ExampleAnimation, Handle<Animation>>);

impl FromWorld for LoadedAnimations {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let mut animations = HashMap::new();
        animations.insert(
            ExampleAnimation::OneShotted,
            asset_server.load("multi.animation#one_shot"),
        );
        animations.insert(
            ExampleAnimation::Looped,
            asset_server.load("multi.animation#loop"),
        );
        animations.insert(
            ExampleAnimation::Single,
            asset_server.load("single.animation"),
        );
        Self(animations)
    }
}

impl LoadedAnimations {
    pub fn get_handle_for(&self, animation: ExampleAnimation) -> Option<Handle<Animation>> {
        self.0.get(&animation).cloned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ExampleAnimation {
    OneShotted,
    Looped,
    Single,
}

#[derive(Component)]
struct NextAnimation(ExampleAnimation);

impl NextAnimation {
    pub fn next(&mut self) {
        self.0 = match self.0 {
            ExampleAnimation::OneShotted => ExampleAnimation::Looped,
            ExampleAnimation::Looped => ExampleAnimation::Single,
            ExampleAnimation::Single => ExampleAnimation::OneShotted,
        }
    }
}

fn switch_animation_on_finish(
    mut commands: Commands,
    mut e_animation_finished: EventReader<AnimationFinishedEvent>,
    loaded_animations: Res<LoadedAnimations>,
    mut q_animations: Query<&mut NextAnimation>,
) {
    for event in e_animation_finished.read() {
        info!("Animation finished, switching to the next one.");
        if let Ok(mut next_animation) = q_animations.get_mut(event.entity) {
            if let Some(handle) = loaded_animations.get_handle_for(next_animation.0) {
                commands.entity(event.entity).insert(handle);
                next_animation.next();
            }
        }
    }
}
