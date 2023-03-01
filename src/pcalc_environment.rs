use crate::pcalc_value::{Value, ValueError, ValueResult};
use std::collections::HashMap;
use std::fmt;

pub struct Environment {
    table: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Environment { table: HashMap::new() }
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

    pub fn reset(&mut self) {
        self.table.clear();
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    pub fn show(&self) {
        let width = self.table.iter().map(|kv| kv.0.len()).max().unwrap_or(0);
        Self::prt_name_value(width, "name", "value");
        Self::prt_name_value(width, "----", "-----");
        for (name, value) in &self.table {
            Self::prt_name_value(width, name, value);
        }
    }

    fn prt_name_value<Value: fmt::Display + ?Sized>(width: usize, name: &str, value: &Value) {
        println!("{name:<width$}   {value}", name = name, width = width, value = value);
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment() {
        let mut env = Environment::new();
        assert_eq!(env.len(), 0);
        assert!(env.is_empty());

        env.def("x", Value::from_num(10.0)).unwrap();
        assert_eq!(env.get("x").unwrap(), Value::from_num(10.0));
        assert_eq!(env.len(), 1);
        assert!(!env.is_empty());

        env.set("x", Value::from_num(15.0)).unwrap();
        assert_eq!(env.get("x").unwrap(), Value::from_num(15.0));
        assert_eq!(env.len(), 1);
        assert!(!env.is_empty());

        assert!(env.get("y").is_err());

        env.reset();
        assert_eq!(env.len(), 0);
        assert!(env.is_empty());
    }
}
