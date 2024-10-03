use bevy::color::palettes::tailwind::GRAY_400;
use bevy::prelude::*;
use bevy_support::ui::{ButtonState, UiSupportPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(UiSupportPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        ButtonState::default(),
        ButtonBundle {
            style: Style {
                width: Val::Px(150.),
                height: Val::Px(50.),
                ..default()
            },
            background_color: GRAY_400.into(),
            ..default()
        },
    ));
}
