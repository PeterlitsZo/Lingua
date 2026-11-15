use super::atom::Atom;

pub(super) struct Ast {
    pub(super) root: AstNode,
}

pub(super) enum AstNode {
    StringLiteral(String),
    AtomLiteral(Atom),
    NullLiteral,
    BooleanLiteral(bool),
    Variable(String),
    Include(Box<AstNode>),
    Concat(Vec<AstNode>),
    Match {
        value: Box<AstNode>,
        arms: Vec<MatchArm>,
    },
}

pub(super) struct MatchArm {
    pub(super) pattern: MatchPattern,
    pub(super) value: AstNode,
}

pub(super) enum MatchPattern {
    String(String),
    Atom(Atom),
    Boolean(bool),
    Null,
    Wildcard,
}
