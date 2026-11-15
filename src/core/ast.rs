pub(super) struct Ast {
    pub(super) root: AstNode,
}

pub(super) enum AstNode {
    StringLiteral(String),
}

impl Ast {
    pub(super) fn parse(source: String) -> Self {
        Self {
            root: AstNode::StringLiteral(source),
        }
    }
}
