use std::{cell::RefCell, rc::Rc};

use bevy::prelude::{World, NodeBundle};
use dioxus::prelude::VirtualDom;

use crate::{app_root::{AppRootComponent, AppRootElement}, integration_data::IntegrationData};

pub struct Dioxus {
    vdom: VirtualDom,
    integration_data: IntegrationData,
}

pub type BevyWorld = Rc<RefCell<World>>;

pub fn setup_dioxus(world: &mut World) {
    let app_root = world.get_resource::<AppRootComponent>()
        .expect("AppRoot resource should exist");

    let mut vdom = VirtualDom::new(app_root.0.clone());
    
    vdom.base_scope().provide_context(Rc::new(RefCell::new(World::new())));
    let context_world = vdom.base_scope().consume_context::<BevyWorld>().unwrap();
    std::mem::swap(world, &mut context_world.borrow_mut());

    let mutations = vdom.rebuild();

    let mut integration_data = IntegrationData::default();
    let root_entity = context_world.borrow_mut().spawn((NodeBundle::default(), AppRootElement)).id();
    integration_data.set_root(root_entity);

    integration_data.update_dom(&mut context_world.borrow_mut(), mutations);

    std::mem::swap(world, &mut context_world.borrow_mut());

    world.insert_non_send_resource(Dioxus {
        vdom,
        integration_data,
    });
}

pub fn update_dioxus(world: &mut World) {
    let mut dioxus = world
        .remove_non_send_resource::<Dioxus>()
        .expect("Dioxus resource should exist");

    let context_world = dioxus.vdom.base_scope().consume_context::<BevyWorld>().unwrap();
    std::mem::swap(world, &mut context_world.borrow_mut());

    let mutations = dioxus.vdom.render_immediate();
    let mut integration_data = std::mem::take(&mut dioxus.integration_data);

    integration_data.update_dom(&mut context_world.borrow_mut(), mutations);

    std::mem::swap(world, &mut context_world.borrow_mut());

    dioxus.integration_data = integration_data;

    world.insert_non_send_resource(dioxus);
}
