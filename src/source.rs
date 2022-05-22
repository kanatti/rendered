// Abstraction to read and seek through source
pub struct Source {
    pos: usize,
    source: String,
}

impl Source {
    pub fn new(source: String) -> Self {
        Self { pos: 0, source }
    }

    pub fn peek(&self) -> char {
        self.source[self.pos..].chars().next().unwrap()
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.source[self.pos..].starts_with(s)
    }

    pub fn eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    pub fn consume_char(&mut self) -> char {
        let mut iter = self.source[self.pos..].chars();
        let cur_char = iter.next().unwrap();
        self.pos += 1;
        cur_char
    }

    pub fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.peek()) {
            result.push(self.consume_char());
        }

        result
    }

    pub fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
}
