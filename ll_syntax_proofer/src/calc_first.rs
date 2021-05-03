use crate::syntax::Syntax;
use std::collections::HashSet;
use crate::symbol::Symbol;

impl Syntax {
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