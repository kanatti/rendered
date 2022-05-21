use html::Parser;

mod dom;
mod html;

fn main() {
    let source = "<div> Hello <span class='test' id='id1'>Ok</span></div>";
    let mut parser =  Parser::new(source.to_string());
    let dom = parser.parse();
    dom.pretty_print();
}
