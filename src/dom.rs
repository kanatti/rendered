use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

impl ElementData {
    pub fn tag_name(&self) -> &String {
        &self.tag_name
    }

    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classes) => classes.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

pub type AttrMap = HashMap<String, String>;

impl Node {
    pub fn text(data: String) -> Self {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Self {
        Node {
            children,
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
        }
    }

    pub fn comment(data: String) -> Self {
        Node {
            children: Vec::new(),
            node_type: NodeType::Comment(data),
        }
    }

    pub fn pretty_print(&self) {
        print_node(self, 0);
    }
}

static PRINT_TAB: usize = 2;

fn print_node(node: &Node, level: usize) {
    match &node.node_type {
        NodeType::Text(text) => leveled(format!("{}", text), level),
        NodeType::Comment(comment) => leveled(format!("{}", comment), level),
        NodeType::Element(elem) => {
            print_elem(elem, &node.children, level);
        }
    }
}

fn print_elem(elem_data: &ElementData, children: &Vec<Node>, level: usize) {
    leveled(
        format!("<{} {:?}>", elem_data.tag_name, elem_data.attributes),
        level,
    );
    for child in children.iter() {
        print_node(child, level + 1);
    }
    leveled(format!("</{}>", elem_data.tag_name), level);
}

fn leveled(val: String, level: usize) {
    println!("{:>indent$}", val, indent = PRINT_TAB * level + val.len());
}
