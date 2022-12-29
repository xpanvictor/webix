use std::collections::HashMap;

pub struct Node {
    // type of the node element
    pub(crate) node_type: NodeType,
    // the children of the node in the tree
    pub(crate) children: Vec<Node>
}

// type string or element
pub enum NodeType {
    Text(String),
    Element(ElementData)
}

pub struct ElementData {
    pub(crate) tag_name: String,
    pub(crate) attributes: AttrMap
}

pub type AttrMap = HashMap<String, String>;

// constructor function for text node
fn text(data: String) -> Node {
    Node {
        node_type: NodeType::Text(data),
        children: Vec::new()
    }
}

// constructor fn for element node
pub fn element_constructor(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        node_type: NodeType::Element(
            ElementData {
                tag_name: name,
                attributes: attrs
            }
        ),
        children: children
    }
}