use crate::pcalc_function::{FunctionPtr, FunctionResult};
use crate::pcalc_function_table::{FunctionTable, FunctionTablePtr};
use crate::pcalc_value::{Value, ValueResult};
use crate::pcalc_variable_table::VariableTable;

pub struct Environment {
    vars: VariableTable,
    funcs: FunctionTablePtr
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            vars: VariableTable::new(),
            funcs: FunctionTablePtr::new(FunctionTable::new())
        }
    }

    pub fn with_parent_funcs(parent: &Environment) -> Self {
        Environment {
            vars: VariableTable::new(),
            funcs: FunctionTablePtr::clone(&parent.funcs)
        }
    }

    #[inline(always)]
    pub fn get_var(&self, name: &str) -> ValueResult {
        self.vars.get(name)
    }

    #[inline(always)]
    pub fn def_var(&mut self, name: &str, value: Value) -> ValueResult {
        self.vars.def(name, value)
    }

    #[inline(always)]
    pub fn set_var(&mut self, name: &str, value: Value) -> ValueResult {
        self.vars.set(name, value)
    }

    #[inline(always)]
    pub fn get_func(&self, name: &str) -> FunctionResult {
        self.funcs.get(name)
    }

    #[inline(always)]
    pub fn def_func(&mut self, name: &str, func: &FunctionPtr) {
        FunctionTablePtr::get_mut(&mut self.funcs).expect("Missing funcs table").def(name, func);
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.vars.reset();
        FunctionTablePtr::get_mut(&mut self.funcs).expect("Missing funcs table").reset();
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.vars.len() + self.funcs.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty() && self.funcs.is_empty()
    }

    pub fn show(&self) {
        let pvars: bool = !self.vars.is_empty();
        let pfuns: bool = !self.funcs.is_empty();
        let newln: bool = pvars && pfuns;
        if pvars {
            self.vars.show();
        }
        if newln {
            println!();
        }
        if pfuns {
            self.funcs.show();
        }
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
    use crate::pcalc_code::Literal;
    use crate::pcalc_function::*;

    #[test]
    fn test_environment_variables() {
        let mut env = Environment::new();
        assert_eq!(env.len(), 0);
        assert!(env.is_empty());

        env.def_var("x", Value::from_num(10.0)).unwrap();
        assert_eq!(env.get_var("x").unwrap(), Value::from_num(10.0));
        assert_eq!(env.len(), 1);
        assert!(!env.is_empty());

        env.set_var("x", Value::from_num(15.0)).unwrap();
        assert_eq!(env.get_var("x").unwrap(), Value::from_num(15.0));
        assert_eq!(env.len(), 1);
        assert!(!env.is_empty());

        assert!(env.get_var("y").is_err());

        env.reset();
        assert_eq!(env.len(), 0);
        assert!(env.is_empty());
    }

    #[test]
    fn test_environment_functions() {
        let mut env = Environment::new();
        assert!(env.is_empty());
        assert_eq!(env.len(), 0);

        let fname = "foo";

        env.def_func(fname, &FunctionPtr::new(Function::new(Parameters::new(), Expressions::new())));
        assert!(env.get_func(fname).is_ok());
        assert!(!env.is_empty());
        assert_eq!(env.len(), 1);

        env.def_func(fname, &FunctionPtr::new(Function::new(Parameters::new(), Expressions::new())));
        assert!(env.get_func(fname).is_ok());
        assert!(!env.is_empty());
        assert_eq!(env.len(), 1);

        assert!(env.get_func("bar").is_err());

        env.reset();
        assert!(env.is_empty());
        assert_eq!(env.len(), 0);
    }

    #[test]
    fn test_environment_eval_function() {
        let mut env = Environment::new();

        let params = Parameters::new();
        let mut exprs = Expressions::new();
        exprs.push(Box::new(Literal::new(Value::from_num(5.0))));

        env.def_func("f", &FunctionPtr::new(Function::new(params, exprs)));

        let func = FunctionPtr::clone(env.get_func("f").unwrap());
        assert_eq!(func.eval(&mut env, &Arguments::new()).unwrap(), Value::from_num(5.0));
    }
}
