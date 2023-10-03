use app_root::AppRootComponent;
use bevy::{prelude::App, DefaultPlugins};

mod plugin;
mod integration;
mod app_root;

use dioxus::prelude::{Scope, Element, rsx};
use plugin::DioxusPlugin;

use dioxus::html as dioxus_elements;

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
            div { "hello world" }
            div { "world hello" }
        }
    })
}