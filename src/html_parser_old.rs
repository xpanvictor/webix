use std::collections::HashMap;
use crate::dom_old;
use crate::dom_old::{AttrMap, element_constructor};

struct Parser {
    // counter position
    pos: usize,
    // the text input
    input: String
}

impl Parser {
    // a method to read current char without using
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }
    // a method to compare next char with a string
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }
    // a method to check if we've reached end of file
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // a method to actually consume next character
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, curr_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return curr_char;
    }
    // a method to consume a group of chars under a condition
    fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
        let mut group_of_words = String::new();
        while !self.eof() && test(self.next_char()) {
            group_of_words.push(self.consume_char());
        }
        return group_of_words;
    }

    // now use the consume_while to discard whitespace characters
    fn consume_whitespace(&mut self) {
        // note, im using char and not CharExt as Im not sure of the namespace
        self.consume_while(char::is_whitespace);
    }

    // a method to parse tag name
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(
            |c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => true,
                _ => false
            }
        )
    }

    // a method to actually parse a tag
    fn parse_tag(&mut self) -> dom_old::Node {
        assert_eq!(self.consume_char(), '<');
        // use a loop to make children
        let children = Vec::new();
        let tag_name = self.parse_tag_name();
        self.consume_char();
        // fetch all attributes
        let attrs = self.parse_attributes();
        // ensure end of tag
        assert_eq!(self.consume_char(), '>');
        // now let's take all the children
        return element_constructor(tag_name, attrs, children)
    }

    // a method to parse attributes
    fn parse_attr(&mut self) -> (String, String) {
        let key = self.consume_while(
            |c| match c {
                '=' => true,
                _ => false
            }
        );
        let value = self.consume_while(
            |c| match c {
                ' ' => true,
                _ => false
            }
        );
        return (key, value)
    }

    // a method to parse all the attributes
    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = HashMap::new();
        loop {
            if (self.consume_char() == '>') {
                break;
            }
            let (key, value) = self.parse_attr();
            attributes.insert(key, value);
        }
        return attributes
    }
}