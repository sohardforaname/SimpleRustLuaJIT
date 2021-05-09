use std::collections::HashMap;
use crate::string_hash_map;
use std::fmt::{Display, Formatter, Result, Debug};
use ll_syntax_proofer::symbol::Symbol;

#[allow(dead_code)]
#[derive(Debug, Clone, Hash)]
pub enum KeyWord {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    POW,
    ASS,
    EQU,
    NEQ,
    GR,
    LE,
    GRE,
    LEE,
    CON,
    DOT,
    AND,
    OR,
    LEN,
    MIN,
    NOT,
    LSM,
    RSM,
    LMI,
    RMI,
    LLA,
    RLA,
    COM,
    SEM,

    BRK,
    DO,
    ELS,
    ELI,
    END,
    FAL,
    FOR,
    FUN,
    IF,
    IN,
    LOC,
    NIL,
    REP,
    RET,
    THE,
    TRU,
    UNT,
    WHI,
}

impl KeyWord {
    pub fn is_binary_operator(&self) -> bool {
        match *self {
            KeyWord::ADD |
            KeyWord::SUB |
            KeyWord::MUL |
            KeyWord::DIV |
            KeyWord::MOD |
            KeyWord::POW |
            KeyWord::ASS |
            KeyWord::EQU |
            KeyWord::NEQ |
            KeyWord::GR |
            KeyWord::LE |
            KeyWord::GRE |
            KeyWord::LEE |
            KeyWord::CON |
            KeyWord::DOT |
            KeyWord::COM |
            KeyWord::AND |
            KeyWord::OR => true,
            _ => false
        }
    }

    pub fn is_unique_operator(&self) -> bool {
        match *self {
            KeyWord::LEN |
            KeyWord::MIN |
            KeyWord::NOT => true,
            _ => false
        }
    }

    pub fn is_divide_operator(&self) -> bool {
        match *self {
            KeyWord::LSM |
            KeyWord::RSM |
            KeyWord::LMI |
            KeyWord::RMI |
            KeyWord::LLA |
            KeyWord::RLA => true,
            _ => false
        }
    }

    pub fn get_display_str(&self) -> &'static str {
        match *self {
            KeyWord::ADD => "+",
            KeyWord::SUB => "-",
            KeyWord::MUL => "*",
            KeyWord::DIV => "/",
            KeyWord::MOD => "%",
            KeyWord::POW => "^",
            KeyWord::ASS => "=",
            KeyWord::EQU => "==",
            KeyWord::NEQ => "~=",
            KeyWord::GR => ">",
            KeyWord::LE => "<",
            KeyWord::GRE => ">=",
            KeyWord::LEE => "<=",
            KeyWord::CON => "..",
            KeyWord::DOT => ".",
            KeyWord::AND => "and",
            KeyWord::OR => "or",
            KeyWord::LEN => "#",
            KeyWord::MIN => "-",
            KeyWord::NOT => "not",
            KeyWord::LSM => "(",
            KeyWord::RSM => ")",
            KeyWord::LMI => "[",
            KeyWord::RMI => "]",
            KeyWord::LLA => "{",
            KeyWord::RLA => "}",
            KeyWord::COM => ",",
            KeyWord::SEM => ";",

            KeyWord::BRK => "break",
            KeyWord::DO => "do",
            KeyWord::ELS => "else",
            KeyWord::ELI => "elseif",
            KeyWord::END => "end",
            KeyWord::FAL => "false",
            KeyWord::FOR => "for",
            KeyWord::FUN => "function",
            KeyWord::IF => "if",
            KeyWord::IN => "in",
            KeyWord::LOC => "local",
            KeyWord::NIL => "nil",
            KeyWord::REP => "repeat",
            KeyWord::RET => "return",
            KeyWord::THE => "then",
            KeyWord::TRU => "true",
            KeyWord::UNT => "until",
            KeyWord::WHI => "while",
        }
    }
}

impl PartialEq<KeyWord> for KeyWord {
    fn eq(&self, key_word: &KeyWord) -> bool {
        *key_word == *self
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    OptKeyWord(KeyWord),
    ID(String),
    String(String),
    Number(f64),
    EOF,
}

impl From<KeyWord> for TokenType {
    fn from(key_word: KeyWord) -> TokenType {
        TokenType::OptKeyWord(key_word)
    }
}

impl TokenType {
    pub fn to_str(&self) -> &'static str {
        match self {
            TokenType::OptKeyWord(key_word) => key_word.get_display_str(),
            TokenType::Number(num) => "num",
            TokenType::String(str) => "str",
            TokenType::ID(str) => "id",
            _ => "eof"
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_str())
    }
}

impl From<TokenType> for String {
    fn from(token_type: TokenType) -> String {
        token_type.to_string()
    }
}

#[derive(Debug)]
pub struct Token {
    pub type_id: TokenType,
    pub raw_data: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(type_id: TokenType, raw_data: String, line: usize, column: usize) -> Token {
        Token { type_id, raw_data, line, column }
    }
    pub fn get_id(&self) -> Option<&String> {
        match self.type_id {
            TokenType::ID(ref id) => Some(id),
            _ => None
        }
    }

    pub fn get_num(&self) -> Option<&f64> {
        match self.type_id {
            TokenType::Number(ref num) => Some(num),
            _ => None
        }
    }

    pub fn get_key_word(&self) -> Option<&KeyWord> {
        match self.type_id {
            TokenType::OptKeyWord(ref key) => Some(key),
            _ => None
        }
    }

    pub fn to_symbol(&self) -> Symbol {
        Symbol { text: self.type_id.to_str().to_string() }
    }

    pub fn eof() -> Token {
        Token {
            type_id: TokenType::EOF,
            raw_data: "".to_string(),
            line: 0,
            column: 0,
        }
    }
}

impl PartialEq<Token> for KeyWord {
    fn eq(&self, token: &Token) -> bool {
        token.type_id == TokenType::from(self.clone())
    }
}

impl PartialEq<Token> for Token {
    fn eq(&self, token: &Token) -> bool {
        token.raw_data == self.raw_data
    }
}


impl PartialEq<Symbol> for Token {
    fn eq(&self, symbol: &Symbol) -> bool {
        self.type_id.to_str().eq(&symbol.text)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Token: {{type: {}, raw: {}, line: {}, column: {}}}",
               self.type_id, self.raw_data, self.line, self.column)
    }
}

pub fn get_opt_map() -> HashMap<String, KeyWord> {
    string_hash_map![
        "+"     =>  KeyWord::ADD,
        "-"     =>  KeyWord::SUB,
        "*"     =>  KeyWord::MUL,
        "/"     =>  KeyWord::DIV,
        "%"     =>  KeyWord::MOD,
        "^"     =>  KeyWord::POW,
        "="     =>  KeyWord::ASS,
        "=="    =>  KeyWord::EQU,
        "~="    =>  KeyWord::NEQ,
        ">"     =>  KeyWord::GR,
        "<"     =>  KeyWord::LE,
        ">="    =>  KeyWord::GRE,
        "<="    =>  KeyWord::LEE,
        ".."    =>  KeyWord::CON,
        "."     =>  KeyWord::DOT,
        "and"   =>  KeyWord::AND,
        "or"    =>  KeyWord::OR,
        "#"     =>  KeyWord::LEN,
        "-"     =>  KeyWord::MIN,
        "not"   =>  KeyWord::NOT,
        "("     =>  KeyWord::LSM,
        ")"     =>  KeyWord::RSM,
        "["     =>  KeyWord::LMI,
        "]"     =>  KeyWord::RMI,
        "{"     =>  KeyWord::LLA,
        "}"     =>  KeyWord::RLA,
        ","     =>  KeyWord::COM,
        ";"     =>  KeyWord::SEM
    ]
}

pub fn get_key_word_map() -> HashMap<String, KeyWord> {
    string_hash_map![
        "break"     =>  KeyWord::BRK,
        "do"        =>  KeyWord::DO,
        "else"      =>  KeyWord::ELS,
        "elseif"    =>  KeyWord::ELI,
        "end"       =>  KeyWord::END,
        "false"     =>  KeyWord::FAL,
        "for"       =>  KeyWord::FOR,
        "function"  =>  KeyWord::FUN,
        "if"        =>  KeyWord::IF,
        "in"        =>  KeyWord::IN,
        "local"     =>  KeyWord::LOC,
        "nil"       =>  KeyWord::NIL,
        "repeat"    =>  KeyWord::REP,
        "return"    =>  KeyWord::RET,
        "then"      =>  KeyWord::THE,
        "true"      =>  KeyWord::TRU,
        "until"     =>  KeyWord::UNT,
        "while"     =>  KeyWord::WHI
    ]
}