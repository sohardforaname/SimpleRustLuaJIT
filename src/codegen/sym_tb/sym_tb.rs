use std::collections::{HashMap, HashSet};
use crate::ast::lexer::token::Token;
use std::borrow::Borrow;

struct SymbolTable {
    scope_index: HashMap<Token, Vec<usize>>,
    token_stack: Vec<HashSet<Token>>,
}

impl SymbolTable {
    /*pub fn new() -> SymbolTable {
        SymbolTable {
            scope_index: HashMap::new(),
            token_stack: Vec::new(),
        }
    }
    pub fn query_token(&self, token: &Token) -> bool {
        self.scope_index.borrow().get(token).is_some()
    }

    pub fn insert_token(&mut self, token: &Token) {

    }

    pub fn push_scope() {

    }

    pub fn pop_scope() {

    }*/
}