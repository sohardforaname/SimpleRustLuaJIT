pub mod exp_def;
pub mod stat_def;
pub mod block_def;

pub trait Exp {}

pub enum StatType {
    EmptyStatTag
}

pub trait Stat {
    fn get_type(&self) -> StatType;
}