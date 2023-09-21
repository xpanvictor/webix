
use crate::dom;

struct Parser {
    pos: usize,
    input: String
}

impl Parser {
    /// Methods for working with the file
    // Read next char without consuming
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // check if next characters start with a symbol
    fn starts_with(&self, symbol: &str) -> bool {
        self.next_char().eq(symbol)
    }

    // check if input has reached end of file
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // consume next char; use char_indices as rust stores strings as utf8 byte array
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, curr_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        curr_char
    }

    // consume sequence of chars as long as a condition is met
    fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
        let mut result = String::new();

        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }

        result
    }

    // consume whitespaces
    fn consume_whitespace(&mut self) {
        self.consume_while(|char| char == ' '); // initial solution
        // self.consume_while(char::is_whitespace()); // better solution
    }

    fn consume_comment(&mut self) {
        self.consume_while(|char| char == '\n');
    }

    // parse a tag or attribute name
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(
            |char| match char {
                'a'...'z' | 'A'...'Z' | '0'...'9' => true,
                _ => false
            }
        )
    }

    // Node parsers

    // Element starts with <
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_node(),
            // support for commenting using match guards :)
            '/' if self.next_char() == "/" => self.consume_comment(),
            _ => self.parse_text()
        }
    }

    // text parser
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|char| char != '<'))
    }

    // element parser
    fn parse_element(&mut self) -> dom::Node {
        // check for opening tag
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.consume_while(self.parse_tag_name());
        let attrs = self.parse_attributes();
        assert_eq!(self.consume_char(), '>'); // todo: check for self closing tags

        // contents
        let children = self.parse_nodes();

        // ensure closing tag; todo: ensure check is for non self closing tags only
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.consume_while(|char| char == '>'), tag_name);
        assert_eq!(self.consume_char(), '>');

        dom::elem(tag_name, attrs, children)
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        todo!()
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        todo!()
    }
}
