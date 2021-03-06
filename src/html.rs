use std::collections::HashMap;

use crate::{dom, source::Source};

pub struct Parser {
    input: Source,
}

impl Parser {
    pub fn new(source: String) -> Self {
        Self {
            input: Source::new(source),
        }
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.peek() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_element(&mut self) -> dom::Node {
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        let children = self.parse_nodes();

        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        dom::Node::elem(tag_name, attrs, children)
    }

    fn parse_text(&mut self) -> dom::Node {
        let text = self.consume_while(|c| c != '<');
        dom::Node::text(text)
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.peek() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    fn parse_attr(&mut self) -> (String, String) {
        let key = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        (key, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        value
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    pub fn parse(&mut self) -> dom::Node {
        let mut nodes = self.parse_nodes();

        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            dom::Node::elem("html".to_string(), HashMap::new(), nodes)
        }
    }

    // DELEGATIONS

    fn peek(&self) -> char {
        self.input.peek()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input.starts_with(s)
    }

    fn eof(&self) -> bool {
        self.input.eof()
    }

    fn consume_char(&mut self) -> char {
        self.input.consume_char()
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        self.input.consume_while(test)
    }

    fn consume_whitespace(&mut self) {
        self.input.consume_whitespace()
    }
}
