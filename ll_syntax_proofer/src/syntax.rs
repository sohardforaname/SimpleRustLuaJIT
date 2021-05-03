use std::collections::{HashMap, HashSet};
use crate::symbol::{SymbolList, Symbol};
use std::ops::Not;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Syntax {
    pub symbols: HashSet<Symbol>,
    pub generators: HashMap<Symbol, HashSet<SymbolList>>,
    empty_status_map: HashMap<Symbol, bool>,
    first_set_map: HashMap<Symbol, HashSet<Symbol>>,
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

    fn get_deleting_generator_group<F>(&self, f: F) -> SymbolList
        where F: Fn(&SymbolList) -> bool {
        let mut sym_list = SymbolList::new();

        //TODO:f(generator) closure replace with f
        self.generators.iter().for_each(|sym| {
            if sym.1.iter().take_while(|generator| { f(generator).not() })
                .count() < sym.1.len() {
                sym_list.vec.push(sym.0.clone());
            }
        });
        sym_list
    }

    fn get_deleting_generator<F>(&self, f: F) -> HashMap<Symbol, HashSet<SymbolList>>
        where F: Fn(&SymbolList) -> bool {
        let mut generator_map = HashMap::<Symbol, HashSet<SymbolList>>::new();

        //TODO:f(generator) closure replace with f
        self.generators.iter().for_each(|sym| {
            sym.1.iter().filter(|generator| { f(generator) })
                .for_each(|generator| {
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

    fn delete_generator(&mut self, sym: (&Symbol, &HashSet<SymbolList>)) {
        sym.1.iter().for_each(|generator| {
            let set = self.generators.get_mut(sym.0).unwrap();
            set.remove(generator);
            if set.len() == 0 {
                self.empty_status_map.insert(sym.0.clone(), false);
                self.generators.remove(sym.0);
            }
        });
    }

    pub fn calc_empty_set(&mut self) {
        let copied_generator = self.generators.clone();

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

        self.generators = copied_generator;
    }

    pub fn calc_first_select(&mut self) {
        for dat in self.empty_status_map.iter() {
            if *dat.1 {
                self.first_set_map.get_mut(dat.0).unwrap().insert(
                    Symbol::from("z")
                );
            }
        }

        for sym in self.generators.iter() {
            for sym_vec in sym.1.iter() {
                if sym_vec.vec[0].is_end_symbol() {
                    self.first_set_map.get_mut(sym.0).unwrap()
                        .insert(sym_vec.vec[0].clone());
                }
            }
        }


        let mut is_modified = true;
        while is_modified {
            is_modified = false;
            for sym in self.generators.iter() {
                let mut new_first_set = HashSet::<Symbol>::new();
                let mut count = 0;
                for sym_vec in sym.1.iter() {
                    for cur_sym in sym_vec.vec.iter() {
                        if cur_sym.is_not_end_symbol() {
                            new_first_set = new_first_set.union(
                                self.first_set_map.get(cur_sym).unwrap()
                            ).cloned().collect();
                            if !self.empty_status_map.get(cur_sym).unwrap() {
                                break;
                            }
                            count += 1;
                            continue;
                        }
                        break;
                    }
                }
                if count < sym.1.len() {
                    new_first_set.remove(&Symbol::from("z"));
                }
                let mut ori_first_set = self.first_set_map.get_mut(sym.0).unwrap();
                if ori_first_set.len() < new_first_set.len() {
                    is_modified = true;
                    *ori_first_set = new_first_set;
                }
            }
        }
    }
}