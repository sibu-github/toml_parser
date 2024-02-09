#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
    Slash,
    Number(u32),
    Character(char),
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Period,
            ',' => Self::Comma,
            '#' => Self::Pound,
            '[' => Self::LeftSquareBracket,
            ']' => Self::RightSquareBracket,
            '{' => Self::LeftCurlyBrace,
            '}' => Self::RightCurlyBrace,
            '=' => Self::Equal,
            '"' => Self::DoubleQuote,
            '\'' => Self::SingleQuote,
            '+' => Self::PlusSign,
            '-' => Self::MinusSign,
            '/' => Self::Slash,
            ch if ch == 0xA as char => Self::NewLine,
            ch if ch.is_whitespace() => Self::WhiteSpace,
            ch if ch.is_ascii_digit() => Self::Number(ch.to_digit(10).unwrap()),
            ch => Self::Character(ch),
        }
    }
}

#[derive(Debug)]
pub struct InputTokens {
    line: usize,
    col: usize,
    pos: usize,
    tokens: Vec<Token>,
}

impl FromIterator<char> for InputTokens {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let tokens = iter.into_iter().map(Into::into).collect();
        Self {
            line: 0,
            col: 0,
            pos: 0,
            tokens,
        }
    }
}

impl Iterator for InputTokens {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.get(self.pos).cloned().map(|t| {
            if t == Token::NewLine {
                self.line += 1;
                self.col = 0;
            }
            self.pos += 1;
            t
        })
    }
}

impl InputTokens {
    pub fn has_next(&self) -> bool {
        self.tokens.len() > self.pos
    }

    pub fn next_token(&self) -> Option<Token> {
        self.tokens.get(self.pos + 1).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_from_char() {
        assert_eq!(Token::from(' '), Token::WhiteSpace);
        assert_eq!(Token::from('\t'), Token::WhiteSpace);
        assert_eq!(Token::from('\n'), Token::NewLine);
        assert_eq!(Token::from(0xA as char), Token::NewLine);
        assert_eq!(Token::from('.'), Token::Period);
        assert_eq!(Token::from(','), Token::Comma);
        assert_eq!(Token::from('#'), Token::Pound);
        assert_eq!(Token::from('['), Token::LeftSquareBracket);
        assert_eq!(Token::from(']'), Token::RightSquareBracket);
        assert_eq!(Token::from('{'), Token::LeftCurlyBrace);
        assert_eq!(Token::from('}'), Token::RightCurlyBrace);
        assert_eq!(Token::from('='), Token::Equal);
        assert_eq!(Token::from('"'), Token::DoubleQuote);
        assert_eq!(Token::from('\''), Token::SingleQuote);
        assert_eq!(Token::from('+'), Token::PlusSign);
        assert_eq!(Token::from('-'), Token::MinusSign);
        assert_eq!(Token::from('/'), Token::Slash);
        (0..10).for_each(|d| {
            let c = char::from_digit(d, 10).unwrap();
            assert_eq!(Token::from(c), Token::Number(d));
        });
        ('a'..='z').for_each(|ch| {
            assert_eq!(Token::from(ch), Token::Character(ch));
        });
        ('A'..='Z').for_each(|ch| {
            assert_eq!(Token::from(ch), Token::Character(ch));
        });
        let tokens = vec![
            '!', '@', '$', '%', '^', '&', '*', '(', ')', '~', '<', '>', ':', ';', '\\',
        ];
        tokens.into_iter().for_each(|ch| {
            assert_eq!(Token::from(ch), Token::Character(ch));
        });
    }
}
