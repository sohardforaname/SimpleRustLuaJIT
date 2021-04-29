pub mod token;
pub mod util;

use std::iter::{Peekable, FromIterator, IntoIterator};
use std::collections::HashMap;
use token::{get_key_word_map, get_opt_map, TokenType, Token};
use crate::ast::lexer::token::KeyWord;
use std::vec::IntoIter;
use util::{CondTake};

pub struct Lexer {
    char_iter: Peekable<IntoIter<char>>,
    opt_hash_map: HashMap<String, KeyWord>,
    key_word_hash_map: HashMap<String, KeyWord>,
    cur_line: usize,
    cur_column: usize,
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
        }
    }

    fn iter_advance(&mut self, n: usize) {
        self.cur_column += n;
    }

    fn iter_new_line(&mut self) {
        self.cur_line += 1;
        self.cur_column = 1;
    }

    fn parse_id(&mut self) -> TokenType {
        let char_filter = |ch: &char| {
            ch.is_alphanumeric() || *ch == '_'
        };
        let id_str: String = self.char_iter.take_conditional(char_filter).collect();

        self.iter_advance(id_str.len());
        match self.key_word_hash_map.get(&id_str) {
            Some(key_word) => TokenType::OptKeyWord(key_word.clone()),
            _ => TokenType::ID(id_str)
        }
    }

    fn parse_number(&mut self) -> TokenType {
        let num_filter = |ch: &char| {
            ch.is_numeric() || *ch == '.'
        };
        //let num_str: String = self.char_iter.take_conditional(num_filter).collect();
        let num_str: String = self.char_iter.clone().take_while(num_filter).collect();
        self.char_iter.by_ref().take(num_str.len()).count();

        self.iter_advance(num_str.len());
        TokenType::Number(num_str.parse::<f64>().unwrap_or_else(|_| {
            panic!("Parse number token error: '{}'", num_str)
        }))
    }

    fn parse_str(&mut self) -> TokenType {
        let str_filter = |ch: &char| {
            *ch != '"'
        };
        self.char_iter.next();
        let str: String = self.char_iter.take_conditional(str_filter).collect();

        self.char_iter.next();
        self.iter_advance(str.len() + 2);
        TokenType::String(str)
    }

    fn parser_operator(&mut self) -> TokenType {
        for len in (1..4).rev() {
            let ope_str: String = self.char_iter.clone().take(len).collect();
            if let Some(key_word) = self.opt_hash_map.get(&ope_str).cloned() {
                self.char_iter.by_ref().take(len).count();
                self.iter_advance(len);
                return TokenType::OptKeyWord(key_word);
            }
        }
        panic!("Parse operator token error at {}: {}", self.cur_line, self.cur_column)
    }

    fn skip_space<F>(&mut self, mut filter: F)
        where F: FnMut(&char) -> bool {
        while if let Some(ch) = self.char_iter.peek() {
            filter(ch)
        } else {
            false
        } {
            self.char_iter.next();
            self.iter_advance(1);
        }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        match self.char_iter.peek() {
            Some(val) => {
                if val.is_numeric() {
                    Some(Token::new(self.parse_number(), self.cur_line, self.cur_column))
                } else if val.is_alphanumeric() || *val == '_' {
                    Some(Token::new(self.parse_id(), self.cur_line, self.cur_column))
                } else if *val == '"' {
                    Some(Token::new(self.parse_str(), self.cur_line, self.cur_column))
                } else if *val == '\n' {
                    while if let Some(ch) = self.char_iter.peek() {
                        (|ch: &char| {
                            *ch == '\n'
                        })(ch)
                    } else {
                        false
                    } {
                        self.char_iter.next();
                        self.iter_new_line();
                    }
                    self.get_next_token()
                } else if *val == ' ' || *val == '\t' {
                    while if let Some(ch) = self.char_iter.peek() {
                        (|ch: &char| {
                            *ch == ' ' || *ch == '\t'
                        })(ch)
                    } else {
                        false
                    } {
                        self.char_iter.next();
                        self.iter_advance(1);
                    }
                    self.get_next_token()
                } else {
                    Some(Token::new(self.parser_operator(), self.cur_line, self.cur_column))
                }
            }
            None => None
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.get_next_token()
    }
}