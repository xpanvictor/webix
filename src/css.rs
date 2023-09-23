
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
