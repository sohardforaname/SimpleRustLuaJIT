use std::collections::{HashMap, HashSet};
use crate::symbol::{SymBolList, Symbol};
use std::ops::Not;
use std::io::empty;

pub struct Syntax {
    pub end_symbol: SymBolList,
    pub not_end_symbol: SymBolList,
    pub generators: HashMap<Symbol, HashSet<SymBolList>>,
    empty_status_map: HashMap<Symbol, bool>,
}

impl Syntax {
    fn new(end_sym: &SymBolList, not_end_sym: &SymBolList, generators: &HashMap<Symbol, HashSet<SymBolList>>) -> Syntax {
        Syntax {
            end_symbol: end_sym.clone(),
            not_end_symbol: not_end_sym.clone(),
            generators: generators.clone(),
            empty_status_map: HashMap::new(),
        }
    }

    fn get_deleting_generator_group<F>(&self, f: F) -> SymBolList
        where F: Fn(&SymBolList) -> bool {
        let mut sym_list = SymBolList::new();
        for sym in self.generators.iter() {
            for generator in sym.1.iter() {
                if f(generator) {
                    sym_list.vec.push(sym.0.clone());
                    break;
                }
            }
        }

        sym_list
    }

    fn get_deleting_generator<F>(&self, f: F) -> HashMap<Symbol, HashSet<SymBolList>>
        where F: Fn(&SymBolList) -> bool {
        let mut generator_map = HashMap::<Symbol, HashSet<SymBolList>>::new();
        for sym in self.generators.iter() {
            for generator in sym.1.iter() {
                if f(generator) {
                    match generator_map.get_mut(sym.0) {
                        Some(set) => { set.insert(generator.clone()); }
                        None => {
                            let mut generator_set = HashSet::new();
                            generator_set.insert(generator.clone());
                            generator_map.insert(sym.0.clone(), generator_set);
                        }
                    }
                }
            }
        }
        generator_map
    }

    fn get_deleting_generator_group1<F>(&self, f: F) -> SymBolList
        where F: Fn(&SymBolList) -> bool {
        let mut sym_list = SymBolList::new();
        self.generators.iter().map(|sym| {
            if sym.1.iter().take_while(|generator| {
                f(generator).not()
            }).count() < sym.1.len() {
                sym_list.vec.push(sym.0.clone());
            }
        });
        sym_list
    }

    fn get_deleting_generator1<F>(&self, f: F) -> HashMap<Symbol, HashSet<SymBolList>>
        where F: Fn(&SymBolList) -> bool {
        let mut generator_map = HashMap::<Symbol, HashSet<SymBolList>>::new();

        self.generators.iter().map(|sym| {
            sym.1.iter().filter(|generator| {
                f(generator)
            }).map(|generator| {
                match generator_map.get_mut(sym.0) {
                    Some(set) => {
                        set.insert(generator.clone());
                    }
                    None => {
                        let mut generator_set = HashSet::new();
                        generator_set.insert(generator.clone());
                        generator_map.insert(sym.0.clone(), generator_set);
                    }
                }
            });
        });

        generator_map
    }

    fn calc_empty(&mut self) {
        self.get_deleting_generator_group(|generator: &SymBolList| { generator.is_empty_str_symbol_list() })
            .vec.iter().map(|sym| {
            self.empty_status_map.insert(sym.clone(), true);
        });

        self.get_deleting_generator(|generator: &SymBolList| { generator.is_contain_end_symbol() })
            .iter().map(|sym| {
            sym.1.iter().map(|generator|{
                self.generators.get_mut(sym.0).unwrap().remove(generator);
            });
        });

        while self.empty_status_map.len() < self.not_end_symbol.vec.len() {
            self.get_deleting_generator_group(|generator: &SymBolList| { generator.is_all_true_not_end_symbol(&self.empty_status_map) })
                .vec.iter().map(|sym| {
                self.empty_status_map.insert(sym.clone(), true);
            });

            self.get_deleting_generator(|generator: &SymBolList| { generator.is_contain_false_not_end_symbol(&self.empty_status_map) })
                .iter().map(|sym| {
                sym.1.iter().map(|generator|{
                    self.generators.get_mut(sym.0).unwrap().remove(generator);
                });
            });
        }
    }
}