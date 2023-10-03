use bevy::prelude::{Plugin, Update, Startup};

use crate::integration::{update_dioxus, setup_dioxus};

pub struct DioxusPlugin;

impl Plugin for DioxusPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, setup_dioxus)
            .add_systems(Update, update_dioxus);
    }
}