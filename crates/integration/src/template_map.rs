use bevy::{utils::HashMap, text::{Text, TextStyle}};
use dioxus::prelude::{TemplateNode, Template};

use crate::{node::{RootNode, NodeChildrenTree, NodeChild, Element, ChildNode}, nodes::{SimpleNode, ImageNode, ButtonNode, TextNode}};

#[derive(Default, Debug)]
pub struct TemplateMap {
    pub map: HashMap<String, Vec<RootNode>>,
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
    fn create_node(&mut self, name: String, node: &TemplateNode) -> RootNode {
        match node {
            TemplateNode::Element {
                tag,
                children,
                ..
            } => {
                let mut children_tree = NodeChildrenTree::default();

                for node in *children {
                    let index = children_tree.placeholder();
                    let child = self.create_child(&mut children_tree, name.clone(), node);
                    children_tree.replace(index, child);
                }

                let element = Self::create_element(*tag);
                
                if children.is_empty() {
                    RootNode::Element { element }
                } else {
                    RootNode::ElementWithChildren {
                        element,
                        children: children_tree,
                    }
                }
            },
            TemplateNode::Text { text } => RootNode::Text { node: Self::create_text(*text) },
            TemplateNode::DynamicText { .. } => RootNode::Text { node: Self::create_dynamic_text() },
            TemplateNode::Dynamic { .. } => RootNode::PlaceHolder,
        }
    }
    fn create_child(
        &mut self,
        children_tree: &mut NodeChildrenTree,
        name: String,
        node: &TemplateNode,
    ) -> ChildNode {
        match node {
            TemplateNode::Element {
                tag,
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

                ChildNode::Element { element: Self::create_element(*tag) }
            },
            TemplateNode::Text { text } => ChildNode::Text { node: Self::create_text(*text) },
            TemplateNode::DynamicText { .. } => ChildNode::Text { node: Self::create_dynamic_text() },
            TemplateNode::Dynamic { .. } => ChildNode::PlaceHolder,
        }
    }
    fn create_element(tag: &str) -> Element {
        // Wish I could avoid using string for tags
        match tag {
            "div" => Element::Div {
                node: SimpleNode { }
            },
            "img" => Element::Image {
                node: ImageNode { },
            },
            "button" => Element::Button {
                node: ButtonNode { },
            },
            _ => panic!("Invalid tag, this shouldn't happen"),
        }
    }
    fn create_text(text: impl Into<String>) -> TextNode {
        TextNode {
            text: Text::from_section(text, TextStyle::default()),
        }
    }
    fn create_dynamic_text() -> TextNode {
        TextNode {
            text: Text::from_section("", TextStyle::default()),
        }
    }
}