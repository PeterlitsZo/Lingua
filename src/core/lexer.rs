use super::{Error, ErrorKind, atom::Atom};

pub(super) enum TokenKind {
    StringLiteral(String),
    AtomLiteral(Atom),
    Identifier(String),
    NullLiteral,
    BooleanLiteral(bool),
    Match,
    OpenParen,
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
    lookahead: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub(super) fn new(source: &'a str, position: usize) -> Self {
        Self {
            source,
            position,
            lookahead: None,
        }
    }

    pub(super) fn position(&self) -> usize {
        self.position
    }

    pub(super) fn peek_token(&mut self) -> Result<&Token, Error> {
        if self.lookahead.is_none() {
            self.lookahead = Some(self.next_token()?);
        }
        Ok(self.lookahead.as_ref().expect("lookahead is present"))
    }

    pub(super) fn next_token(&mut self) -> Result<Token, Error> {
        if let Some(token) = self.lookahead.take() {
            return Ok(token);
        }
        while self.source[self.position..].starts_with(char::is_whitespace) {
            self.advance();
        }
        let offset = self.position;
        let kind = match self.advance() {
            Some('"') => {
                let value = if self.source[self.position..].starts_with("\"\"") {
                    self.position += 2;
                    self.read_multiline_string(offset)?
                } else {
                    self.read_string(offset)?
                };
                TokenKind::StringLiteral(value)
            }
            Some(':') => TokenKind::AtomLiteral(self.read_atom(offset)?),
            Some(character) if character.is_ascii_alphabetic() || character == '_' => {
                self.read_identifier(offset)
            }
            Some('(') => TokenKind::OpenParen,
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

    fn read_identifier(&mut self, start: usize) -> TokenKind {
        while let Some(character) = self.source[self.position..].chars().next() {
            if !character.is_ascii_alphanumeric() && character != '_' {
                break;
            }
            self.advance();
        }
        match &self.source[start..self.position] {
            "match" => TokenKind::Match,
            "null" => TokenKind::NullLiteral,
            "true" => TokenKind::BooleanLiteral(true),
            "false" => TokenKind::BooleanLiteral(false),
            name => TokenKind::Identifier(name.into()),
        }
    }

    fn read_atom(&mut self, offset: usize) -> Result<Atom, Error> {
        let start = self.position;
        match self.source[start..].chars().next() {
            Some(character) if character.is_ascii_alphabetic() || character == '_' => {
                self.advance();
            }
            _ => return Err(Self::error("Expected an atom name", offset)),
        }
        while let Some(character) = self.source[self.position..].chars().next() {
            if !character.is_ascii_alphanumeric() && character != '_' {
                break;
            }
            self.advance();
        }
        Atom::intern(&self.source[start..self.position])
            .map_err(|message| Self::error(message, offset))
    }

    fn read_multiline_string(&mut self, start: usize) -> Result<String, Error> {
        let content_start = self.position;
        let mut value = String::new();
        let mut has_line = false;
        while self.position < self.source.len() {
            let line_start = self.position;
            let remaining = &self.source[line_start..];
            let newline = remaining.find('\n');
            let line = match newline {
                Some(length) => {
                    self.position += length + 1;
                    let line = &remaining[..length];
                    line.strip_suffix('\r').unwrap_or(line)
                }
                None => {
                    self.position = self.source.len();
                    remaining
                }
            };
            let trimmed = line.trim_start_matches(char::is_whitespace);
            let marker = line_start + line.len() - trimmed.len();
            if trimmed.starts_with("\"\"\"") {
                self.position = marker + 3;
                return Ok(value);
            }
            if trimmed.is_empty() && (line_start == content_start || newline.is_none()) {
                continue;
            }
            let content = trimmed.strip_prefix('|').ok_or_else(|| {
                Self::error(
                    "Expected '|' at the start of a multiline string line",
                    marker,
                )
            })?;
            let separator = content
                .chars()
                .next()
                .filter(|character| character.is_whitespace())
                .ok_or_else(|| Self::error("Expected whitespace after '|'", marker + 1))?;
            if has_line {
                value.push('\n');
            }
            value.push_str(&content[separator.len_utf8()..]);
            has_line = true;
        }
        Err(Self::error("Unterminated multiline string", start))
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
