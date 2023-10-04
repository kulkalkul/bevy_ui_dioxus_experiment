use bevy::{prelude::{World, BuildWorldChildren, Entity}, ecs::world::EntityMut};
use dioxus::core::{Mutations, Mutation, ElementId};

use crate::{template_map::TemplateMap, element_map::ElementMap, node::{Element, NodeChild, ChildNode, RootNode, NodeChildrenTree}};

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
            match edit {
            //     Mutation::AppendChildren { id, m } => todo!(),
            //     Mutation::AssignId { path, id } => todo!(),
            //     Mutation::CreatePlaceholder { id } => todo!(),
            //     Mutation::CreateTextNode { value, id } => println!("create_text_node = {value}"),
            //     Mutation::HydrateText { path, value, id } => todo!(),
                Mutation::LoadTemplate {
                    name, index, id
                } => self.load_template(world, name, index, id),
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
            _ => (),
            }
        }
    }
    fn load_template(&mut self, world: &mut World, name: &str, index: usize, element_id: ElementId) {
        match &self.template_map.map[name][index] {
            RootNode::ElementWithChildren {
                element,
                children,
            } => {
                let parent = match element {
                    Element::Div { node } => world.spawn(node.bundle()),
                    Element::Image { node } => world.spawn(node.bundle()),
                    Element::Button { node } => world.spawn(node.bundle()),
                }.id();

                let mut current = parent;
                let mut created = Entity::PLACEHOLDER;
                let mut stack = Vec::with_capacity(children.nodes.len());

                for node_child in &children.nodes {
                    match node_child {
                        NodeChild::Node(node) => {
                            world.entity_mut(current).with_children(|builder| {
                                created = match node {
                                    ChildNode::Element { element } => match element {
                                        Element::Div { node } => builder.spawn(node.bundle()),
                                        Element::Image { node } => builder.spawn(node.bundle()),
                                        Element::Button { node } => builder.spawn(node.bundle()),
                                    },
                                    ChildNode::Text { node } => builder.spawn(node.bundle()),
                                    ChildNode::PlaceHolder => todo!(),
                                }.id();
                            });
                        },
                        NodeChild::In => {
                            stack.push(current);
                            current = created;
                        },
                        NodeChild::Out => {
                            current = stack.pop().expect("shouldn't have empty vec at this stage");
                        },
                    }
                }
            },
            RootNode::Element { element } => {
                match element {
                    Element::Div { node } => world.spawn(node.bundle()),
                    Element::Image { node } => world.spawn(node.bundle()),
                    Element::Button { node } => world.spawn(node.bundle()),
                };
            },
            RootNode::Text { node } => {
                world.spawn(node.bundle());
            },
            RootNode::PlaceHolder => todo!(),
        }
    }
}