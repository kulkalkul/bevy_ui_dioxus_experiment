use bevy::prelude::World;
use dioxus::core::Mutations;

use crate::{template_map::TemplateMap, element_map::ElementMap};

#[derive(Default, Debug)]
pub struct IntegrationData {
    template_map: TemplateMap,
    element_map: ElementMap,
}

impl IntegrationData {
    pub fn update_dom(&mut self, world: &mut World, mutations: Mutations) {
        for template in mutations.templates {
            self.template_map.add(template);
        }
    
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
}