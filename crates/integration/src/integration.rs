use bevy::{prelude::{World, Entity, default, NodeBundle}, utils::HashMap};
use dioxus::{prelude::{VirtualDom, TemplateNode, Template}, core::Mutations};

use crate::app_root::AppRootComponent;

pub struct Dioxus {
    vdom: VirtualDom,
    integration_data: IntegrationData,
}

#[derive(Default)]
struct IntegrationData {
    template_map: TemplateMap,
    element_map: ElementMap,
}

enum NodeType {
    Div {
        
    },
    Image {

    },
    Button {

    },
    Text {
        text: String,
    },
    PlaceHolder,
}

#[derive(Default)]
struct TemplateMap {
    map: HashMap<String, Vec<NodeType>>,
}

impl TemplateMap {
    fn add(&mut self, template: Template) {
        let mut template_roots = Vec::with_capacity(template.roots.len());
        
        for node in template.roots {
            let node_type = self.create_node(template.name.to_string(), node);
            template_roots.push(node);
        }

        self.map.insert(template.name.to_string(), template_roots);
    }
    fn create_node(&mut self, name: String, node: &TemplateNode) -> NodeType {
        let node_type = match node {
            TemplateNode::Element {
                tag,
                namespace,
                attrs,
                children,
            } => {
                // TODO: Handle child nodes
                for node in *children {
                    self.create_node(name.clone(), node);
                }

                // Wish I could avoid using string for tags
                match *tag {
                    "div" => NodeType::Div {
                    },
                    "img" => NodeType::Image {
                    },
                    "button" => NodeType::Button {
                    },
                    _ => panic!("Invalid tag, this shouldn't happen"),
                }
            },
            TemplateNode::Text { text } => NodeType::Text {
                text: text.to_string(),
            },
            TemplateNode::Dynamic { id } => NodeType::PlaceHolder,
            TemplateNode::DynamicText { id } => NodeType::Text {
                text: "".to_string(),
            },
        };
    }
}

#[derive(Default)]
struct ElementMap {
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
    for template in mutations.templates {
        integration_data.template_map.add(template);
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