mod ast;
mod util;

use ast::lexer::Lexer;
use crate::ast::parser::Parser;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};

fn main() {
    /*
    let mut parser: Parser = Parser::new(&"local a = 123 + 6 b = 1"
        .to_string(), &"D:\\test.txt".to_string());
    parser.check_grammar();
    */

    let mut file: File = File::open("D:\\testLua.lua")
        .unwrap_or_else(|_| { panic!("File open error\n"); });
    let mut code = String::new();
    file.read_to_string(&mut code);

    let mut lexer = Lexer::new(&code);
    for token in lexer {
        println!("{}", token);
    }
}
