use super::{
    Error, ErrorKind,
    ast::{Ast, AstNode},
    lexer::{Lexer, Token, TokenKind},
};

pub(crate) struct Parser<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
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
        if matches!(token.kind, TokenKind::EndOfInput) {
            return Err(Self::error("Unterminated interpolation", start));
        }
        let node = Self::parse_node(token)?;
        let token = lexer.next_token()?;
        match token.kind {
            TokenKind::CloseParen => {
                self.position = lexer.position();
                Ok(node)
            }
            TokenKind::EndOfInput => Err(Self::error("Unterminated interpolation", start)),
            _ => Err(Self::error("Expected closing parenthesis", token.offset)),
        }
    }

    pub(crate) fn parse_identifier(self) -> Result<String, Error> {
        let mut lexer = Lexer::new(self.source, 0);
        let token = lexer.next_token()?;
        match token.kind {
            TokenKind::Identifier(name) if name == self.source => {
                Self::expect_end(&mut lexer)?;
                Ok(name)
            }
            _ => Err(Self::error("Expected an identifier", token.offset)),
        }
    }

    pub(super) fn parse_literal(self) -> Result<Ast, Error> {
        let mut lexer = Lexer::new(self.source, 0);
        let token = lexer.next_token()?;
        let offset = token.offset;
        let root = Self::parse_node(token)?;
        if matches!(root, AstNode::Variable(_)) {
            return Err(Self::error("Expected a literal", offset));
        }
        Self::expect_end(&mut lexer)?;
        Ok(Ast { root })
    }

    fn expect_end(lexer: &mut Lexer<'_>) -> Result<(), Error> {
        let token = lexer.next_token()?;
        if matches!(token.kind, TokenKind::EndOfInput) {
            Ok(())
        } else {
            Err(Self::error("Unexpected trailing content", token.offset))
        }
    }

    fn parse_node(token: Token) -> Result<AstNode, Error> {
        match token.kind {
            TokenKind::StringLiteral(value) => Ok(AstNode::StringLiteral(value)),
            TokenKind::AtomLiteral(atom) => Ok(AstNode::AtomLiteral(atom)),
            TokenKind::NullLiteral => Ok(AstNode::NullLiteral),
            TokenKind::BooleanLiteral(value) => Ok(AstNode::BooleanLiteral(value)),
            TokenKind::Identifier(name) => Ok(AstNode::Variable(name)),
            _ => Err(Self::error("Expected an expression", token.offset)),
        }
    }

    fn error(message: &str, offset: usize) -> Error {
        Error::new(ErrorKind::Parse, message).with_metadata("offset", offset.to_string())
    }
}
