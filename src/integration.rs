use bevy::prelude::{World, Entity, default};
use dioxus::{prelude::{VirtualDom}, core::Mutations};

use crate::app_root::AppRootComponent;

pub struct Dioxus {
    vdom: VirtualDom,
    integration_data: IntegrationData,
}

#[derive(Default)]
struct IntegrationData {
    entity_map: EntityMap,
}

#[derive(Default)]
struct EntityMap {
    map: Vec<Entity>,
}

pub fn setup_dioxus(world: &mut World) {
    let app_root = world.get_resource::<AppRootComponent>()
        .expect("AppRoot resource should exist");

    let mut vdom = VirtualDom::new(app_root.0.clone());
    let mutations = vdom.rebuild();

    let mut integration_data = IntegrationData::default();

    update_dom(world, mutations, &mut integration_data);

    world.insert_non_send_resource(Dioxus {
        vdom,
        integration_data,
    });
}

pub fn update_dioxus(world: &mut World) {
    let mut dioxus = world.remove_non_send_resource::<Dioxus>().expect("Dioxus resource should exist");

    let mutations = dioxus.vdom.render_immediate();
    let mut integration_data = std::mem::take(&mut dioxus.integration_data);

    update_dom(world, mutations, &mut integration_data);

    dioxus.integration_data = integration_data;

    world.insert_non_send_resource(dioxus);
}

fn update_dom(world: &mut World, mutations: Mutations, integration_data: &mut IntegrationData) {
    println!("{mutations:?}");
    for edit in mutations.edits {
        // match edit {
        //     Mutation::AppendChildren { id, m } => todo!(),
        //     Mutation::AssignId { path, id } => todo!(),
        //     Mutation::CreatePlaceholder { id } => todo!(),
        //     Mutation::CreateTextNode { value, id } => println!("create_text_node = {value}"),
        //     Mutation::HydrateText { path, value, id } => todo!(),
        //     Mutation::LoadTemplate { name, index, id } => todo!(),
        //     Mutation::ReplaceWith { id, m } => todo!(),
        //     Mutation::ReplacePlaceholder { path, m } => todo!(),
        //     Mutation::InsertAfter { id, m } => todo!(),
        //     Mutation::InsertBefore { id, m } => todo!(),
        //     Mutation::SetAttribute { name, value, id, ns } => todo!(),
        //     Mutation::SetText { value, id } => println!("set_text = {value}"),
        //     Mutation::NewEventListener { name, id } => todo!(),
        //     Mutation::RemoveEventListener { name, id } => todo!(),
        //     Mutation::Remove { id } => todo!(),
        //     Mutation::PushRoot { id } => println!(""),
        // }
    }
}