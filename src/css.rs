use crate::source::Source;

#[derive(Debug)]
pub struct StyleSheet {
    rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    classes: Vec<String>,
}

#[derive(Debug)]
struct Declaration {
    name: String,
    value: Value,
}

#[derive(Debug)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug)]
pub enum Unit {
    Px,
}

#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct Parser {
    src: Source,
}

impl Parser {
    pub fn new(source: String) -> Self {
        Self {
            src: Source::new(source),
        }
    }

    pub fn parse(&mut self) -> StyleSheet {
        let mut rules = Vec::new();

        while !self.src.eof() {
            rules.push(self.parse_rule());
        }

        StyleSheet { rules }
    }

    fn parse_rule(&mut self) -> Rule {
        let selectors = self.parse_selectors();
        assert!(self.src.consume_char() == '{');
        let declarations = self.parse_declarations();

        Rule {
            selectors,
            declarations,
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        while self.src.peek() != '{' {
            selectors.push(self.parse_selector());
        }

        selectors
    }

    fn parse_selector(&mut self) -> Selector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            classes: vec![],
        };

        while !matches!(self.src.peek(), ',' | ' ') {
            match self.src.peek() {
                '.' => {
                    self.src.consume_char();
                    let class_name = self.consume_identifier();
                    selector.classes.push(class_name);
                }
                '#' => {
                    self.src.consume_char();
                    selector.id = Some(self.consume_identifier());
                }
                _ => {
                    selector.tag_name = Some(self.consume_identifier());
                }
            }
        }

        if self.src.peek() == ',' {
            self.src.consume_char();
        }

        self.src.consume_whitespace();

        Selector::Simple(selector)
    }

    fn consume_identifier(&mut self) -> String {
        self.src.consume_while(Self::is_valid_identifier)
    }

    fn is_valid_identifier(c: char) -> bool {
        char::is_ascii_alphabetic(&c) || char::is_ascii_digit(&c) || c == '-'
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        self.src.consume_while(|c| c != '}');
        self.src.consume_char();
        self.src.consume_whitespace();
        vec![]
    }
}
