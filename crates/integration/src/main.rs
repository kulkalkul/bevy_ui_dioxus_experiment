use app_root::AppRootComponent;
use bevy::{prelude::App, DefaultPlugins};

mod plugin;
mod integration;
mod app_root;
mod template_map;
mod element_map;
mod node;

use dioxus::prelude::{Scope, Element, rsx};
use plugin::DioxusPlugin;

use bevy_ui_dioxus_elements as dioxus_elements;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DioxusPlugin,
        ))
        .insert_resource(AppRootComponent(app_root))
        .run();
}

fn app_root(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            div {
                div { "0" }
            }
            div { "1" }
            div { "2" }
        }
    })
}