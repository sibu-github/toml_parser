#[derive(Debug, PartialEq, Clone)]
enum Tokens {
    WhiteSpace,
    NewLine,
    Period,
    Comma,
    Pound,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBrace,
    RightCurlyBrace,
    Equal,
    DoubleQuote,
    SingleQuote,
    PlusSign,
    MinusSign,
    Number(u32),
    Character(char),
}
