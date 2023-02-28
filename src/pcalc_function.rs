use crate::pcalc_code::CodePtr;
use crate::pcalc_environment::Environment;
use crate::pcalc_value::{Value, ValueError, ValueResult};
use std::iter::zip;

pub type Parameters = Vec<String>;
pub type Arguments = Vec<CodePtr>;
pub type Expressions = Vec<CodePtr>;

pub struct Function {
    params: Parameters,
    body: Expressions
}

impl Function {
    pub fn new(params: Parameters, body: Expressions) -> Self {
        Function { params, body }
    }

    pub fn eval(&self, call_env: &mut Environment, args: &Arguments) -> ValueResult {
        if args.len() != self.params.len() {
            return Err(ValueError::new("Invalid arguments length"));
        }

        let mut func_env: Environment = Default::default();
        for (param, arg) in zip(&self.params, args) {
            func_env.def(param, arg.eval(call_env)?)?;
        }

        let mut result = Value::from_num(0.0);
        for expr in self.body.iter() {
            result = expr.eval(&mut func_env)?;
        }

        Ok(result)
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcalc_binary_ops::*;
    use crate::pcalc_code::*;

    #[test]
    fn test_function_empty() {
        let mut call_env = Environment::new();
        let func = Function::new(Parameters::new(), Expressions::new());
        assert_eq!(func.eval(&mut call_env, &Arguments::new()).unwrap(), Value::from_num(0.0));
    }

    #[test]
    fn test_function_literal() {
        let mut call_env = Environment::new();

        let params = Parameters::new();
        let mut exprs = Expressions::new();
        exprs.push(Box::new(Literal::new(Value::from_num(5.0))));

        let func = Function::new(params, exprs);
        assert_eq!(func.eval(&mut call_env, &Arguments::new()).unwrap(), Value::from_num(5.0));
    }

    #[test]
    fn test_function_add_literals() {
        let mut call_env = Environment::new();

        let params = Parameters::new();
        let mut exprs = Expressions::new();
        exprs.push(Box::new(BinaryOp::new(
            bop2ftn("+").unwrap(),
            Box::new(Literal::new(Value::from_num(2.0))),
            Box::new(Literal::new(Value::from_num(3.0)))
        )));

        let func = Function::new(params, exprs);
        assert_eq!(func.eval(&mut call_env, &Arguments::new()).unwrap(), Value::from_num(5.0));
    }

    #[test]
    fn test_function_add_arguments() {
        let mut call_env = Environment::new();
        call_env.def("z", Value::from_num(6.0)).unwrap();

        let mut params = Parameters::new();
        params.push(String::from("x"));
        params.push(String::from("y"));

        let mut exprs = Expressions::new();
        exprs.push(Box::new(BinaryOp::new(
            bop2ftn("+").unwrap(),
            Box::new(GetVar::new(String::from("x"))),
            Box::new(GetVar::new(String::from("y")))
        )));

        let mut args = Arguments::new();
        args.push(Box::new(Literal::new(Value::from_num(4.0))));
        args.push(Box::new(GetVar::new(String::from("z"))));

        let func = Function::new(params, exprs);
        assert_eq!(func.eval(&mut call_env, &args).unwrap(), Value::from_num(10.0));
    }

    #[test]
    fn test_function_multi_expr() {
        let mut call_env = Environment::new();
        call_env.def("z", Value::from_num(6.0)).unwrap();

        let mut params = Parameters::new();
        params.push(String::from("x"));
        params.push(String::from("y"));

        let mut exprs = Expressions::new();
        exprs.push(Box::new(DefVar::new(
            String::from("temp"),
            Box::new(BinaryOp::new(
                bop2ftn("+").unwrap(),
                Box::new(GetVar::new(String::from("x"))),
                Box::new(GetVar::new(String::from("y")))
            ))
        )));
        exprs.push(Box::new(BinaryOp::new(
            bop2ftn(">").unwrap(),
            Box::new(GetVar::new(String::from("temp"))),
            Box::new(Literal::new(Value::from_num(8.0)))
        )));

        let mut args = Arguments::new();
        args.push(Box::new(Literal::new(Value::from_num(4.0))));
        args.push(Box::new(GetVar::new(String::from("z"))));

        let func = Function::new(params, exprs);
        assert_eq!(func.eval(&mut call_env, &args).unwrap(), Value::from_bool(true));
    }

    #[test]
    fn test_function_temperature() {
        let mut call_env = Environment::new();
        call_env.def("temp", Value::from_num(54.0)).unwrap();

        let mut params = Parameters::new();
        params.push(String::from("fahrenheit"));

        let mut exprs = Expressions::new();
        exprs.push(Box::new(DefVar::new(
            String::from("celsius"),
            Box::new(BinaryOp::new(
                bop2ftn("-").unwrap(),
                Box::new(GetVar::new(String::from("fahrenheit"))),
                Box::new(Literal::new(Value::from_num(32.0)))
            ))
        )));
        exprs.push(Box::new(SetVar::new(
            String::from("celsius"),
            Box::new(BinaryOp::new(
                bop2ftn("*").unwrap(),
                Box::new(GetVar::new(String::from("celsius"))),
                Box::new(Literal::new(Value::from_num(5.0)))
            ))
        )));
        exprs.push(Box::new(BinaryOp::new(
            bop2ftn("/").unwrap(),
            Box::new(GetVar::new(String::from("celsius"))),
            Box::new(Literal::new(Value::from_num(9.0)))
        )));

        let mut args = Arguments::new();
        args.push(Box::new(GetVar::new(String::from("temp"))));

        let func = Function::new(params, exprs);
        assert_eq!(func.eval(&mut call_env, &args).unwrap(), Value::from_num(12.222222222222221));
    }

    #[test]
    fn test_function_invalid_arguments_length() {
        let mut call_env = Environment::new();

        let mut args = Arguments::new();
        args.push(Box::new(Literal::new(Value::from_num(1.0))));

        let func = Function::new(Parameters::new(), Expressions::new());
        match func.eval(&mut call_env, &args) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(format!("{}", err), "Invalid arguments length")
        };
    }
}
