use bevy::{utils::HashMap, prelude::{TextBundle, ButtonBundle, ImageBundle, NodeBundle, default}, text::{Text, TextStyle}};
use dioxus::prelude::{TemplateNode, Template};

use crate::node::{Node, NodeChildrenTree, NodeChild, Element};

#[derive(Default, Debug)]
pub struct TemplateMap {
    map: HashMap<String, Vec<Node>>,
}

impl TemplateMap {
    pub fn add(&mut self, template: Template) {
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