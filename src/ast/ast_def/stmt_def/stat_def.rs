use crate::ast::ast_def::stmt_def::Exp;
use crate::ast::ast_def::stmt_def::exp_def::{FuncDefExp, FuncCallExp};
use crate::ast::ast_def::stmt_def::block_def::Block;

pub struct EmptyStat {}

pub struct BreakStat {
    pub line: usize,
}

pub struct LabelStat {
    pub name: String,
}

pub struct GotoStat {
    pub target: String,
}

pub struct DoStat {
    pub block: Block,
}

pub struct WhileStat {
    pub exp: Box<dyn Exp>,
    pub block: Block,
}

pub struct RepeatStat {
    pub block: Block,
    pub exp: Box<dyn Exp>,
}

pub struct IfStat {
    pub exps: Vec<Box<dyn Exp>>,
    pub blocks: Vec<Block>,
}

type FuncCallStat = FuncCallExp;

pub struct StepForStat {
    pub beg_line: usize,
    pub block_beg_line: usize,
    pub var_name: String,
    pub init_exp: Box<dyn Exp>,
    pub lim_exp: Box<dyn Exp>,
    pub step_exp: Box<dyn Exp>,
    pub block: Block,
}

pub struct RangeForStat {
    pub block_beg_line: usize,
    pub name_list: Vec<String>,
    pub exp_list: Vec<Box<dyn Exp>>,
    pub block: Block,
}

pub struct LocalVarDefStat {
    pub last_line: usize,
    pub name_list: Vec<String>,
    pub exp_list: Vec<Box<dyn Exp>>,
}

pub struct AssignStat {
    pub last_line: usize,
    pub var_list: Vec<Box<dyn Exp>>,
    pub exp_list: Vec<Box<dyn Exp>>,
}

pub struct LocalFuncDefStat {
    pub name: String,
    pub exp: FuncDefExp,
}
