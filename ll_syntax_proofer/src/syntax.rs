use std::collections::{HashMap, HashSet};
use crate::symbol::{SymbolList, Symbol};
use std::ops::Not;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Syntax {
    pub symbols: HashSet<Symbol>,
    pub generators: HashMap<Symbol, HashSet<SymbolList>>,
    pub empty_status_map: HashMap<Symbol, bool>,
    pub first_set_map: HashMap<Symbol, HashSet<Symbol>>,
}

impl Syntax {
    pub fn new(symbols: &HashSet<Symbol>, generators: &HashMap<Symbol, HashSet<SymbolList>>) -> Syntax {
        let mut init_first_map = HashMap::<Symbol, HashSet<Symbol>>::new();
        symbols.iter().for_each(|sym| {

            //TODO: use into_iter to construct hash_set
            init_first_map.insert(sym.clone(), {
                if sym.is_end_symbol() {
                    let mut set = HashSet::<Symbol>::new();
                    set.insert(sym.clone());
                    set
                } else {
                    HashSet::new()
                }
            });
        });
        Syntax {
            symbols: symbols.clone(),
            generators: generators.clone(),
            empty_status_map: HashMap::new(),
            first_set_map: init_first_map,
        }
    }
}