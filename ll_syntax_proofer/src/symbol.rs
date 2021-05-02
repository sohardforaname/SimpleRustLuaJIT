use std::collections::HashMap;
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
pub struct SymbolList {
    pub vec: Vec<Symbol>,
}

impl SymbolList {
    pub fn new() -> SymbolList {
        SymbolList { vec: Vec::new() }
    }

    pub fn new_by_vec(vec: &Vec<Symbol>) -> SymbolList {
        SymbolList { vec: vec.clone() }
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

    pub fn is_empty_str_symbol_list(&self) -> bool {
        self.vec.len() == 1 && self.vec[0].eq(&Symbol::new(&"z".to_string()))
    }

    pub fn is_all_true_not_end_symbol(&self, status_map: &HashMap<Symbol, bool>) -> bool {
        for sym in self.vec.iter() {
            if sym.is_not_end_symbol() && if let Some(status) = status_map.get(sym) {
                !(*status)
            } else {
                true
            } {
                return false;
            }
        }
        true
    }

    pub fn is_contain_false_not_end_symbol(&self, status_map: &HashMap<Symbol, bool>) -> bool {
        for sym in self.vec.iter() {
            if sym.is_not_end_symbol() && if let Some(status) = status_map.get(sym) {
                *status
            } else {
                true
            } {
                return true;
            }
        }
        false
    }
}

impl PartialEq<SymbolList> for SymbolList {
    fn eq(&self, sym_list: &SymbolList) -> bool {
        self.vec.eq(&sym_list.vec)
    }
}