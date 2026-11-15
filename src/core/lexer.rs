use super::{Error, ErrorKind};

pub(super) enum TokenKind {
    StringLiteral(String),
    CloseParen,
    EndOfInput,
}

pub(super) struct Token {
    pub(super) kind: TokenKind,
    pub(super) offset: usize,
}

pub(super) struct Lexer<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub(super) fn new(source: &'a str, position: usize) -> Self {
        Self { source, position }
    }

    pub(super) fn position(&self) -> usize {
        self.position
    }

    pub(super) fn next_token(&mut self) -> Result<Token, Error> {
        while self.source[self.position..].starts_with(char::is_whitespace) {
            self.advance();
        }
        let offset = self.position;
        let kind = match self.advance() {
            Some('"') => TokenKind::StringLiteral(self.read_string(offset)?),
            Some(')') => TokenKind::CloseParen,
            None => TokenKind::EndOfInput,
            Some(_) => return Err(Self::error("Unexpected character in expression", offset)),
        };
        Ok(Token { kind, offset })
    }

    fn advance(&mut self) -> Option<char> {
        let character = self.source[self.position..].chars().next()?;
        self.position += character.len_utf8();
        Some(character)
    }

    fn read_string(&mut self, start: usize) -> Result<String, Error> {
        let mut value = String::new();
        loop {
            let offset = self.position;
            match self.advance() {
                Some('"') => return Ok(value),
                Some('\\') => {
                    let escaped = match self.advance() {
                        Some('"') => '"',
                        Some('\\') => '\\',
                        Some('n') => '\n',
                        Some('r') => '\r',
                        Some('t') => '\t',
                        Some(_) => return Err(Self::error("Unknown string escape", offset)),
                        None => return Err(Self::error("Unterminated string", start)),
                    };
                    value.push(escaped);
                }
                Some(character) => value.push(character),
                None => return Err(Self::error("Unterminated string", start)),
            }
        }
    }

    fn error(message: &str, offset: usize) -> Error {
        Error::new(ErrorKind::Lex, message).with_metadata("offset", offset.to_string())
    }
}
