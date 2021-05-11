use crate::ast::lexer::token::Token;

struct ASTNode {
    pub token: Token,
    pub childs: Vec<ASTNode>,
}

impl ASTNode {
}

struct AST {
    pub root: ASTNode,
}

