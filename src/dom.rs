use std::collections::HashMap;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

impl Node {
    fn text(data: String) -> Self {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Self {
        Node {
            children,
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
        }
    }

    fn comment(data: String) -> Self {
        Node {
            children: Vec::new(),
            node_type: NodeType::Comment(data),
        }
    }
}
