use std::collections::{HashMap, HashSet};
use crate::symbol::{SymbolList, Symbol};
use std::ops::Not;
use std::fmt::{Display, Formatter, Result};

pub struct Syntax {
    pub generators: HashMap<Symbol, HashSet<SymbolList>>,
    pub empty_status_map: HashMap<Symbol, bool>,
}

impl Syntax {
    pub fn new(generators: &HashMap<Symbol, HashSet<SymbolList>>) -> Syntax {
        Syntax {
            generators: generators.clone(),
            empty_status_map: HashMap::new(),
        }
    }

    fn get_deleting_generator_group<F>(&self, f: F) -> SymbolList
        where F: Fn(&SymbolList) -> bool {
        let mut sym_list = SymbolList::new();
        //TODO:f(generator) closure replace with f
        self.generators.iter().for_each(|sym| {
            if sym.1.iter().take_while(|generator| {
                f(generator).not()
            }).count() < sym.1.len() {
                sym_list.vec.push(sym.0.clone());
            }
        });
        sym_list
    }

    fn get_deleting_generator<F>(&self, f: F) -> HashMap<Symbol, HashSet<SymbolList>>
        where F: FnMut(&&SymbolList) -> bool {
        let mut generator_map = HashMap::<Symbol, HashSet<SymbolList>>::new();

        //TODO:f(generator) closure replace with f
        self.generators.iter().for_each(|sym| {
            sym.1.iter().filter(|generator| {
                f(generator)
            }).for_each(|generator| {
                if let Some(set) = generator_map.get_mut(sym.0) {
                    set.insert(generator.clone());
                } else {
                    let mut generator_set = HashSet::new();
                    generator_set.insert(generator.clone());
                    generator_map.insert(sym.0.clone(), generator_set);
                }
            });
        });
        generator_map
    }

    fn delete_generator_group(&mut self, sym: &Symbol) {
        self.empty_status_map.insert(sym.clone(), true);
        self.generators.remove(sym);
    }

    fn delete_generator(&mut self, sym:(&Symbol, &HashSet<SymbolList>)) {
        sym.1.iter().for_each(|generator| {
            let set = self.generators.get_mut(sym.0).unwrap();
            set.remove(generator);
            if set.len() == 0 {
                self.empty_status_map.insert(sym.0.clone(), false);
                self.generators.remove(sym.0);
            }
        });
    }

    pub fn calc_empty(&mut self) {
        self.get_deleting_generator_group(|generator: &SymbolList| {
            generator.is_empty_str_symbol_list()
        }).vec.iter().for_each(|sym| {
            self.delete_generator_group(sym);
        });

        self.get_deleting_generator(|generator: &SymbolList| {
            generator.is_contain_end_symbol()
        }).iter().for_each(|sym| {
            self.delete_generator(sym);
        });

        let mut len = -1i32;
        while self.empty_status_map.len() as i32 > len {
            len = self.empty_status_map.len() as i32;
            self.get_deleting_generator_group(|generator: &SymbolList| {
                generator.is_all_true_not_end_symbol(&self.empty_status_map)
            }).vec.iter().for_each(|sym| {
                self.delete_generator_group(sym);
            });

            self.get_deleting_generator(|generator: &SymbolList| {
                generator.is_contain_false_not_end_symbol(&self.empty_status_map)
            }).iter().for_each(|sym| {
                self.delete_generator(sym);
            });
        }
    }
}