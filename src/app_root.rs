use bevy::prelude::{Resource, Component};
use dioxus::prelude::{Scope, Element};

// TODO: Maybe make plugin setting instead of resource
#[derive(Resource)]
pub struct AppRootComponent(pub fn(Scope) -> Element);

#[derive(Component)]
pub struct AppRootElement;