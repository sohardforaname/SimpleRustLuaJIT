use std::collections::HashMap;
use crate::string_hash_map;
use std::fmt::{Display, Formatter, Result, Debug};

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

    pub fn get_display_str(&self) -> String {
        match *self {
            KeyWord::ADD => "ADD".to_string(),
            KeyWord::SUB => "SUB".to_string(),
            KeyWord::MUL => "MUL".to_string(),
            KeyWord::DIV => "DIV".to_string(),
            KeyWord::MOD => "MOD".to_string(),
            KeyWord::POW => "POW".to_string(),
            KeyWord::ASS => "ASS".to_string(),
            KeyWord::EQU => "EQU".to_string(),
            KeyWord::NEQ => "NEQ".to_string(),
            KeyWord::GR  => "GR".to_string(),
            KeyWord::LE  => "LE".to_string(),
            KeyWord::GRE => "GRE".to_string(),
            KeyWord::LEE => "LEE".to_string(),
            KeyWord::CON => "CON".to_string(),
            KeyWord::DOT => "DOT".to_string(),
            KeyWord::AND => "AND".to_string(),
            KeyWord::OR  => "OR".to_string(),
            KeyWord::LEN => "LEN".to_string(),
            KeyWord::MIN => "MIN".to_string(),
            KeyWord::NOT => "NOT".to_string(),
            KeyWord::LSM => "LSM".to_string(),
            KeyWord::RSM => "RSM".to_string(),
            KeyWord::LMI => "LMI".to_string(),
            KeyWord::RMI => "RMI".to_string(),
            KeyWord::LLA => "LLA".to_string(),
            KeyWord::RLA => "RLA".to_string(),
            KeyWord::COM => "COM".to_string(),
            KeyWord::SEM => "SEM".to_string(),

            KeyWord::BRK => "BRK".to_string(),
            KeyWord::DO  => "DO".to_string(),
            KeyWord::ELS => "ELS".to_string(),
            KeyWord::ELI => "ELI".to_string(),
            KeyWord::END => "END".to_string(),
            KeyWord::FAL => "FAL".to_string(),
            KeyWord::FOR => "FOR".to_string(),
            KeyWord::FUN => "FUN".to_string(),
            KeyWord::IF  => "IF".to_string(),
            KeyWord::IN  => "IN".to_string(),
            KeyWord::LOC => "LOC".to_string(),
            KeyWord::NIL => "NIL".to_string(),
            KeyWord::REP => "REP".to_string(),
            KeyWord::RET => "RET".to_string(),
            KeyWord::THE => "THE".to_string(),
            KeyWord::TRU => "TRU".to_string(),
            KeyWord::UNT => "UNT".to_string(),
            KeyWord::WHI => "WHI".to_string(),
        }
    }
}

impl PartialEq<KeyWord> for KeyWord {
    fn eq(&self, key_word: &KeyWord) -> bool {
        *key_word == self.clone()
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    OptKeyWord(KeyWord),
    ID(String),
    String(String),
    Number(f64),
    None,
}

impl From<KeyWord> for TokenType {
    fn from(key_word: KeyWord) -> TokenType {
        TokenType::OptKeyWord(key_word)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match self {
            TokenType::OptKeyWord(key_word) => "KEYWORD",
            TokenType::Number(num) => "NUM",
            TokenType::String(str) => "STR",
            TokenType::ID(str) => "ID",
            _ => "None"
        })
    }
}

#[derive(Debug)]
pub struct Token {
    pub token: TokenType,
    pub raw_data: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token: TokenType, raw_data: String, line: usize, column: usize) -> Token {
        Token { token, raw_data, line, column }
    }
    pub fn get_id(&self) -> Option<&String> {
        match self.token {
            TokenType::ID(ref id) => Some(id),
            _ => None
        }
    }

    pub fn get_num(&self) -> Option<&f64> {
        match self.token {
            TokenType::Number(ref num) => Some(num),
            _ => None
        }
    }

    pub fn get_key_word(&self) -> Option<&KeyWord> {
        match self.token {
            TokenType::OptKeyWord(ref key) => Some(key),
            _ => None
        }
    }

    pub fn eof() -> Token {
        Token {
            token: TokenType::None,
            raw_data: "".to_string(),
            line: 0,
            column: 0,
        }
    }
}

impl PartialEq<Token> for KeyWord {
    fn eq(&self, token: &Token) -> bool {
        token.token == TokenType::from(self.clone())
    }
}

impl Into<TokenType> for Token {
    fn into(self) -> TokenType {
        self.token.clone()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Token: {{type: {}, raw: {}, line: {}, column: {}}}",
               self.token, self.raw_data, self.line, self.column)
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