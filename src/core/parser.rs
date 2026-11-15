use super::{
    Error, ErrorKind,
    ast::{Ast, AstNode},
    lexer::{Lexer, TokenKind},
};

pub(super) struct Parser<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    pub(super) fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    pub(super) fn parse(mut self) -> Result<Ast, Error> {
        let mut nodes = Vec::new();
        while let Some(relative) = self.source[self.position..].find("\\(") {
            let start = self.position + relative;
            if start > self.position {
                nodes.push(AstNode::StringLiteral(
                    self.source[self.position..start].into(),
                ));
            }
            self.position = start + 2;
            nodes.push(self.parse_expression(start)?);
        }
        if self.position < self.source.len() {
            nodes.push(AstNode::StringLiteral(self.source[self.position..].into()));
        }
        let root = match nodes.len() {
            0 => AstNode::StringLiteral(String::new()),
            1 => nodes.pop().expect("one node is present"),
            _ => AstNode::Concat(nodes),
        };
        Ok(Ast { root })
    }

    fn parse_expression(&mut self, start: usize) -> Result<AstNode, Error> {
        let mut lexer = Lexer::new(self.source, self.position);
        let token = lexer.next_token()?;
        let value = match token.kind {
            TokenKind::StringLiteral(value) => value,
            TokenKind::EndOfInput => return Err(Self::error("Unterminated interpolation", start)),
            _ => return Err(Self::error("Expected a string literal", token.offset)),
        };
        let token = lexer.next_token()?;
        match token.kind {
            TokenKind::CloseParen => {
                self.position = lexer.position();
                Ok(AstNode::StringLiteral(value))
            }
            TokenKind::EndOfInput => Err(Self::error("Unterminated interpolation", start)),
            _ => Err(Self::error("Expected closing parenthesis", token.offset)),
        }
    }

    fn error(message: &str, offset: usize) -> Error {
        Error::new(ErrorKind::Parse, message).with_metadata("offset", offset.to_string())
    }
}
