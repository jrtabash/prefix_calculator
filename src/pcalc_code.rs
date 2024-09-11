use crate::pcalc_binary_ops::BinaryFtn;
use crate::pcalc_environment::Environment;
use crate::pcalc_function::{Arguments, Expressions, Function, FunctionPtr, Parameters};
use crate::pcalc_recursive_check::*;
use crate::pcalc_unary_ops::UnaryFtn;
use crate::pcalc_value::{Value, ValueError, ValueResult};
use std::fmt;

// --------------------------------------------------------------------------------
// Code

pub trait Code {
    fn eval(&self, env: &mut Environment) -> ValueResult;

    #[inline(always)]
    fn is_evaluable(&self) -> bool {
        true
    }

    #[inline(always)]
    fn is_funcall(&self) -> bool {
        false
    }

    #[inline(always)]
    fn get_name(&self) -> Option<&str> {
        None
    }
}

pub type CodePtr = Box<dyn Code>;

impl fmt::Debug for dyn Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("[Code]").finish()
    }
}

// --------------------------------------------------------------------------------
// NoOp

pub struct NoOp {}

impl NoOp {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        NoOp {}
    }
}

impl Code for NoOp {
    fn eval(&self, _env: &mut Environment) -> ValueResult {
        Err(ValueError::new("Eval called on noop"))
    }

    #[inline(always)]
    fn is_evaluable(&self) -> bool {
        false
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
    pub fn new(name: String, code: CodePtr) -> Self {
        DefVar { name, code }
    }
}

impl Code for DefVar {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        let value = self.code.eval(env)?;
        env.def_var(&self.name, value)
    }

    #[inline(always)]
    fn get_name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

// --------------------------------------------------------------------------------
// SetVar

pub struct SetVar {
    name: String,
    code: CodePtr
}

impl SetVar {
    pub fn new(name: String, code: CodePtr) -> Self {
        SetVar { name, code }
    }
}

impl Code for SetVar {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        let value = self.code.eval(env)?;
        env.set_var(&self.name, value)
    }

    #[inline(always)]
    fn get_name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

// --------------------------------------------------------------------------------
// GetVar

pub struct GetVar {
    name: String
}

impl GetVar {
    pub fn new(name: String) -> Self {
        GetVar { name }
    }
}

impl Code for GetVar {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        env.get_var(&self.name)
    }

    #[inline(always)]
    fn get_name(&self) -> Option<&str> {
        Some(&self.name)
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
    pub fn new(op_ftn: BinaryFtn, lhs_arg: CodePtr, rhs_arg: CodePtr) -> Self {
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
    pub fn new(op_ftn: UnaryFtn, arg: CodePtr) -> Self {
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
// XPrint - Execute and Print Expression

pub struct XPrint {
    expr: CodePtr
}

impl XPrint {
    pub fn new(expr: CodePtr) -> Self {
        XPrint { expr }
    }
}

impl Code for XPrint {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        let value = self.expr.eval(env)?;
        println!("{}", value);
        Ok(value)
    }
}

// --------------------------------------------------------------------------------
// Defun - Define Function

pub struct Defun {
    name: String,
    func: FunctionPtr
}

impl Defun {
    pub fn new(name: String, params: Parameters, body: Expressions) -> Self {
        Defun {
            name,
            func: FunctionPtr::new(Function::new(params, body))
        }
    }
}

impl Code for Defun {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        check_self_recursive(&self.name, &self.func)?;
        check_cross_recursive(&self.name, &self.func, env)?;

        env.def_func(&self.name, &self.func);
        Ok(Value::from_bool(true))
    }

    #[inline(always)]
    fn get_name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

// --------------------------------------------------------------------------------
// Funcall - Function Call

pub struct Funcall {
    name: String,
    args: Arguments
}

impl Funcall {
    pub fn new(name: String, args: Arguments) -> Self {
        Funcall { name, args }
    }
}

impl Code for Funcall {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        let func = FunctionPtr::clone(env.get_func(&self.name)?);
        func.eval(env, &self.args)
    }

    #[inline(always)]
    fn is_funcall(&self) -> bool {
        true
    }

    #[inline(always)]
    fn get_name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

// --------------------------------------------------------------------------------
// Conditional - If/Else

pub struct Conditional {
    cond: CodePtr,
    true_code: CodePtr,
    false_code: CodePtr
}

impl Conditional {
    pub fn new(cond: CodePtr, true_code: CodePtr, false_code: CodePtr) -> Self {
        Conditional { cond, true_code, false_code }
    }

    pub fn when(cond: CodePtr, true_code: CodePtr) -> Self {
        Conditional {
            cond,
            true_code,
            false_code: Box::new(Literal::new(Value::from_bool(false)))
        }
    }
}

impl Code for Conditional {
    fn eval(&self, env: &mut Environment) -> ValueResult {
        if self.cond.eval(env)?.as_bool() {
            Ok(self.true_code.eval(env)?)
        } else {
            Ok(self.false_code.eval(env)?)
        }
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
        assert_eq!(env.get_var("x").unwrap(), Value::from_num(5.0));

        let setvar = SetVar::new(String::from("x"), Box::new(Literal::new(Value::from_num(10.0))));
        assert_eq!(setvar.eval(&mut env).unwrap(), Value::from_num(10.0));
        assert_eq!(env.get_var("x").unwrap(), Value::from_num(10.0));

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

    #[test]
    fn test_print() {
        let mut env = Environment::new();

        let xprt = XPrint::new(Box::new(Literal::new(Value::from_num(5.0))));
        assert_eq!(xprt.eval(&mut env).unwrap(), Value::from_num(5.0));
    }

    #[test]
    fn test_defun() {
        let mut func_env = Environment::new();
        let mut call_env = Environment::new();
        call_env.def_var("z", Value::from_num(6.0)).unwrap();

        let mut params = Parameters::new();
        params.push(String::from("x"));
        params.push(String::from("y"));

        let mut exprs = Expressions::new();
        exprs.push(Box::new(BinaryOp::new(
            bop2ftn("+").unwrap(),
            Box::new(GetVar::new(String::from("x"))),
            Box::new(GetVar::new(String::from("y")))
        )));

        let defun = Defun::new("my_add".to_string(), params, exprs);
        assert_eq!(defun.eval(&mut func_env).unwrap(), Value::from_bool(true));

        let my_add = func_env.get_func("my_add").unwrap();

        let mut args = Arguments::new();
        args.push(Box::new(Literal::new(Value::from_num(4.0))));
        args.push(Box::new(GetVar::new(String::from("z"))));

        assert_eq!(my_add.eval(&mut call_env, &args).unwrap(), Value::from_num(10.0));
    }

    #[test]
    fn test_funcall() {
        let mut call_env = Environment::new();
        call_env.def_var("z", Value::from_num(6.0)).unwrap();

        let mut params = Parameters::new();
        params.push(String::from("x"));
        params.push(String::from("y"));

        let mut exprs = Expressions::new();
        exprs.push(Box::new(BinaryOp::new(
            bop2ftn("+").unwrap(),
            Box::new(GetVar::new(String::from("x"))),
            Box::new(GetVar::new(String::from("y")))
        )));

        let defun = Defun::new("my_add".to_string(), params, exprs);
        assert_eq!(defun.eval(&mut call_env).unwrap(), Value::from_bool(true));

        let mut args = Arguments::new();
        args.push(Box::new(Literal::new(Value::from_num(4.0))));
        args.push(Box::new(GetVar::new(String::from("z"))));

        let funcall = Funcall::new("my_add".to_string(), args);
        assert_eq!(funcall.eval(&mut call_env).unwrap(), Value::from_num(10.0));
    }

    #[test]
    fn test_conditional() {
        let mut env = Environment::new();
        env.def_var("check3", Value::from_bool(true)).unwrap();
        env.def_var("check4", Value::from_bool(false)).unwrap();
        env.def_var("true_code", Value::from_num(3.0)).unwrap();
        env.def_var("false_code", Value::from_num(4.0)).unwrap();

        let cond = Conditional::new(
            Box::new(Literal::new(Value::from_bool(true))),
            Box::new(Literal::new(Value::from_num(1.0))),
            Box::new(Literal::new(Value::from_num(2.0)))
        );
        assert_eq!(cond.eval(&mut env).unwrap(), Value::from_num(1.0));

        let cond = Conditional::new(
            Box::new(Literal::new(Value::from_bool(false))),
            Box::new(Literal::new(Value::from_num(1.0))),
            Box::new(Literal::new(Value::from_num(2.0)))
        );
        assert_eq!(cond.eval(&mut env).unwrap(), Value::from_num(2.0));

        let cond = Conditional::new(
            Box::new(GetVar::new(String::from("check3"))),
            Box::new(GetVar::new(String::from("true_code"))),
            Box::new(GetVar::new(String::from("false_code")))
        );
        assert_eq!(cond.eval(&mut env).unwrap(), Value::from_num(3.0));

        let cond = Conditional::new(
            Box::new(GetVar::new(String::from("check4"))),
            Box::new(GetVar::new(String::from("true_code"))),
            Box::new(GetVar::new(String::from("false_code")))
        );
        assert_eq!(cond.eval(&mut env).unwrap(), Value::from_num(4.0));
    }

    #[test]
    fn test_conditional_when() {
        let mut env = Environment::new();
        env.def_var("check3", Value::from_bool(true)).unwrap();
        env.def_var("check4", Value::from_bool(false)).unwrap();
        env.def_var("true_code", Value::from_num(3.0)).unwrap();

        let cond = Conditional::when(Box::new(Literal::new(Value::from_bool(true))), Box::new(Literal::new(Value::from_num(1.0))));
        assert_eq!(cond.eval(&mut env).unwrap(), Value::from_num(1.0));

        let cond = Conditional::when(Box::new(Literal::new(Value::from_bool(false))), Box::new(Literal::new(Value::from_num(1.0))));
        assert_eq!(cond.eval(&mut env).unwrap(), Value::from_bool(false));

        let cond = Conditional::when(Box::new(GetVar::new(String::from("check3"))), Box::new(GetVar::new(String::from("true_code"))));
        assert_eq!(cond.eval(&mut env).unwrap(), Value::from_num(3.0));

        let cond = Conditional::when(Box::new(GetVar::new(String::from("check4"))), Box::new(GetVar::new(String::from("true_code"))));
        assert_eq!(cond.eval(&mut env).unwrap(), Value::from_bool(false));
    }
}
