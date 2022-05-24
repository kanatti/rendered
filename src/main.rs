use crate::css::Parser as CssParser;
use crate::html::Parser as HtmlParser;
use crate::style::style_tree;
use indoc::indoc;

mod css;
mod dom;
mod html;
mod source;
mod style;
mod layout;

fn main() {
    let source = indoc! {
        "<div>
            Hello
            <div class='note'>Note this</div>
            <span class='test' id='id1'>Ok</span>
        </div>"
    };
    let css = indoc! {
        "h1, h2, h3.main { margin: auto; color: #cc0000; }
         div.note { margin-bottom: 20px; padding: 10px; }
         #ans, #answer, span { display: none; }"
    };
    let mut parser = HtmlParser::new(source.to_string());
    let dom = parser.parse();
    println!("{:#?}", dom);
    dom.pretty_print();

    println!("----css----");
    let mut css_parser = CssParser::new(css.to_string());
    let styles = css_parser.parse();
    println!("{:#?}", styles);

    println!("----style tree----");
    let style_tree = style_tree(&dom, &styles);
    println!("{:#?}", style_tree);
}
