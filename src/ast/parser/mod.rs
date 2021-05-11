mod load_tb;

use crate::ast::lexer::Lexer;
use ll_syntax_proofer::symbol::Symbol;
use crate::ast::parser::load_tb::AnalyzeTable;
use crate::ast::lexer::token::Token;
use std::fmt::{Display, Formatter};

pub struct Parser {
    lexer: Lexer,
    analyze_table: AnalyzeTable,
}

fn print_stack(symbol_stack: &Vec<Symbol>) -> String {
    let mut str = String::new();
    str.push_str("[");
    for sym in symbol_stack.iter() {
        str.push_str(sym.text.as_str());
        str.push_str(" ")
    }
    str.push_str("]");
    str
}

impl Parser {
    pub fn new(source_code: &String, syntax_file_path: &String) -> Parser {
        Parser {
            lexer: Lexer::new(source_code),
            analyze_table: AnalyzeTable::get_table_from_syntax_file(syntax_file_path),
        }
    }

    pub fn check_grammar(&mut self) {
        let mut symbol_stack: Vec<Symbol> = vec![Symbol::eof_symbol(), Symbol::from("S")];

        let mut token = self.lexer.next().unwrap();
        let mut top = symbol_stack.last().unwrap();
        let mut cur_line = 1;
        let mut cur_column = 0;

        let eof_symbol = Symbol::eof_symbol();

        loop {
            println!("{}, {}", token, top);
            println!("stack: {}", print_stack(&symbol_stack));
            if top.eq(&eof_symbol) && token.eq(&Token::eof()) {
                break;
            } else if top.is_end_symbol() || top.eq(&eof_symbol) {
                if token.eq(top) {
                    symbol_stack.pop();
                    cur_line = token.line;
                    cur_column = token.column;
                    token = self.lexer.next().unwrap();
                } else {
                    panic!("looking for symbol: {} after symbol at ({}, {})", top, cur_line, cur_column);
                }
            } else {
                if let Some(production) = self.analyze_table
                    .analyze_table.get(&(top.clone(), token.to_symbol())) {
                    symbol_stack.pop();
                    for symbol in production.vec.iter().rev() {
                        if !symbol.is_empty_symbol() {
                            symbol_stack.push(symbol.clone());
                        }
                    }
                } else {
                    panic!("expand symbol: {} error", top);
                }
            }
            top = symbol_stack.last().unwrap();
        }
    }
}
