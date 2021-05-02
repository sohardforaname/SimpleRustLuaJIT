use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::syntax_file_parser::SyntaxParser;
use crate::syntax::Syntax;

pub mod util;
pub mod syntax;
pub mod symbol;
pub mod syntax_file_parser;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::syntax::Syntax;
    use crate::syntax_file_parser::SyntaxParser;
    use std::io::{BufReader, BufRead};

    #[test]
    fn it_works() {
        let file: File = File::open("D:\\test.txt").unwrap_or_else(|_| { panic!("File open error\n") });
        let mut parser = SyntaxParser::new();
        let lines = BufReader::new(file);
        for line in lines.lines() {
            parser.add_syntax_line(line.unwrap_or_else(|_| { panic!("line error\n") }));
        }
        let mut syntax = Syntax::new(&parser.generators);
        syntax.calc_empty();

        let i = 0;
    }
}