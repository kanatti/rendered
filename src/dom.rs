use std::collections::HashMap;

pub struct Node<'a> {
    children: Vec<Node<'a>>,
    node_type: NodeType<'a>,
}

pub enum NodeType<'a> {
    Text(&'a str),
    Element(ElementData<'a>),
    Comment(&'a str),
}

pub struct ElementData<'a> {
    tag_name: &'a str,
    attributes: AttrMap<'a>,
}

pub type AttrMap<'a> = HashMap<&'a str, &'a str>;

impl<'a> Node<'a> {
    pub fn text(data: &'a str) -> Self {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    pub fn elem(name: &'a str, attrs: AttrMap<'a>, children: Vec<Node<'a>>) -> Self {
        Node {
            children,
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
        }
    }

    pub fn comment(data: &'a str) -> Self {
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
   leveled(format!("<{} {:?}>", elem_data.tag_name, elem_data.attributes), level);
   for child in children.iter() {
       print_node(child, level + 1);
   }
  leveled(format!("</{}>", elem_data.tag_name), level);
}

fn leveled(val: String, level: usize) {
    println!("{:>indent$}", val, indent=PRINT_TAB*level + val.len());
}
