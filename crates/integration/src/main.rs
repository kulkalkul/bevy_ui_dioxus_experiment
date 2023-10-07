use app_root::AppRootComponent;
use bevy::{prelude::{App, Commands, Camera2dBundle, Startup}, DefaultPlugins};

mod plugin;
mod integration;
mod app_root;
mod template_map;
mod element_map;
mod integration_data;
mod node;
mod nodes;
mod attributes;

use dioxus::prelude::{Scope, Element, rsx};
use plugin::DioxusPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


use bevy_ui_dioxus_elements as dioxus_elements;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DioxusPlugin,
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(AppRootComponent(app_root))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands,) {
    commands.spawn(Camera2dBundle::default());
}


fn app_root(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "test"
        }
    })
}