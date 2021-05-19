use crate::ast::ast_def::stmt_def::Exp;
use crate::ast::ast_def::stmt_def::block_def::Block;

pub struct NilExp {
    pub line: usize,
}

pub struct TrueExp {
    pub line: usize,
}

pub struct FalseExp {
    pub line: usize,
}

pub struct IntegerExp {
    pub line: usize,
    pub num: i64,
}

pub struct FloatExp {
    pub line: usize,
    pub num: f64,
}

pub struct StringExp {
    pub line: usize,
    pub str: String,
}

pub struct IDExp {
    pub line: usize,
    pub name: String,
}

pub struct UnopExp {
    pub line: usize,
    pub op: usize,
    pub exp: Box<dyn Exp>,
}

pub struct BinopExp {
    pub line: usize,
    pub op: usize,
    pub left_exp: Box<dyn Exp>,
    pub right_exp: Box<dyn Exp>,
}

pub struct ConExp {
    pub line: usize,
    pub exps: Vec<Box<dyn Exp>>,
}

pub struct TableConsExp {
    pub line: usize,
    pub last_line: usize,
    pub key_exps: Vec<Box<dyn Exp>>,
    pub val_exps: Vec<Box<dyn Exp>>,
}

pub struct FuncDefExp {
    pub line: usize,
    pub last_line: usize,
    pub par_list: Vec<String>,
    pub is_vararg: bool,
    pub block: Block,
}

pub struct ParensExp {
    pub in_exp: Box<Box<dyn Exp>>
}

pub struct TableAccessExp {
    pub last_line: usize,
    pub prefix: Box<dyn Exp>,
    pub key: Box<dyn Exp>,
}

pub struct FuncCallExp {
    pub line: usize,
    pub last_line: usize,
    pub prefix: Box<dyn Exp>,
    pub name_exp: StringExp,
    pub args: Vec<Box<dyn Exp>>,
}