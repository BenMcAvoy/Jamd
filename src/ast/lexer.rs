#![cfg_attr(debug_assertions, allow(dead_code))]

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Bad,
    Mod,
    Whitespace,
    Let,
    Identifier(String),
    Equals,
    Eof,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Number(_) => "Number",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Asterisk => "*",
            Self::Slash => "/",
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::Bad => "Bad",
            Self::Mod => "%",
            Self::Whitespace => "Whitespace",
            Self::Let => "Let",
            Self::Identifier(identifier) => identifier,
            Self::Equals => "=",
            Self::Eof => "EOF",
        };

        write!(f, "{}", string.to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TextSpan {
    pub literal: String,
    pub start: usize,
    pub end: usize,
}

impl TextSpan {
    pub const fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            literal,
            start,
            end,
        }
    }

    pub const fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub const fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos == self.input.len() {
            self.current_pos += 1;

            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(0, 0, '\0'.to_string()),
            ));
        }

        let char = self.current_char();

        char.map(|char| {
            let start = self.current_pos;

            let kind = match char {
                _ if char.is_ascii_digit() => {
                    let number = self.consume_number();
                    TokenKind::Number(number)
                }

                _ if char.is_whitespace() => {
                    self.consume();
                    TokenKind::Whitespace
                }

                _ if char.is_alphabetic() => {
                    let identifier = self.consume_identifier();

                    match identifier.as_str() {
                        "let" => TokenKind::Let,
                        _ => TokenKind::Identifier(identifier),
                    }
                }

                _ => self.consume_punctuation(),
            };

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);

            Token::new(kind, span)
        })
    }
}

impl<'a> Lexer<'a> {
    pub const fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }

        let char = self.current_char();
        self.current_pos += 1;
        char
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.consume();
                number = number * 10 + i64::from(c.to_digit(10).expect("Bad digit"));
            } else {
                break;
            }
        }

        number
    }

    fn consume_punctuation(&mut self) -> TokenKind {
        let char = self.consume().expect("Failed to get next char.");

        match char {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '%' => TokenKind::Mod,
            '=' => TokenKind::Equals,
            _ => TokenKind::Bad,
        }
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(c) = self.current_char() {
            if c.is_alphabetic() {
                self.consume();
                identifier.push(c);
            } else {
                break;
            }
        }

        identifier
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }
}
