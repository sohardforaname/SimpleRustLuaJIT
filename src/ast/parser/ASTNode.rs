use super::token;
use crate::ast::lexer::token::Token;

pub struct ASTNode {
    token: Token,
    childs: Vec<ASTNode>,
}

impl ASTNode {

}

pub struct AST {
    root: ASTNode,
}