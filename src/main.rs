#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod ui;
mod ui_scroll;

use bevy::{
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme},
    prelude::*,
    window::WindowResolution,
};

use ai::AiPlugin;
use ui::UiPlugin;
use ui_scroll::UiScrollPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(480, 720),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }),
        FeathersPlugins,
        AiPlugin,
        UiPlugin,
        UiScrollPlugin,
    ));

    app.insert_resource(UiTheme(create_dark_theme())).run();
}
