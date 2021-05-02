use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::syntax_file_parser::SyntaxParser;

pub mod util;
pub mod syntax;
pub mod symbol;
pub mod syntax_file_parser;

fn main() {
    let file: File = File::open("D:\\test.txt").unwrap_or_else(|_| { panic!("File open error\n") });
    let mut parser = SyntaxParser::new();
    let lines = BufReader::new(file);
    for line in lines.lines() {
        parser.add_syntax_line(&mut line.unwrap_or_else(|_| { panic!("line error\n") }));
    }
}
