// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use bevy::{
    app::{App, Plugin, Update},
    math::Vec3,
    prelude::{Changed, Component, Query, Transform},
    ui::Interaction,
};

pub struct UiSupportPlugin;

impl Plugin for UiSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_press_down_effect);
    }
}

#[derive(Component, Default)]
pub struct PressEffect {
    pub pressing: bool,
    pub orig_scale: Vec3,
}

pub fn button_press_down_effect(
    mut query: Query<(&mut Transform, &mut PressEffect, &Interaction), Changed<Interaction>>,
) {
    for (mut transform, mut state, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                if !state.pressing {
                    state.orig_scale = transform.scale;
                    state.pressing = true;
                    transform.scale.x *= 0.8;
                    transform.scale.y *= 0.8;
                }
            }
            Interaction::Hovered => {
                if state.pressing {
                    state.pressing = false;
                    transform.scale = state.orig_scale;
                }
            }
            Interaction::None => {
                if state.pressing {
                    state.pressing = false;
                    transform.scale = state.orig_scale;
                }
            }
        }
    }
}
