use crate::symbol::{Symbol, Production};
use std::collections::{HashMap, HashSet};

pub struct SyntaxParser {
    pub symbols: HashSet<Symbol>,
    pub generators: HashMap<Symbol, HashSet<Production>>,
}

impl SyntaxParser {
    pub fn new() -> SyntaxParser {
        SyntaxParser {
            symbols: HashSet::new(),
            generators: HashMap::new(),
        }
    }

    pub fn add_syntax_line(&mut self, line: String) {
        let vec = line.split("=>")
            .map(|str| {
                str.trim()
            }).collect::<Vec<&str>>();

        if vec.len() != 2 {
            return;
        }

        self.symbols.insert(Symbol::from(vec[0]));

        let mut sym_vec = Vec::new();
        vec[1].split_ascii_whitespace().into_iter().for_each(|str| {
            let sym = Symbol::new(&str.to_string());
            sym_vec.push(sym.clone());
            self.symbols.insert(sym.clone());
        });

        if let Some(gen_set) = self.generators.get_mut(&Symbol::from(vec[0])) {
            gen_set.insert(Production::new_by_vec(Symbol::from(vec[0]), &sym_vec));
        } else {
            self.generators.insert(Symbol::from(vec[0]), {
                let mut set = HashSet::<Production>::new();
                set.insert(Production::new_by_vec(Symbol::from(vec[0]), &sym_vec));
                set
            });
        }
    }
}