use app_root::AppRootComponent;
use attributes::Attr;
use bevy::{prelude::{App, Commands, Camera2dBundle, PreStartup, Component}, DefaultPlugins, ui::Val};

mod plugin;
mod integration;
mod app_root;
mod template_map;
mod element_map;
mod integration_data;
mod ui_node;
mod bevy_node;
mod attributes;
mod hooks;

use dioxus::prelude::{Scope, Element, rsx, use_state, use_effect, to_owned};
use hooks::{use_query, use_world};
use plugin::DioxusPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_ui_dioxus_elements::GlobalAttributes;
use bevy_ui_dioxus_elements as dioxus_elements;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DioxusPlugin,
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(AppRootComponent(app_root))
        .add_systems(PreStartup, setup)
        .run();
}

#[derive(Component)]
struct Count {
    value: usize,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Count { value: 5 });
}

fn app_root(cx: Scope) -> Element {
    let world = use_world(cx);

    let mut count = use_query::<&Count>(world);
    let count = count.single(&world.borrow()).value;

    let left = use_state(cx, || 50.0);

    cx.render(rsx! {
        div {
            left: Attr(Val::Px(*left.get())),
            "Counter: {count}"
        }
    })
}