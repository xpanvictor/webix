
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
}
