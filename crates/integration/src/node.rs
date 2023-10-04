use bevy::prelude::{TextBundle, NodeBundle, ImageBundle, ButtonBundle};

#[derive(Debug)]
pub enum Node {
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
pub enum Element {
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
pub struct NodeChildrenTree {
    nodes: Vec<NodeChild>,
}

impl NodeChildrenTree {
    pub fn placeholder(&mut self) -> usize {
        self.nodes.push(NodeChild::Node(Node::PlaceHolder));
        self.nodes.len() - 1
    }
    pub fn replace(&mut self, index: usize, child: NodeChild) {
        self.nodes[index] = child;
    }
    pub fn add(&mut self, child: NodeChild) {
        self.nodes.push(child);
    }
}

impl Default for NodeChildrenTree {
    fn default() -> Self {
        Self { nodes: Default::default() }
    }
}

#[derive(Debug)]
pub enum NodeChild {
    Node(Node),
    In,
    Out,
}