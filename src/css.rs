
struct Stylesheet {
    rules: Vec<Rule>
}

struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>
}

enum Selector {
    Simple(SimpleSelector)
}

struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>
}

// Declaration is a name value pair
struct Declaration {
    name: String,
    value: DeclarationValue
}

enum DeclarationValue {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color)
}

enum Unit {
    Px
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}


struct Parser {
    pos: usize,
    input: String
}

impl Parser {
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Parse rules
    fn parse_rules(&mut self) -> Vec<Rule> {
        todo!()
    }

    fn parse_rule(&mut self) -> Rule {
        todo!()
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        todo!()
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        todo!()
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        todo!()
    }

    fn parse_declaration(&mut self) -> Declaration {
        todo!()
    }

    // Parsing values
    fn parse_value(&mut self) -> DeclarationValue {
        todo!()
    }

    fn parse_length(&mut self) -> DeclarationValue {
        todo!()
    }

    fn parse_float(&mut self) -> f32 {
        todo!()
    }

    fn parse_unit(&mut self) -> Unit {
        todo!()
    }

    fn parse_color(&mut self) -> Color {
        todo!()
    }

    /// parse two hexadecimal pair
    fn parse_hex_pair(&mut self) -> u8 {
        todo!()
    }

    fn parse_identifier(&mut self) -> String {
        todo!()
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    // todo: extract into parent parser utilities
    fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }

        result
    }

    fn consume_char(&mut self) -> char {
        todo!()
    }
}
