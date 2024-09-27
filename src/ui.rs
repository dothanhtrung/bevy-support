use bevy::{
    app::{App, Plugin, Update},
    math::Vec3,
    prelude::{Button, Changed, Component, Query, Transform, With},
    ui::Interaction,
};

pub struct UiSupportPlugin;

impl Plugin for UiSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_press_down_effect);
    }
}

#[derive(Component, Default)]
pub struct ButtonState {
    pub pressed: bool,
    pub orig_translation: Vec3,
}

pub fn button_press_down_effect(
    mut query: Query<(&mut Transform, &mut ButtonState, &Interaction), (With<Button>, Changed<Interaction>)>,
) {
    for (mut transform, mut state, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                if !state.pressed {
                    state.orig_translation = transform.translation;
                    state.pressed = true;
                    transform.translation.y -= 2.;
                }
            }
            Interaction::Hovered => {
                if state.pressed {
                    state.pressed = false;
                    transform.translation = state.orig_translation;
                }
            }
            Interaction::None => {
                if state.pressed {
                    state.pressed = false;
                    transform.translation = state.orig_translation;
                }
            }
        }
    }
}
