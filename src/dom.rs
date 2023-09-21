use std::collections::HashMap;
pub enum NodeType {
    Text(String),
    Element(ElementData)
}

struct ElementData {
    tagName: String,
    attributes: AttrMap
}

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    // children under this node
    children: Vec<Node>,
    // the type of node this is
    nodeType: NodeType
}

pub fn text(data: String) -> Node {
    Node { children: Vec::new(), nodeType: NodeType::Text(data) }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        nodeType: NodeType::Element(
            ElementData {
                attributes: attrs,
                tagName: name
            }
        )
    }
}

impl Node {
    fn pretty_print(&self) {
        println!("{:#?}", self)
    }
}
