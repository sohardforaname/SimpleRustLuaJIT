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
        let mut syntax = Syntax::new(&parser.symbols, &parser.generators);
        match syntax.check_if_ll() {
            Ok(status) => {
                println!("the LL(1) checker result of the grammar: true");
                let table = syntax.build_analyze_table();
            }
            Err(set) => {
                println!("the LL(1) checker result of the grammar: false. failed symbol:");
                for symbol in set.iter() {
                    print!("{} ", symbol);
                }
                print!("\n");
            }
        }
    }
}
