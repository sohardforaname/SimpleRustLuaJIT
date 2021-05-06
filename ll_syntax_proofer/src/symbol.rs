use std::borrow::Borrow;
use std::fmt::{Display, Formatter, Result, Debug};

#[derive(Clone, Eq, Hash, Debug)]
pub struct Symbol {
    pub text: String,
}

impl Symbol {
    pub fn new(text: &String) -> Symbol {
        Symbol { text: (*text).clone() }
    }

    pub fn is_end_symbol(&self) -> bool {
        let sym_char_vec: Vec<char> = self.text.chars().collect();
        sym_char_vec[0].is_lowercase()
    }

    pub fn is_not_end_symbol(&self) -> bool {
        !self.is_end_symbol()
    }
}

impl PartialEq<Symbol> for Symbol {
    fn eq(&self, sym: &Symbol) -> bool {
        self.text.eq(&sym.text)
    }
}

impl From<String> for Symbol {
    fn from(text: String) -> Symbol {
        Symbol::new(&text)
    }
}

impl From<&str> for Symbol {
    fn from(text: &str) -> Symbol {
        Symbol::new(text.to_string().borrow())
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Symbol: {}", self.text)
    }
}

#[derive(Clone, Eq, Hash, Debug)]
pub struct Production {
    pub head: Symbol,
    pub vec: Vec<Symbol>,
}

impl Production {
    pub fn new_by_vec(head_symbol: Symbol, vec: &Vec<Symbol>) -> Production {
        Production { head: head_symbol, vec: vec.clone() }
    }

    pub fn is_contain_end_symbol(&self) -> bool {
        for sym in self.vec.iter() {
            if sym.is_end_symbol() {
                return true;
            }
        }
        false
    }

    pub fn is_not_contain_end_symbol(&self) -> bool {
        for sym in self.vec.iter() {
            if sym.is_end_symbol() {
                return false;
            }
        }
        true
    }

    pub fn is_empty_production(&self, end_symbol: &Symbol) -> bool {
        self.vec.len() == 1 && self.vec[0].eq(end_symbol)
    }

}

impl PartialEq<Production> for Production {
    fn eq(&self, sym_list: &Production) -> bool {
        self.vec.eq(&sym_list.vec)
    }
}