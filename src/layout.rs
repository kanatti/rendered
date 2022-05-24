use crate::style::{Display, StyledNode};

#[derive(Default)]
struct Dimensions {
    content: Rect,
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

#[derive(Default)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Default)]
struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

enum BoxType<'a> {
    Block(&'a StyledNode<'a>),
    Inline(&'a StyledNode<'a>),
    AnonymousBlock,
}

fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
    let mut root = LayoutBox::new(match style_node.display() {
        Display::Block => BoxType::Block(style_node),
        Display::Inline => BoxType::Inline(style_node),
        Display::None => panic!("Root node has display none"),
    });

    for child in style_node.children.iter() {
        match child.display() {
            Display::Block => root.children.push(build_layout_tree(child)),
            Display::Inline => match inline_boxgen(&root) {
                BoxGen::Root => root.children.push(build_layout_tree(child)),
                BoxGen::LastChild => root
                    .children
                    .last_mut()
                    .unwrap()
                    .children
                    .push(build_layout_tree(child)),
                BoxGen::NewBox => {
                    let mut anonymous_block = LayoutBox::new(BoxType::AnonymousBlock);
                    anonymous_block.children.push(build_layout_tree(child));
                    root.children.push(anonymous_block);
                }
            },
            Display::None => {}
        }
    }

    root
}

enum BoxGen {
    Root,
    LastChild,
    NewBox,
}

fn inline_boxgen<'a>(layout_box: &'a LayoutBox<'a>) -> BoxGen {
    match layout_box.box_type {
        BoxType::Inline(_) | BoxType::AnonymousBlock => BoxGen::Root,
        BoxType::Block(_) => match layout_box.children.last() {
            Some(&LayoutBox {
                box_type: BoxType::AnonymousBlock,
                ..
            }) => BoxGen::LastChild,
            _ => BoxGen::NewBox,
        },
    }
}

impl<'a> LayoutBox<'a> {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }
}
