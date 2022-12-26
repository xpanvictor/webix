
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
}