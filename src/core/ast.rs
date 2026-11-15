use super::atom::Atom;

pub(super) struct Ast {
    pub(super) root: AstNode,
}

pub(super) enum AstNode {
    StringLiteral(String),
    AtomLiteral(Atom),
    NullLiteral,
    BooleanLiteral(bool),
    Concat(Vec<AstNode>),
}
