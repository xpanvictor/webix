use std::collections::HashMap;

struct Node {
    // type of the node element
    node_type: NodeType,
    // the children of the node in the tree
    children: Vec<Node>
}

// type string or element
enum NodeType {
    Text(String),
    Element(ElementData)
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap
}

type AttrMap = HashMap<String, String>;

// constructor function for text node
fn text(data: String) -> Node {
    Node {
        node_type: NodeType::Text(data),
        children: Vec::new()
    }
}

// constructor fn for element node
fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
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