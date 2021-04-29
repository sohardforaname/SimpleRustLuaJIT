pub mod astnode;

use super::lexer;
use crate::ast::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,

}