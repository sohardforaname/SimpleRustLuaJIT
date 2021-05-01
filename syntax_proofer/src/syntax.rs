use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::symbol::{SymBolList, Symbol};
use std::borrow::BorrowMut;

struct Syntax {
    pub end_symbol: SymBolList,
    pub not_end_symbol: SymBolList,
    pub generators: HashMap<Symbol, HashSet<SymBolList>>,
}

impl Syntax {
    fn new(end_sym: &SymBolList, not_end_sym: &SymBolList, generators: &HashMap<Symbol, HashSet<SymBolList>>) -> Syntax {
        Syntax {
            end_symbol: end_sym.clone(),
            not_end_symbol: not_end_sym.clone(),
            generators: generators.clone(),
        }
    }

    fn delete_generator_group<F>(&mut self, status_map: &mut HashMap<Symbol, bool>, f: F)
        where F: FnMut(&SymBolList) -> bool {
        for mut sym in self.generators.iter() {
            for generator in sym.1.iter() {
                if f(generator) {
                    self.generators.remove(sym.0);
                    status_map.insert(sym.0.clone(), true);
                    break;
                }
            }
        }
    }

    fn delete_generator<F>(&mut self, status_map: &mut HashMap<Symbol, bool>, f: F)
        where F: FnMut(&SymBolList, &HashMap<Symbol, bool>) -> bool {
        for mut sym in self.generators.iter() {
            for generator in sym.1.iter() {
                if f(generator, status_map) {
                    sym.1.remove(generator);
                    if sym.1.len() == 0 {
                        status_map.insert(sym.0.clone(), false);
                    }
                }
            }
        }
    }

    pub fn calc_empty(&mut self) -> HashMap<Symbol, bool> {
        let mut status_map: HashMap<Symbol, bool> = HashMap::new();

        self.delete_generator_group(&mut status_map,
                                    |generator| {
                                        generator.is_empty_str_symbol_list()
                                    });
        self.delete_generator(&mut status_map,
                              |generator| {
                                  generator.is_contain_end_symbol()
                              });

        while status_map.len() < self.not_end_symbol.vec.len() {
            self.delete_generator_group(&mut status_map,
                                        |generator, status_map| {
                                            generator.is_all_true_not_end_symbol(status_map)
                                        });
            self.delete_generator(&mut status_map,
                                  |generator, status_map| {
                                      generator.is_contain_false_not_end_symbol(status_map)
                                  });
        }
        status_map
    }
}