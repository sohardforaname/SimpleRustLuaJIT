#[derive(Clone, Hash)]
pub struct Symbol {
    pub text: String,
}

impl Symbol {
    fn new(text: &String) -> Symbol {
        Symbol { text: (*text).clone() }
    }

    pub fn is_end_symbol(&self) -> bool {
        let sym_char_vec: Vec<char> = self.text.chars().collect();
        sym_char_vec[0].is_lowercase()
    }

    pub fn is_not_end_symbol(&self) -> bool {
        !self.is_end_symbol()
    }
}

impl PartialEq<Symbol> for Symbol {
    fn eq(&self, sym: &Symbol) -> bool {
        self.text.eq(&sym.text)
    }
}

#[derive(Clone, Hash)]
pub struct SymBolList {
    pub vec: Vec<Symbol>,
}

impl SymBolList {
    pub fn is_contain_end_symbol(&self) -> bool {
        for sym in self.vec.iter() {
            if sym.is_end_symbol() {
                return true;
            }
        }
        false
    }

    pub fn is_not_contain_end_symbol(&self) -> bool {
        for sym in self.vec.iter() {
            if sym.is_end_symbol() {
                return false;
            }
        }
        true
    }

    pub fn is_empty_str_symbol_list(&self) -> bool {
        self.vec.len() == 1 && self.vec[0].eq(&Symbol::new(&"e".to_string()))
    }
}

impl PartialEq<SymBolList> for SymBolList {
    fn eq(&self, sym_list: &SymBolList) -> bool {
        self.vec.eq(&sym_list.vec)
    }
}