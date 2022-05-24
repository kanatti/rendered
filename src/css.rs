use crate::source::Source;

#[derive(Debug)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug, Clone)]
pub enum Unit {
    Px,
}

#[derive(Debug, Clone)]
pub enum Color {
    RGBA(u8, u8, u8, u8),
    HEX(String),
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Self::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.classes.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
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

        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
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
        let mut declarations = Vec::new();

        while self.src.peek() != '}' {
            declarations.push(self.parse_declaration());
        }

        self.src.consume_char();
        self.src.consume_whitespace();
        declarations
    }

    fn parse_declaration(&mut self) -> Declaration {
        self.src.consume_whitespace();
        let name = self.consume_identifier();
        assert!(self.src.consume_char() == ':');
        self.src.consume_whitespace();

        let raw_value = self.src.consume_while(|c| c != ';');

        let value;

        if raw_value.ends_with("px") {
            value = Value::Length(
                raw_value[..raw_value.len() - 2].parse::<f32>().unwrap(),
                Unit::Px,
            );
        } else if raw_value.starts_with('#') {
            value = Value::ColorValue(Color::HEX(raw_value[1..].to_string()));
        } else {
            value = Value::Keyword(raw_value);
        }

        assert!(self.src.consume_char() == ';');
        self.src.consume_whitespace();

        Declaration { name, value }
    }
}
