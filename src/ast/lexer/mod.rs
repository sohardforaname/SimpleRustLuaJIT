pub mod token;
pub mod util;

use std::iter::{Peekable, FromIterator, IntoIterator};
use std::collections::HashMap;
use token::{get_key_word_map, get_opt_map, TokenType, Token};
use crate::ast::lexer::token::KeyWord;
use std::vec::IntoIter;
use std::ops::Not;

pub struct Lexer {
    char_iter: Peekable<IntoIter<char>>,
    opt_hash_map: HashMap<String, KeyWord>,
    key_word_hash_map: HashMap<String, KeyWord>,
    cur_line: usize,
    cur_column: usize,
    eof: bool,
}

impl Lexer {
    pub fn new(source_code: &String) -> Lexer {
        let chars = Vec::from_iter(source_code.chars());
        Lexer {
            char_iter: chars.into_iter().peekable(),
            opt_hash_map: get_opt_map(),
            key_word_hash_map: get_key_word_map(),
            cur_line: 1,
            cur_column: 1,
            eof: false,
        }
    }

    fn iter_advance(&mut self, n: usize) {
        self.cur_column += n;
    }

    fn iter_new_line(&mut self) {
        self.cur_line += 1;
        self.cur_column = 1;
    }

    fn take_word<F>(&mut self, filter: F) -> String
        where F: Fn(&char) -> bool {
        let mut res: String = String::new();
        while if let Some(ch) = self.char_iter.peek() {
            filter(ch)
        } else {
            false
        } {
            res.push(self.char_iter.next().unwrap());
        }
        res
    }

    fn parse_id(&mut self) -> (TokenType, String) {
        let char_filter = |ch: &char| {
            ch.is_alphanumeric() || *ch == '_'
        };
        //let id_str: String = self.char_iter.by_ref().take_while(char_filter).collect();
        let id_str = self.take_word(char_filter);

        self.iter_advance(id_str.len());
        match self.key_word_hash_map.get(&id_str) {
            Some(key_word) => (TokenType::OptKeyWord(key_word.clone()), id_str.clone()),
            None => (TokenType::ID(id_str.clone()), id_str.clone())
        }
    }

    fn parse_number(&mut self) -> (TokenType, String) {
        let num_filter = |ch: &char| {
            ch.is_alphanumeric() || *ch == '.' || *ch == '_'
        };
        let num_str: String = self.take_word(num_filter);

        self.iter_advance(num_str.len());
        (TokenType::Number(num_str.parse::<f64>().unwrap_or_else(|_| {
            panic!("Parse number token error: '{}'", num_str.clone())
        })), num_str.clone())
    }

    fn parse_str(&mut self) -> (TokenType, String) {
        let str_filter = |ch: &char| {
            *ch != '"'
        };
        self.char_iter.next();
        let str: String = self.take_word(str_filter);

        self.char_iter.next();
        self.iter_advance(str.len() + 2);
        (TokenType::String(str.clone()), str.clone())
    }

    fn parser_operator(&mut self) -> (TokenType, String) {
        for len in (1..4).rev() {
            let ope_str: String = self.char_iter.clone().take(len).collect();
            if let Some(key_word) = self.opt_hash_map.get(&ope_str).cloned() {
                self.char_iter.by_ref().take(len).count();
                self.iter_advance(len);
                return (TokenType::OptKeyWord(key_word), ope_str);
            }
        }
        panic!("Parse operator token error at {}: {}", self.cur_line, self.cur_column)
    }

    fn skip_comment(&mut self) {
        while if let Some(ch) = self.char_iter.peek() {
            (|ch: &char| {
                *ch != '\n'
            })(ch)
        } else {
            false
        } {
            self.char_iter.next();
        }
        self.char_iter.next();
        self.iter_new_line();
    }

    fn handle_sub_and_comment(&mut self, cur_line: usize, cur_column: usize) -> Option<Token> {
        self.char_iter.next();
        self.iter_advance(1);
        let next_val = self.char_iter.peek().unwrap_or_else(|| { &'\n' });
        if *next_val == '-' {
            self.char_iter.next();
            self.skip_comment();
            return self.get_next_token();
        }
        Some(Token::new(TokenType::OptKeyWord(KeyWord::SUB), "-".to_string(), cur_line, cur_column))
    }


    pub fn get_next_token(&mut self) -> Option<Token> {
        match self.char_iter.peek() {
            Some(val) => {
                let token_info: (TokenType, String);
                let cur_line = self.cur_line;
                let cur_column = self.cur_column;

                if val.is_numeric() {
                    token_info = self.parse_number();
                } else if val.is_alphanumeric() || *val == '_' {
                    token_info = self.parse_id();
                } else if *val == '"' {
                    token_info = self.parse_str();
                } else if !val.is_ascii_graphic() {
                    loop {
                        match self.char_iter.peek() {
                            Some(ch) => {
                                if *ch == '\n' {
                                    self.iter_new_line();
                                } else if ch.is_ascii_graphic().not() {
                                    self.iter_advance(1);
                                } else {
                                    break;
                                }
                            }
                            None => {
                                break;
                            }
                        }
                        self.char_iter.next();
                    }
                    return self.get_next_token();
                } else if *val == '-' {
                    return self.handle_sub_and_comment(cur_line, cur_column);
                } else {
                    token_info = self.parser_operator();
                }
                Some(Token::new(token_info.0, token_info.1, cur_line, cur_column))
            }
            None => {
                if !self.eof {
                    self.eof = true;
                    Some(Token::eof())
                } else {
                    None
                }
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.get_next_token()
    }
}