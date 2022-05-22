use crate::css::Parser as CssParser;
use crate::html::Parser as HtmlParser;
use indoc::indoc;

mod css;
mod dom;
mod html;
mod source;

fn main() {
    let source = indoc! {
        "<div>
            Hello
            <span class='test' id='id1'>Ok</span>
        </div>"
    };
    let css = indoc! {
        "h1, h2, h3 { margin: auto; color: #cc0000; }
         div.note { margin-bottom: 20px; padding: 10px; }
         #answer { display: none; }"
    };
    let mut parser = HtmlParser::new(source.to_string());
    let dom = parser.parse();
    println!("{:#?}", dom);
    dom.pretty_print();

    println!("----css----");
    let mut css_parser = CssParser::new(css.to_string());
    let styles = css_parser.parse();
    println!("{:#?}", styles);
}
