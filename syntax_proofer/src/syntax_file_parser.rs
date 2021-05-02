use crate::symbol::{Symbol, SymBolList};
use std::collections::{HashMap, HashSet};

pub struct SyntaxParser {
    pub end_symbol: SymBolList,
    pub not_end_symbol: SymBolList,
    pub generators: HashMap<Symbol, HashSet<SymBolList>>,
}

impl SyntaxParser {
    pub fn new() -> SyntaxParser {
        SyntaxParser {
            end_symbol: SymBolList::new(),
            not_end_symbol: SymBolList::new(),
            generators: HashMap::new(),
        }
    }

    pub fn add_syntax_line(&mut self, line: &mut String) {
        let vec = line.split("->")
            .map(|str| {
                str.trim()
            }).collect::<Vec<&str>>();

        let mut sym_vec: Vec<Symbol> = vec![];
        let sym = Symbol::new(&vec[0].to_string());
        for sym in vec[1].split_ascii_whitespace() {
            sym_vec.push(Symbol::new(&sym.to_string()));
        }

    }
}