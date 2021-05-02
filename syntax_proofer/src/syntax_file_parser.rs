use crate::symbol::{Symbol, SymbolList};
use std::collections::{HashMap, HashSet};

pub struct SyntaxParser {
    pub end_symbol: HashSet<Symbol>,
    pub not_end_symbol: HashSet<Symbol>,
    pub generators: HashMap<Symbol, HashSet<SymbolList>>,
}

impl SyntaxParser {
    pub fn new() -> SyntaxParser {
        SyntaxParser {
            end_symbol: HashSet::new(),
            not_end_symbol: HashSet::new(),
            generators: HashMap::new(),
        }
    }

    pub fn add_syntax_line(&mut self, line: String) {
        let vec = line.split("->")
            .map(|str| {
                str.trim()
            }).collect::<Vec<&str>>();

        self.not_end_symbol.insert(Symbol::from(vec[0]));

        let mut sym_vec = Vec::new();
        vec[1].split_ascii_whitespace().into_iter().for_each(|str| {
            let sym = Symbol::new(&str.to_string());
            sym_vec.push(sym.clone());
            if sym.is_end_symbol() {
                self.end_symbol.insert(sym.clone());
            } else {
                self.not_end_symbol.insert(sym.clone());
            }
        });

        if let Some(gen_set) = self.generators.get_mut(&Symbol::from(vec[0])) {
            gen_set.insert(SymbolList::new_by_vec(&sym_vec));
        } else {
            self.generators.insert(Symbol::from(vec[0]), {
                let mut set = HashSet::<SymbolList>::new();
                set.insert(SymbolList::new_by_vec(&sym_vec));
                set
            });
        }
    }
}