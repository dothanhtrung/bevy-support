// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//! ### Plugin
//! doc goes here

use bevy::prelude::{Component, Query, Visibility, With};

pub mod ui;

pub fn show_ui<T>(mut mainmenu: Query<&mut Visibility, With<T>>)
where
    T: Component,
{
    for mut visibility in mainmenu.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_ui<T>(mut mainmenu: Query<&mut Visibility, With<T>>)
where
    T: Component,
{
    for mut visibility in mainmenu.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}
