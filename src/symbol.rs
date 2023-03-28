use std::collections::HashMap;

pub enum Symbol {
    LatchVar,
    InputVar,
    Define,
}

pub struct SymbolTable {
    table: HashMap<String, Symbol>,
}
