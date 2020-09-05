use std::collections::HashMap;

pub struct SymbolTable {
    contents: HashMap
}

pub fn new() -> SymbolTable {
    SymbolTable{
        contents: HashMap::new();
    }
}

impl SymbolTable {
    fn add_entry(&self, symbol: String, address: usize) {
        self.contents.insert(symbol, address);
    }
    
    fn contains(&self, symbol: String) bool {
        self.contents.contains_key(symbol);
    }
    
    pub fn get_address(&self, symbol: String) {
        self.contents[symbol]
    }
}



