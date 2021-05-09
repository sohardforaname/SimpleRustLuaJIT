use std::collections::HashMap;
use ll_syntax_proofer::symbol::{Symbol, Production};
use ll_syntax_proofer::syntax::Syntax;
use std::fs::File;
use ll_syntax_proofer::syntax_file_parser::SyntaxParser;
use std::io::{BufReader, BufRead};
use std::path::Path;

pub struct AnalyzeTable {
    pub analyze_table: HashMap<(Symbol, Symbol), Production>,
    pub empty_symbol: Symbol,
}

impl AnalyzeTable {
    pub fn get_table_from_syntax_file(file_path: &String) -> AnalyzeTable {
        let file: File = File::open(file_path).unwrap_or_else(|_| { panic!("File open error\n") });
        let mut parser = SyntaxParser::new();
        let lines = BufReader::new(file);
        for line in lines.lines() {
            parser.add_syntax_line(line.unwrap_or_else(|_| { panic!("line error\n") }));
        }
        let mut syntax = Syntax::new(&parser.symbols, &parser.generators);
        if !syntax.check_if_ll() {
            panic!("Not LL(1) grammar");
        }
        AnalyzeTable {
            analyze_table: syntax.build_analyze_table(),
            empty_symbol: syntax.empty_symbol
        }
    }
}