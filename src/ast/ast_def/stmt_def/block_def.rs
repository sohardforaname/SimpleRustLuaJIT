use crate::ast::ast_def::stmt_def::{Stat, Exp};

pub struct Block {
    pub last_line: usize,
    pub stats: Vec<Box<dyn Stat>>,
    pub is_contain_ret: bool,
    pub ret_exps: Option<Vec<Box<dyn Exp>>>,
}

impl Block {
    pub fn new(last_line: usize
           , stats: Vec<Box<dyn Stat>>
           , opt_ret_exps: Option<Vec<Box<dyn Exp>>>) -> Block {
        let is_contain_ret = opt_ret_exps.is_some();
        Block {
            last_line,
            stats,
            is_contain_ret,
            ret_exps: opt_ret_exps,
        }
    }
}