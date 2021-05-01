mod ast;
mod util;

use ast::lexer::Lexer;

fn main() {
    let mut lexer: Lexer = Lexer::new(
        &"local a = 123 --text
        a = 1 --comment2".to_string());
    for token in lexer {
        println!("{}", token);
    }
}
