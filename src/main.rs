mod ast;
mod util;

use ast::lexer::Lexer;
use crate::ast::parser::Parser;

fn main() {
    /*
    let mut parser: Parser = Parser::new(&"local a = 123 --text
        a = 1 --comment2".to_string(), &"D:\\test.txt".to_string());
    parser.check_grammar();
    */
    let mut lexer = Lexer::new(&"local a = 123 --text
    a = 1 --comment2".to_string());
    for token in lexer {
        println!("{}", token);
    }
}
