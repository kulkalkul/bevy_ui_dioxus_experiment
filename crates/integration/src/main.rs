use app_root::AppRootComponent;
use bevy::{prelude::{App, Commands, Camera2dBundle, Startup, Update, Query}, DefaultPlugins, text::Text};

mod plugin;
mod integration;
mod app_root;
mod template_map;
mod element_map;
mod integration_data;
mod node;
mod nodes;

use dioxus::prelude::{Scope, Element, rsx, use_state, use_effect, to_owned};
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
    let count = use_state(cx, || 0);

    use_effect(cx, (count,), |(count,)| {
        to_owned![count];
        async move {
            count.modify(|x| x + 1);
        }
    });

    cx.render(rsx! {
        div {
            "This is a counter {count}"
        }
    })
}