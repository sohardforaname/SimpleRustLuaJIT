mod load_tb;

use super::lexer;
use crate::ast::lexer::Lexer;
use ll_syntax_proofer::symbol::Symbol;
use crate::ast::parser::load_tb::AnalyzeTable;

pub struct Parser {
    lexer: Lexer,
    analyze_table: AnalyzeTable,
}

impl Parser {
    pub fn new(source_code: &String, syntax_file_path: &String) -> Parser {
        Parser {
            lexer: Lexer::new(source_code),
            analyze_table: AnalyzeTable::get_table_from_syntax_file(syntax_file_path),
        }
    }

    pub fn check_grammar(&mut self) {
        let mut symbol_stack: Vec<Symbol> = Vec::new();
        symbol_stack.push(Symbol::from("S"));
        let mut cur_line: usize = 1;
        let mut cur_column: usize = 1;
        while symbol_stack.len() {
            let top = symbol_stack.last().unwrap();
            if top.is_end_symbol() {
                if let Some(token) = self.lexer.next() {
                    cur_line = token.line;
                    cur_line = token.column;
                    if Symbol::from(token.raw_data).eq(top) {
                        symbol_stack.pop();
                        continue;
                    }
                }
                panic!("Expect Token: {} in line: {}, column: {}\n", top, cur_line, cur_column);
            } else {
                if let Some(token) = self.lexer.next() {
                    symbol_stack.pop();
                    cur_line = token.line;
                    cur_line = token.column;
                    let token_symbol = Symbol::from(token.raw_data);
                    while top.is_not_end_symbol() {
                        let production = self.analyze_table.analyze_table.get(
                            &(top.clone(), token_symbol.clone())
                        ).unwrap();
                        for symbol in production.vec.iter().rev() {
                            symbol_stack.push(symbol.clone());
                        }
                        let top = symbol_stack.last().unwrap();
                    }
                    if token_symbol.eq(top) {
                        symbol_stack.pop();
                        continue;
                    }
                }
                panic!("Expect Token: {} in line: {}, column: {}\n", top, cur_line, cur_column);
            }
        }
    }
}
