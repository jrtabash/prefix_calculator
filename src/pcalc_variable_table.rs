use crate::pcalc_value::{Value, ValueError, ValueResult};
use std::collections::HashMap;
use std::fmt;

pub struct VariableTable {
    table: HashMap<String, Value>
}

impl VariableTable {
    pub fn new() -> Self {
        VariableTable { table: HashMap::new() }
    }

    pub fn get(&self, name: &str) -> ValueResult {
        if let Some(value) = self.table.get(name) {
            Ok(*value)
        } else {
            Err(ValueError::new(&format!("Unknown variable '{}'", name)))
        }
    }

    pub fn def(&mut self, name: &str, value: Value) -> ValueResult {
        if !self.table.contains_key(name) {
            self.table.insert(String::from(name), value);
            Ok(value)
        } else {
            Err(ValueError::new(&format!("Duplicate variable definition '{}'", name)))
        }
    }

    pub fn set(&mut self, name: &str, value: Value) -> ValueResult {
        if let Some(val) = self.table.get_mut(name) {
            *val = value;
            Ok(value)
        } else {
            Err(ValueError::new(&format!("Unknown variable '{}'", name)))
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.table.clear();
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.table.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    pub fn show(&self) {
        let width = self.table.iter().map(|kv| kv.0.len()).max().unwrap_or(0);
        Self::prt_name_value(width, "var", "value");
        Self::prt_name_value(width, "---", "-----");
        for (name, value) in &self.table {
            Self::prt_name_value(width, name, value);
        }
    }

    fn prt_name_value<Value: fmt::Display + ?Sized>(width: usize, name: &str, value: &Value) {
        println!("{name:<width$}   {value}", name = name, width = width, value = value);
    }
}

impl Default for VariableTable {
    fn default() -> Self {
        Self::new()
    }
}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_table() {
        let mut vtab = VariableTable::new();
        assert_eq!(vtab.len(), 0);
        assert!(vtab.is_empty());

        vtab.def("x", Value::from_num(10.0)).unwrap();
        assert_eq!(vtab.get("x").unwrap(), Value::from_num(10.0));
        assert_eq!(vtab.len(), 1);
        assert!(!vtab.is_empty());

        vtab.set("x", Value::from_num(15.0)).unwrap();
        assert_eq!(vtab.get("x").unwrap(), Value::from_num(15.0));
        assert_eq!(vtab.len(), 1);
        assert!(!vtab.is_empty());

        assert!(vtab.get("y").is_err());

        vtab.reset();
        assert_eq!(vtab.len(), 0);
        assert!(vtab.is_empty());
    }
}
