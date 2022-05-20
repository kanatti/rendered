use std::collections::HashMap;

use dom::Node;

mod dom;

fn main() {
    let text1 = Node::text("text1");
    let text2 = Node::text("text2");
    let comment = Node::comment("Just commenting");
    let div1 = Node::elem(
        "div",
        HashMap::from([("class", "data"), ("id", "main")]),
        vec![comment, text1, text2],
    );

    div1.pretty_print();
}
