use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::symbol::{SymBolList, Symbol};

struct Syntax {
    pub end_symbol: SymBolList,
    pub not_end_symbol: SymBolList,
    pub generators: HashMap<Symbol, HashSet<SymBolList>>,
}

impl Syntax {
    fn new(end_sym: &SymBolList, not_end_sym: &SymBolList, generators: &HashMap<Symbol, HashSet<SymBolList>>) -> Syntax {
        Syntax {
            end_symbol: (*end_sym).clone(),
            not_end_symbol: (*not_end_sym).clone(),
            generators: (*generators).clone(),
        }
    }

    fn calc_empty(&mut self) -> HashMap<Symbol, bool> {
        let status_map: HashMap<Symbol, bool> = HashMap::new();
        for sym in self.generators.iter() {
            for generator in sym.1.iter() {
                if generator.is_empty_str_symbol_list() {}
            }
        }
        status_map
    }
}