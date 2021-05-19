use crate::ast::lexer::Lexer;
use crate::ast::lexer::token::{Token, TokenType, KeyWord};
use std::fmt::{Display, Formatter};
use crate::ast::ast_def::stmt_def::block_def::Block;
use crate::ast::ast_def::stmt_def::{Stat, Exp, StatType};
use crate::ast::ast_def::stmt_def::stat_def::*;
use crate::ast::ast_def::stmt_def::exp_def::{TrueExp, IntegerExp, FuncDefExp};

macro_rules! tk_from_kw {
    ($KEYWORD: expr) => { TokenType::from($KEYWORD) }
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    fn new(source_code: &String) -> Parser {
        Parser { lexer: Lexer::new(source_code) }
    }
}

impl Parser {
    fn is_ret_or_block_end(token_type: &TokenType) -> bool {
        match token_type {
            TokenType::OptKeyWord(key_word) => {
                match key_word {
                    KeyWord::RET |
                    KeyWord::END |
                    KeyWord::ELI |
                    KeyWord::ELS |
                    KeyWord::UNT => true,
                    _ => false
                }
            }
            TokenType::EOF => true,
            _ => false
        }
    }

    fn expected_token(&mut self, expected_type: TokenType) {
        if !self.lexer.peek_token_type().eq(&expected_type) {
            panic!("expected: {}", expected_type);
        }
        self.lexer.next_token();
    }

    fn expected_id(&mut self) -> Token {
        match self.lexer.peek_token_type() {
            TokenType::ID(_) => self.lexer.next_token().unwrap(),
            _ => panic!("expected id")
        }
    }
}

impl Parser {
    fn parse_stat(&mut self) -> Box<dyn Stat> {
        match self.lexer.peek_token_type() {
            TokenType::OptKeyWord(key_word) => {
                match key_word {
                    KeyWord::SEM => self.parse_empty_stat(),
                    KeyWord::BRK => self.parse_break_stat(),
                    KeyWord::PATH => self.parse_label_stat(),
                    KeyWord::GOT => self.parse_goto_stat(),
                    KeyWord::DO => self.parse_do_stat(),
                    KeyWord::WHI => self.parse_while_stat(),
                    KeyWord::REP => self.parse_repeat_stat(),
                    KeyWord::IF => self.parse_if_stat(),
                    KeyWord::FOR => self.parse_for_stat(),
                    KeyWord::FUN => self.parse_func_def_stat(),
                    KeyWord::LOC => self.parse_local_stat(),
                    _ => self.parse_func_call_or_assign_stat()
                }
            }
            _ => self.parse_func_call_or_assign_stat()
        }
    }

    fn parse_empty_stat(&mut self) -> Box<dyn Stat> {
        self.expected_token(tk_from_kw!(KeyWord::SEM));
        Box::new(EmptyStat {})
    }
    fn parse_break_stat(&mut self) -> Box<dyn Stat> {
        self.expected_token(tk_from_kw!(KeyWord::BRK));
        Box::new(BreakStat { line: 0 })
    }
    fn parse_label_stat(&mut self) -> Box<dyn Stat> {
        self.expected_token(tk_from_kw!(KeyWord::PATH));
        let name = self.expected_id().raw_data;
        self.expected_token(tk_from_kw!(KeyWord::PATH));
        Box::new(LabelStat { name })
    }
    fn parse_goto_stat(&mut self) -> Box<dyn Stat> {
        self.expected_token(tk_from_kw!(KeyWord::GOT));
        let name = self.expected_id().raw_data;
        Box::new(GotoStat { target: name })
    }
    fn parse_do_stat(&mut self) -> Box<dyn Stat> {
        self.expected_token(tk_from_kw!(KeyWord::DO));
        let block = self.parse();
        self.expected_token(tk_from_kw!(KeyWord::END));
        Box::new(DoStat { block })
    }
    fn parse_while_stat(&mut self) -> Box<dyn Stat> {
        self.expected_token(tk_from_kw!(KeyWord::WHI));
        let exp = self.parse_exp();
        self.expected_token(tk_from_kw!(KeyWord::DO));
        let block = self.parse();
        self.expected_token(tk_from_kw!(KeyWord::END));
        Box::new(WhileStat { exp, block })
    }
    fn parse_repeat_stat(&mut self) -> Box<dyn Stat> {
        self.expected_token(tk_from_kw!(KeyWord::REP));
        let block = self.parse();
        self.expected_token(tk_from_kw!(KeyWord::UNT));
        let exp = self.parse_exp();
        Box::new(RepeatStat { block, exp })
    }
    fn parse_if_stat(&mut self) -> Box<dyn Stat> {
        let mut exps = Vec::new();
        let mut blocks = Vec::new();
        self.expected_token(tk_from_kw!(KeyWord::IF));
        exps.push(self.parse_exp());
        self.expected_token(tk_from_kw!(KeyWord::THE));
        blocks.push(self.parse());

        while self.lexer.peek_token_type().eq(&tk_from_kw!(KeyWord::ELI)) {
            self.lexer.next_token();
            exps.push(self.parse_exp());
            self.expected_token(tk_from_kw!(KeyWord::THE));
            blocks.push(self.parse());
        }

        if self.lexer.peek_token_type().eq(&tk_from_kw!(KeyWord::ELS)) {
            exps.push(Box::new(TrueExp { line: 0 }));
            blocks.push(self.parse());
        }

        Box::new(IfStat { exps, blocks })
    }
    fn parse_for_stat(&mut self) -> Box<dyn Stat> {
        self.expected_token(tk_from_kw!(KeyWord::FOR));
        let name = self.expected_id().raw_data;
        if self.lexer.peek_token_type().eq(&tk_from_kw!(KeyWord::ASS)) {
            self.parse_step_for_stat(name)
        } else {
            self.parse_range_for_stat(name)
        }
    }
    fn parse_step_for_stat(&mut self, first_val: String) -> Box<dyn Stat> {
        self.expected_token(tk_from_kw!(KeyWord::ASS));
        let init_exp = self.parse_exp();
        self.expected_token(tk_from_kw!(KeyWord::COM));
        let lim_exp = self.parse_exp();

        let step_exp: Box<dyn Exp>;

        if self.lexer.peek_token_type().eq(&tk_from_kw!(KeyWord::COM)) {
            self.lexer.next_token();
            step_exp = self.parse_exp();
        } else {
            step_exp = Box::new(IntegerExp { line: 0, num: 1 });
        }

        self.expected_token(tk_from_kw!(KeyWord::DO));
        let block = self.parse();
        self.expected_token(tk_from_kw!(KeyWord::END));

        Box::new(StepForStat {
            beg_line: 0,
            block_beg_line: 0,
            var_name: name,
            init_exp,
            lim_exp,
            step_exp,
            block,
        })
    }
    fn parse_range_for_stat(&mut self, first_val: String) -> Box<dyn Stat> {
        let name_list = self.parse_name_list();
        self.expected_token(tk_from_kw!(KeyWord::ASS));
        let exp_list = self.parse_exp_list();
        self.expected_token(tk_from_kw!(KeyWord::DO));
        let block = self.parse();
        self.expected_token(tk_from_kw!(KeyWord::END));
        Box::new(RangeForStat { block_beg_line: 0, name_list, exp_list, block })
    }
    fn parse_func_def_stat(&mut self) -> FuncDefExp {}
    fn parse_local_stat(&mut self) -> Box<dyn Stat> {
        self.expected_token(tk_from_kw!(KeyWord::LOC));
        if self.lexer.peek_token_type().eq(&tk_from_kw!(KeyWord::FUN)) {
            self.parse_local_func_def_stat()
        } else {
            self.parse_local_val_stat()
        }
    }
    fn parse_local_func_stat(&mut self) -> Box<dyn Stat> {
        self.expected_token(tk_from_kw!(KeyWord::FUN));
        let name = self.expected_id().raw_data;
        let func_body = self.parse_func_def_stat();
        Box::new(LocalFuncDefStat { name, exp: func_body })
    }
    fn parse_local_val_stat(&mut self) -> Box<dyn Stat> {
        let name_list = self.parse_name_list();
        let mut exp_list = Vec::new();
        if self.lexer.peek_token_type().eq(&tk_from_kw!(KeyWord::ASS)) {
            self.lexer.next_token();
            exps = self.parse_exp_list();
        }
        Box::new(LocalVarDefStat { last_line: 0, name_list, exp_list })
    }
    fn parse_func_call_or_assign_stat(&mut self) -> Box<dyn Stat> {}
    fn parse_func_call_stat(&mut self) -> Box<dyn Stat> {}
    fn parse_assign_stat(&mut self) -> Box<dyn Stat> {}
}

impl Parser {
    fn parse_exp_list(&mut self) -> Vec<Box<dyn Exp>> {
        let mut exps = vec![self.parse_exp()];
        while self.lexer.peek_token_type().eq(&tk_from_kw!(KeyWord::COM)) {
            self.lexer.next_token();
            exps.push(self.parse_exp());
        }
        exps
    }

    fn parse_name_list(&mut self) -> Vec<String> {}
}

impl Parser {
    fn parse_exp(&mut self) -> Box<dyn Exp> {}

    fn parse_prefix_exp(&mut self) -> Box<dyn Exp> {}
}

impl Parser {
    fn parse_stats(&mut self) -> Vec<Box<dyn Stat>> {
        let mut stats = Vec::new();
        while !Parser::is_ret_or_block_end(&self.lexer.peek_token_type()) {
            let stat = self.parse_stat();
            if let StatType::EmptyStatTag = stat.get_type() {
                continue;
            }
            stats.push(stat);
        }
        stats
    }

    fn parse_ret_exps(&mut self) -> Option<Vec<Box<dyn Exp>>> {
        let mut exps = Vec::new();
        if !self.lexer.peek_token_type().eq(tk_from_kw!(KeyWord::RET)) {
            return None;
        }
        self.lexer.next_token();
        Some(match self.lexer.peek_token_type() {
            TokenType::OptKeyWord(key_word) => {
                match key_word {
                    KeyWord::END |
                    KeyWord::ELI |
                    KeyWord::ELS |
                    KeyWord::UNT => exps,
                    KeyWord::SEM => {
                        self.lexer.next_token();
                        exps
                    }
                    _ => panic!("grammar error!")
                }
            }
            TokenType::EOF => exps,
            _ => {
                exps = self.parse_exp_list();
                if self.lexer.peek_token_type().eq(tk_from_kw!(KeyWord::SEM)) {
                    self.lexer.next_token();
                }
                exps
            }
        })
    }

    pub fn parse(&mut self) -> Block {
        let stats = self.parse_stats();
        let opt_ret_exps = self.parse_ret_exps();
        Block::new(0, stats, opt_ret_exps)
    }
}

