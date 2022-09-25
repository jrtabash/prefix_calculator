use crate::pcalc_binary_ops::BinaryFtn;
use crate::pcalc_environment::Environment;
use crate::pcalc_unary_ops::UnaryFtn;
use crate::pcalc_value::{Value, ValueResult};
use std::fmt;

// --------------------------------------------------------------------------------
// Code

pub trait Code {
    fn eval(&self, env: &mut Environment) -> ValueResult;
}

pub type CodePtr = Box<dyn Code>;

impl fmt::Debug for dyn Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("[Code]").finish()
    }
}

// --------------------------------------------------------------------------------
// Literal

pub struct Literal {
    value: Value
}

impl Literal {
    pub fn new(value: Value) -> Self {
        Literal { value }
    }
}

impl Code for Literal {
    fn eval(&self, _env: &mut Environment) -> ValueResult {
        Ok(self.value)
    }
}

// --------------------------------------------------------------------------------
// DefVar

pub struct DefVar {
    name: String,
    code: CodePtr
}

impl DefVar {
    pub fn new(name: String, code: CodePtr) -> DefVar {
        DefVar { name, code }
    }
}

impl Code for DefVar {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        let value = self.code.eval(env)?;
        env.def(&self.name, value)
    }
}

// --------------------------------------------------------------------------------
// SetVar

pub struct SetVar {
    name: String,
    code: CodePtr
}

impl SetVar {
    pub fn new(name: String, code: CodePtr) -> SetVar {
        SetVar { name, code }
    }
}

impl Code for SetVar {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        let value = self.code.eval(env)?;
        env.set(&self.name, value)
    }
}

// --------------------------------------------------------------------------------
// GetVar

pub struct GetVar {
    name: String
}

impl GetVar {
    pub fn new(name: String) -> GetVar {
        GetVar { name }
    }
}

impl Code for GetVar {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        env.get(&self.name)
    }
}

// --------------------------------------------------------------------------------
// BinaryOp

pub struct BinaryOp {
    op_ftn: BinaryFtn,
    lhs_arg: CodePtr,
    rhs_arg: CodePtr
}

impl BinaryOp {
    pub fn new(op_ftn: BinaryFtn, lhs_arg: CodePtr, rhs_arg: CodePtr) -> BinaryOp {
        BinaryOp { op_ftn, lhs_arg, rhs_arg }
    }
}

impl Code for BinaryOp {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        let lhs_value = self.lhs_arg.eval(env)?;
        let rhs_value = self.rhs_arg.eval(env)?;
        (self.op_ftn)(&lhs_value, &rhs_value)
    }
}

// --------------------------------------------------------------------------------
// UnaryOp

pub struct UnaryOp {
    op_ftn: UnaryFtn,
    arg: CodePtr
}

impl UnaryOp {
    pub fn new(op_ftn: UnaryFtn, arg: CodePtr) -> UnaryOp {
        UnaryOp { op_ftn, arg }
    }
}

impl Code for UnaryOp {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        let value = self.arg.eval(env)?;
        (self.op_ftn)(&value)
    }
}

// --------------------------------------------------------------------------------
// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcalc_binary_ops::bop2ftn;
    use crate::pcalc_unary_ops::uop2ftn;

    #[test]
    fn test_literal() {
        let mut env = Environment::new();

        let lit = Literal::new(Value::from_num(5.0));
        assert_eq!(lit.eval(&mut env).unwrap(), Value::from_num(5.0));

        let lit = Literal::new(Value::from_bool(true));
        assert_eq!(lit.eval(&mut env).unwrap(), Value::from_bool(true));

        let lit = Literal::new(Value::from_bool(false));
        assert_eq!(lit.eval(&mut env).unwrap(), Value::from_bool(false));
    }

    #[test]
    fn test_variable() {
        let mut env = Environment::new();

        let defvar = DefVar::new(String::from("x"), Box::new(Literal::new(Value::from_num(5.0))));
        assert_eq!(defvar.eval(&mut env).unwrap(), Value::from_num(5.0));
        assert_eq!(env.get("x").unwrap(), Value::from_num(5.0));

        let setvar = SetVar::new(String::from("x"), Box::new(Literal::new(Value::from_num(10.0))));
        assert_eq!(setvar.eval(&mut env).unwrap(), Value::from_num(10.0));
        assert_eq!(env.get("x").unwrap(), Value::from_num(10.0));

        let getvar = GetVar::new(String::from("x"));
        assert_eq!(getvar.eval(&mut env).unwrap(), Value::from_num(10.0));
    }

    #[test]
    fn test_binaryop() {
        let mut env = Environment::new();

        let bop = BinaryOp::new(
            bop2ftn("+").unwrap(),
            Box::new(Literal::new(Value::from_num(2.0))),
            Box::new(Literal::new(Value::from_num(3.0)))
        );
        assert_eq!(bop.eval(&mut env).unwrap(), Value::from_num(5.0));

        let bop = BinaryOp::new(
            bop2ftn("or").unwrap(),
            Box::new(Literal::new(Value::from_bool(false))),
            Box::new(Literal::new(Value::from_bool(true)))
        );
        assert_eq!(bop.eval(&mut env).unwrap(), Value::from_bool(true));
    }

    #[test]
    fn test_unaryop() {
        let mut env = Environment::new();

        let uop = UnaryOp::new(uop2ftn("abs").unwrap(), Box::new(Literal::new(Value::from_num(-2.0))));
        assert_eq!(uop.eval(&mut env).unwrap(), Value::from_num(2.0));

        let uop = UnaryOp::new(uop2ftn("not").unwrap(), Box::new(Literal::new(Value::from_bool(false))));
        assert_eq!(uop.eval(&mut env).unwrap(), Value::from_bool(true));
    }
}
