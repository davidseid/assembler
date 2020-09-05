use std::collections::HashMap;

pub struct SymbolTable {
    contents: HashMap<String, usize>,
}

pub fn new() -> SymbolTable {
    SymbolTable{
        contents: HashMap::new(),
    }
}

impl SymbolTable {
    pub fn add_entry(&mut self, symbol: String, address: usize) {
        self.contents.insert(symbol, address);
    }
    
    pub fn contains(&self, symbol: String) -> bool {
        self.contents.contains_key(&symbol)
    }
    
    pub fn get_address(&self, symbol: String) -> usize {
        self.contents[&symbol]
    }

    pub fn add_predefined(&mut self) {
        self.add_entry(String::from("SP"), 0);
        self.add_entry(String::from("LCL"), 1);
        self.add_entry(String::from("ARG"), 2);
        self.add_entry(String::from("THIS"), 3);
        self.add_entry(String::from("THAT"), 4);
        self.add_entry(String::from("SCREEN"), 16384);
        self.add_entry(String::from("KBD"), 24576);

        for i in 0..16 {
            let key = String::from(format!{"R{}", i});
            self.add_entry(key, i);
        }
    }
}




