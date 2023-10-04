use crate::nodes::{TextNode, ImageNode, ButtonNode, SimpleNode};


#[derive(Debug, Clone)]
pub enum RootNode {
    ElementWithChildren {
        element: Element,
        children: NodeChildrenTree,
    },
    Element {
        element: Element,
    },
    Text {
        node: TextNode,
    },
    PlaceHolder,
}

#[derive(Debug, Clone)]
pub enum ChildNode {
    Element {
        element: Element,
    },
    Text {
        node: TextNode,
    },
    PlaceHolder,
}

#[derive(Debug, Clone)]
pub enum Element {
    Div {
        node: SimpleNode,
    },
    Image {
        node: ImageNode,
    },
    Button {
        node: ButtonNode,
    },
}

#[derive(Debug, Clone)]
pub struct NodeChildrenTree {
    pub nodes: Vec<NodeChild>,
}

impl NodeChildrenTree {
    pub fn placeholder(&mut self) -> usize {
        self.nodes.push(NodeChild::Node(ChildNode::PlaceHolder));
        self.nodes.len() - 1
    }
    pub fn replace(&mut self, index: usize, child: ChildNode) {
        self.nodes[index] = NodeChild::Node(child);
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

#[derive(Debug, Clone)]
pub enum NodeChild {
    Node(ChildNode),
    In,
    Out,
}