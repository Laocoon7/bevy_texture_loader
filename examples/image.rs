//! Example for testing the crate as it is built

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_sprite_loader::TextureLoaderPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, TextureLoaderPlugin));

    app.add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    info!("Spawning sprite");
    commands.spawn(bevy_sprite_loader::bundles::TextureBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::ONE),
            ..Default::default()
        },
        texture: asset_server.load("dot.texture"),
        ..Default::default()
    });
}
