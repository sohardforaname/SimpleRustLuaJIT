use std::collections::{HashMap, HashSet};
use crate::symbol::{Production, Symbol};
use crate::util::calc_set_map_len;
use std::process::exit;

#[derive(Clone)]
pub struct Syntax {
    pub symbols: HashSet<Symbol>,
    pub generators: HashMap<Symbol, HashSet<Production>>,
    pub nullable_set: HashSet<Symbol>,
    pub first_set_map: HashMap<Symbol, HashSet<Symbol>>,
    pub follow_set_map: HashMap<Symbol, HashSet<Symbol>>,
    pub select_set_map: HashMap<Production, HashSet<Symbol>>,
    pub eof_symbol: Symbol,
    pub is_generated: bool,
}

impl Syntax {
    pub fn new(
        symbols: &HashSet<Symbol>,
        generators: &HashMap<Symbol, HashSet<Production>>,
    ) -> Syntax {
        let mut extend_symbols = symbols.clone();
        let init_eof_symbol = Symbol::from("eof");
        extend_symbols.insert(init_eof_symbol.clone());
        Syntax {
            symbols: extend_symbols.clone(),
            generators: generators.clone(),
            nullable_set: HashSet::new(),
            first_set_map: {
                let mut init_first_set_map = HashMap::new();
                for symbol in extend_symbols.iter() {
                    init_first_set_map.insert(symbol.clone(), HashSet::new());
                }
                init_first_set_map
            },
            follow_set_map: {
                let mut init_follow_set_map = HashMap::new();
                for symbol in symbols.iter() {
                    if symbol.is_not_end_symbol() {
                        init_follow_set_map.insert(symbol.clone(), HashSet::new());
                    }
                }
                init_follow_set_map
            },
            select_set_map: {
                let mut init_select_set_map = HashMap::new();
                for productions in generators.iter() {
                    for production in productions.1.iter() {
                        init_select_set_map.insert(production.clone(), HashSet::new());
                    }
                }
                init_select_set_map
            },
            eof_symbol: init_eof_symbol,
            is_generated: false,
        }
    }
}

impl Syntax {
    fn calc_empty_set(&mut self) {
        self.nullable_set.insert(Symbol::empty_symbol());
        loop {
            let size = self.nullable_set.len();
            for productions in self.generators.iter() {
                for production in productions.1.iter() {
                    if production.is_empty_production() {
                        self.nullable_set.insert(productions.0.clone());
                    } else {
                        let mut all_is_empty = true;
                        for symbol in production.vec.iter() {
                            if !self.nullable_set.contains(symbol) {
                                all_is_empty = false;
                                break;
                            }
                        }
                        if all_is_empty {
                            self.nullable_set.insert(productions.0.clone());
                        }
                    }
                }
            }
            if size == self.nullable_set.len() {
                break;
            }
        }
    }
}

impl Syntax {
    fn calc_first_set(&mut self) {
        for symbol in self.symbols.iter() {
            if symbol.is_end_symbol() || symbol.is_empty_symbol() {
                self.first_set_map.get_mut(symbol).unwrap().insert(symbol.clone());
            }
        }
        loop {
            let size = calc_set_map_len(&self.first_set_map);
            for productions in self.generators.iter() {
                for production in productions.1.iter() {
                    let mut first_set = self.first_set_map.get(productions.0).unwrap().clone();
                    for symbol in production.vec.iter() {
                        if symbol.is_end_symbol() {
                            first_set.insert(symbol.clone());
                            break;
                        }
                        first_set = first_set.union(self.first_set_map.get(symbol).unwrap())
                            .cloned().collect::<HashSet<Symbol>>();
                        if !self.nullable_set.contains(symbol) {
                            break;
                        }
                    }
                    let old_first_set = self.first_set_map.get_mut(productions.0).unwrap();
                    *old_first_set = first_set;
                }
            }
            if size == calc_set_map_len(&self.first_set_map) {
                break;
            }
        }
    }
}

impl Syntax {
    fn calc_follow_set(&mut self) {
        self.follow_set_map.get_mut(&Symbol::from("S")).unwrap()
            .insert(self.eof_symbol.clone());
        loop {
            let size = calc_set_map_len(&self.follow_set_map);
            for productions in self.generators.iter() {
                for production in productions.1.iter() {
                    let mut suffix_follow_set = self.follow_set_map.get(productions.0)
                        .unwrap().clone();
                    for symbol in production.vec.iter().rev() {
                        if symbol.is_end_symbol() {
                            suffix_follow_set.clear();
                            suffix_follow_set.insert(symbol.clone());
                            continue;
                        } else if symbol.is_not_end_symbol() {
                            let cur_follow_set = self.follow_set_map.get_mut(symbol)
                                .unwrap();
                            *cur_follow_set = cur_follow_set.union(&suffix_follow_set).cloned().collect();
                            let cur_first_set = self.first_set_map.get(symbol).unwrap();
                            if !self.nullable_set.contains(symbol) {
                                suffix_follow_set = cur_first_set.clone();
                            } else {
                                suffix_follow_set = suffix_follow_set.union(cur_first_set).cloned().collect();
                            }
                        }
                    }
                }
            }
            if size == calc_set_map_len(&self.follow_set_map) {
                break;
            }
        }
        for set in self.follow_set_map.iter_mut() {
            set.1.remove(&Symbol::empty_symbol());
        }
    }
}

impl Syntax {
    fn calc_select_set(&mut self) {
        for productions in self.generators.iter() {
            for production in productions.1.iter() {
                let select_set = self.select_set_map.get_mut(production)
                    .unwrap();
                let mut is_all_empty_deduced = true;
                for symbol in production.vec.iter() {
                    if symbol.is_end_symbol() {
                        select_set.insert(symbol.clone());
                        is_all_empty_deduced = false;
                        break;
                    }
                    *select_set = select_set.union(self.first_set_map.get(symbol).unwrap()).cloned().collect();
                    if !self.nullable_set.contains(symbol) {
                        is_all_empty_deduced = false;
                        break;
                    }
                }
                if is_all_empty_deduced {
                    *select_set = select_set.union(self.follow_set_map.get(&production.head).unwrap())
                        .cloned().collect();
                }
                select_set.remove(&Symbol::empty_symbol());
            }
        }
    }
}

impl Syntax {
    pub fn generate_sets(&mut self) {
        self.calc_empty_set();
        self.calc_first_set();
        self.calc_follow_set();
        self.calc_select_set();
    }

    pub fn check_if_ll(&mut self) -> bool {
        if !self.is_generated {
            self.generate_sets();
        }
        self.verbose();

        for productions in self.generators.iter() {
            let mut intersection = self.symbols.clone();
            for production in productions.1.iter() {
                intersection = intersection.intersection(self.select_set_map.get(production).unwrap())
                    .cloned().collect();
            }
            if intersection.len() > 0 {
                return false;
            }
        }
        true
    }

    pub fn build_analyze_table(&mut self) -> HashMap<(Symbol, Symbol), Production> {
        if !self.is_generated {
            self.generate_sets();
        }

        let mut trans_map = HashMap::new();

        for select_item in self.select_set_map.iter() {
            let head_sym = &select_item.0.head;
            for symbol in select_item.1.iter() {
                trans_map.insert((head_sym.clone(), symbol.clone()), select_item.0.clone());
            }
        }
        trans_map
    }
}

impl Syntax {
    fn verbose(&self) {
        for sym in self.first_set_map.iter() {
            print!("{} First: ", sym.0);
            for sym1 in sym.1.iter() {
                print!("{} ", sym1);
            }
            print!("\n");
        }

        for sym in self.follow_set_map.iter() {
            print!("{} Follow: ", sym.0);
            for sym1 in sym.1.iter() {
                print!("{} ", sym1);
            }
            print!("\n");
        }

        for sym in self.select_set_map.iter() {
            print!("Production {} -> : ", sym.0.head);
            for sym1 in sym.0.vec.iter() {
                print!("{} ", sym1);
            }
            print!("\n");
            for sym1 in sym.1.iter() {
                print!("{} ", sym1);
            }
            print!("\n");
        }
    }
}