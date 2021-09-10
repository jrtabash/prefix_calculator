use std::collections::HashMap;
use crate::pcalc_value::{Value, ValueResult, ValueError};

pub struct Environment {
    table: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> ValueResult {
        if let Some(value) = self.table.get(name) {
            return Ok(*value)
        }
        else {
            Err(ValueError::new(&format!("Unknown variable '{}'", name)))
        }
    }

    pub fn def(&mut self, name: &str, value: Value) -> ValueResult {
        if !self.table.contains_key(name) {
            self.table.insert(String::from(name), value);
            Ok(value)
        }
        else {
            Err(ValueError::new(&format!("Duplicate variable definition '{}'", name)))
        }
    }

    pub fn set(&mut self, name: &str, value: Value) -> ValueResult {
        if let Some(val) = self.table.get_mut(name) {
            *val = value;
            return Ok(value)
        }
        else {
            Err(ValueError::new(&format!("Unknown variable '{}'", name)))
        }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment() {
        let mut env = Environment::new();

        env.def("x", Value::from_num(10.0)).unwrap();
        assert_eq!(env.get("x").unwrap(), Value::from_num(10.0));

        env.set("x", Value::from_num(15.0)).unwrap();
        assert_eq!(env.get("x").unwrap(), Value::from_num(15.0));

        assert!(env.get("y").is_err());
    }
}
