use bevy::{prelude::{World, Entity, default, NodeBundle, TextBundle, ImageBundle, ButtonBundle}, utils::HashMap, text::{Text, TextStyle}};
use dioxus::{prelude::{VirtualDom, TemplateNode, Template}, core::Mutations};

use crate::app_root::AppRootComponent;

pub struct Dioxus {
    vdom: VirtualDom,
    integration_data: IntegrationData,
}

#[derive(Default, Debug)]
struct IntegrationData {
    template_map: TemplateMap,
    element_map: ElementMap,
}

#[derive(Debug)]
enum Node {
    ElementWithChildren {
        element: Element,
        children: NodeChildrenTree,
    },
    Element {
        element: Element,
    },
    Text {
        bundle: TextBundle,
    },
    PlaceHolder,
}

#[derive(Debug)]
enum Element {
    Div {
        bundle: NodeBundle,
    },
    Image {
        bundle: ImageBundle,
    },
    Button {
        bundle: ButtonBundle,
    },
}

#[derive(Debug)]
struct NodeChildrenTree {
    nodes: Vec<NodeChild>,
}

impl NodeChildrenTree {
    fn placeholder(&mut self) -> usize {
        self.nodes.push(NodeChild::Node(Node::PlaceHolder));
        self.nodes.len() - 1
    }
    fn replace(&mut self, index: usize, child: NodeChild) {
        self.nodes[index] = child;
    }
    fn add(&mut self, child: NodeChild) {
        self.nodes.push(child);
    }
}

impl Default for NodeChildrenTree {
    fn default() -> Self {
        Self { nodes: Default::default() }
    }
}

#[derive(Debug)]
enum NodeChild {
    Node(Node),
    In,
    Out,
}

#[derive(Default, Debug)]
struct TemplateMap {
    map: HashMap<String, Vec<Node>>,
}

impl TemplateMap {
    fn add(&mut self, template: Template) {
        let mut template_roots = Vec::with_capacity(template.roots.len());
        
        for node in template.roots {
            let node = self.create_node(template.name.to_string(), node);
            template_roots.push(node);
        }

        self.map.insert(template.name.to_string(), template_roots);
    }
    fn create_node(&mut self, name: String, node: &TemplateNode) -> Node {
        match node {
            TemplateNode::Element {
                tag,
                attrs,
                children,
                ..
            } => {
                let mut children_tree = NodeChildrenTree::default();

                // TODO: Handle child nodes
                for node in *children {
                    let index = children_tree.placeholder();
                    let child = self.create_child(&mut children_tree, name.clone(), node);
                    children_tree.replace(index, child);
                }

                let element = Self::create_element(*tag);
                
                Node::ElementWithChildren {
                    element,
                    children: children_tree,
                }
            },
            TemplateNode::Text { text } => Self::create_text_node(*text),
            TemplateNode::Dynamic { .. } => Self::create_dynamic_node(),
            TemplateNode::DynamicText { .. } => Self::create_dynamic_text_node(),
        }
    }
    fn create_child(
        &mut self,
        children_tree: &mut NodeChildrenTree,
        name: String,
        node: &TemplateNode,
    ) -> NodeChild {
        let node = match node {
            TemplateNode::Element {
                tag,
                attrs,
                children,
                ..
            } => {
                for node in *children {
                    children_tree.add(NodeChild::In);
                    let index = children_tree.placeholder();
                    let child = self.create_child(children_tree, name.clone(), node);
                    children_tree.replace(index, child);
                    children_tree.add(NodeChild::Out);
                }

                Node::Element { element: Self::create_element(*tag) }
            },
            TemplateNode::Text { text } => Self::create_text_node(*text),
            TemplateNode::Dynamic { .. } => Self::create_dynamic_node(),
            TemplateNode::DynamicText { .. } => Self::create_dynamic_text_node(),
        };

        NodeChild::Node(node)
    }
    fn create_element(tag: &str) -> Element {
        // Wish I could avoid using string for tags
        match tag {
            "div" => Element::Div {
                bundle: NodeBundle {
                    ..default()
                },
            },
            "img" => Element::Image {
                bundle: ImageBundle {
                    ..default()
                },
            },
            "button" => Element::Button {
                bundle: ButtonBundle {
                    ..default()
                },
            },
            _ => panic!("Invalid tag, this shouldn't happen"),
        }
    }
    fn create_text_node(text: impl Into<String>) -> Node {
        Node::Text {
            bundle: TextBundle {
                text: Text::from_section(text, TextStyle::default()),
                ..default()
            }
        }
    }
    fn create_dynamic_node() -> Node {
        Node::PlaceHolder
    }
    fn create_dynamic_text_node() -> Node {
        Node::Text {
            bundle: TextBundle::default(),
        }
    }
}

#[derive(Default, Debug)]
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