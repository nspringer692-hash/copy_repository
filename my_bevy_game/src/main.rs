use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn(Sprite {
        color: Color::srgb(0.3, 0.6, 1.0),
        custom_size: Some(Vec2::new(80.0, 80.0)),
        ..default()
    });
}