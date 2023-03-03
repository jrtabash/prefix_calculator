use crate::pcalc_function::*;
use std::collections::HashMap;

pub struct FunctionTable {
    funcs: HashMap<String, FunctionPtr>
}

impl FunctionTable {
    pub fn new() -> Self {
        FunctionTable { funcs: HashMap::new() }
    }

    pub fn get(&self, name: &str) -> FunctionResult {
        if let Some(func) = self.funcs.get(name) {
            Ok(func)
        } else {
            Err(FunctionError::new(&format!("Unknown function '{}'", name)))
        }
    }

    pub fn def(&mut self, name: &str, func: &FunctionPtr) {
        if let Some(f) = self.funcs.get_mut(name) {
            *f = FunctionPtr::clone(func);
        } else {
            self.funcs.insert(name.to_string(), FunctionPtr::clone(func));
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.funcs.clear();
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.funcs.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.funcs.is_empty()
    }

    pub fn show(&self) {
        let width = self.funcs.iter().map(|kv| kv.0.len()).max().unwrap_or(0);

        let prt_row = |name: &str, value: &str| {
            println!("{name:<width$}   {value}", name = name, width = width, value = value);
        };

        prt_row("Func", "Params");
        prt_row("----", "------");
        for (name, func) in &self.funcs {
            prt_row(name, &format!("({})", func.parameters().join(", ")));
        }
    }
}

impl Default for FunctionTable {
    fn default() -> Self {
        Self::new()
    }
}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcalc_code::Literal;
    use crate::pcalc_environment::Environment;
    use crate::pcalc_value::Value;

    #[test]
    fn test_function_table() {
        let mut ftab = FunctionTable::new();
        assert!(ftab.is_empty());
        assert_eq!(ftab.len(), 0);

        let fname = "foo";

        ftab.def(fname, &FunctionPtr::new(Function::new(Parameters::new(), Expressions::new())));
        assert!(ftab.get(fname).is_ok());
        assert!(!ftab.is_empty());
        assert_eq!(ftab.len(), 1);

        ftab.def(fname, &FunctionPtr::new(Function::new(Parameters::new(), Expressions::new())));
        assert!(ftab.get(fname).is_ok());
        assert!(!ftab.is_empty());
        assert_eq!(ftab.len(), 1);

        assert!(ftab.get("bar").is_err());

        ftab.reset();
        assert!(ftab.is_empty());
        assert_eq!(ftab.len(), 0);
    }

    #[test]
    fn test_environment_eval_function() {
        let mut env = Environment::new();
        let mut ftab = FunctionTable::new();

        let params = Parameters::new();
        let mut exprs = Expressions::new();
        exprs.push(Box::new(Literal::new(Value::from_num(5.0))));

        ftab.def("f", &FunctionPtr::new(Function::new(params, exprs)));

        let func = ftab.get("f").unwrap();
        assert_eq!(func.eval(&mut env, &Arguments::new()).unwrap(), Value::from_num(5.0));
    }
}
