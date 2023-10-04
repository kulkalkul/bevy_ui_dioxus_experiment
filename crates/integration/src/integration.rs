use bevy::prelude::World;
use dioxus::prelude::VirtualDom;

use crate::{app_root::AppRootComponent, integration_data::IntegrationData};

pub struct Dioxus {
    vdom: VirtualDom,
    integration_data: IntegrationData,
}

pub fn setup_dioxus(world: &mut World) {
    let app_root = world.get_resource::<AppRootComponent>()
        .expect("AppRoot resource should exist");

    let mut vdom = VirtualDom::new(app_root.0.clone());
    let mutations = vdom.rebuild();

    let mut integration_data = IntegrationData::default();

    integration_data.update_dom(world, mutations);

    world.insert_non_send_resource(Dioxus {
        vdom,
        integration_data,
    });
}

pub fn update_dioxus(world: &mut World) {
    let mut dioxus = world.remove_non_send_resource::<Dioxus>().expect("Dioxus resource should exist");

    let mutations = dioxus.vdom.render_immediate();
    let mut integration_data = std::mem::take(&mut dioxus.integration_data);

    integration_data.update_dom(world, mutations);

    dioxus.integration_data = integration_data;

    world.insert_non_send_resource(dioxus);
}
