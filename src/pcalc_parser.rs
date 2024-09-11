use crate::pcalc_binary_ops::bop2ftn;
use crate::pcalc_code::{BinaryOp, CodePtr, Conditional, DefVar, Defun, Funcall, GetVar, Literal, NoOp, SetVar, UnaryOp, XPrint};
use crate::pcalc_function::{Arguments, Expressions, Parameters};
use crate::pcalc_keywords as keywords;
use crate::pcalc_lexer::{Lexer, LexerError, TokenType};
use crate::pcalc_unary_ops::uop2ftn;
use crate::pcalc_value::Value;
use std::f64::consts;
use std::fmt;

// --------------------------------------------------------------------------------
// Parser Error

#[derive(Debug, Clone)]
pub struct ParserError {
    error_msg: String
}

impl ParserError {
    pub fn new(err_msg: &str) -> ParserError {
        ParserError {
            error_msg: String::from(err_msg)
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_msg)
    }
}

impl From<std::num::ParseFloatError> for ParserError {
    fn from(item: std::num::ParseFloatError) -> Self {
        ParserError {
            error_msg: format!("{}", item)
        }
    }
}

impl From<LexerError> for ParserError {
    fn from(item: LexerError) -> Self {
        ParserError {
            error_msg: String::from(item.message())
        }
    }
}

// --------------------------------------------------------------------------------
// Parser Result

pub type ParserResult = Result<CodePtr, ParserError>;

// --------------------------------------------------------------------------------
// Parser

pub struct Parser {
    lexer: Lexer
}

impl Parser {
    pub fn new() -> Self {
        Parser { lexer: Default::default() }
    }

    pub fn parse(&mut self, expr: &str) -> ParserResult {
        if let Err(err) = self.lexer.tokenize(expr) {
            self.lexer.clear();
            return Err(err.into());
        }

        if self.lexer.starts_with(TokenType::Defun) && !self.lexer.ends_with(TokenType::End) {
            // Partial function, wait for rest
            return Ok(Box::new(NoOp::new()));
        }

        match self.make_code() {
            Ok(code) => {
                // Expect a full/complete expression.
                if !self.lexer.is_empty() {
                    self.lexer.clear();
                    Err(ParserError::new(&format!("Invalid expression - '{}'", expr)))
                } else {
                    Ok(code)
                }
            }
            Err(err) => {
                self.lexer.clear();
                Err(err)
            }
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.lexer.is_empty()
    }

    // --------------------------------------------------------------------------------
    // Private Functions

    fn make_code(&mut self) -> ParserResult {
        if let Some(first) = self.lexer.next_token() {
            match first.ttype {
                TokenType::Literal => self.make_literal(&first.tname),
                TokenType::Const => self.make_const(&first.tname),
                TokenType::Define => self.make_variable(),
                TokenType::Assign => self.make_set_variable(),
                TokenType::Defun => self.make_function(),
                TokenType::Funcall => self.make_funcall(),
                TokenType::BinaryOp => self.make_binary_op(&first.tname),
                TokenType::UnaryOp => self.make_unary_op(&first.tname),
                TokenType::SpecialFtn => self.make_special_ftn(&first.tname),
                TokenType::Identifier => self.make_get_variable(&first.tname),
                TokenType::Begin => Err(ParserError::new("Invalid expression containing begin")),
                TokenType::End | TokenType::CEnd => Err(ParserError::new("Invalid expression containing end")),
                TokenType::If => self.make_conditional(&first.tname),
                TokenType::Then => Err(ParserError::new("Invalid expression containing then")),
                TokenType::Else => Err(ParserError::new("Invalid expression containing else")),
                TokenType::Fi => Err(ParserError::new("Invalid expression containing fi"))
            }
        } else {
            Err(ParserError::new("Expecting token"))
        }
    }

    fn make_literal(&self, tname: &str) -> ParserResult {
        let value = match tname {
            keywords::TRUE => Value::from_bool(true),
            keywords::FALSE => Value::from_bool(false),
            _ => Value::from_num(tname.parse::<f64>()?)
        };
        Ok(Box::new(Literal::new(value)))
    }

    fn make_const(&self, tname: &str) -> ParserResult {
        let value = match tname {
            keywords::PI => Some(Value::from_num(consts::PI)),
            keywords::TAU => Some(Value::from_num(consts::TAU)),
            keywords::E => Some(Value::from_num(consts::E)),
            keywords::PHI => Some(Value::from_num(1.618033988749895f64)),
            _ => None
        };
        if let Some(val) = value {
            Ok(Box::new(Literal::new(val)))
        } else {
            Err(ParserError::new(&format!("Unknown constant - '{}'", tname)))
        }
    }

    fn make_variable(&mut self) -> ParserResult {
        if let Some(name_token) = self.lexer.next_token() {
            if name_token.ttype == TokenType::Identifier {
                Ok(Box::new(DefVar::new(name_token.tname, self.make_code()?)))
            } else {
                Err(ParserError::new(&format!("Invalid variable definition name - '{}'", name_token.tname)))
            }
        } else {
            Err(ParserError::new("Incomplete variable definition"))
        }
    }

    fn make_set_variable(&mut self) -> ParserResult {
        if let Some(name_token) = self.lexer.next_token() {
            if name_token.ttype == TokenType::Identifier {
                Ok(Box::new(SetVar::new(name_token.tname, self.make_code()?)))
            } else {
                Err(ParserError::new(&format!("Invalid set variable name - '{}'", name_token.tname)))
            }
        } else {
            Err(ParserError::new("Incomplete set variable"))
        }
    }

    fn make_function(&mut self) -> ParserResult {
        if let Some(ftok) = self.lexer.next_token() {
            self.lexer.check_reserved(&ftok, "function name definition")?;

            let mut params = Parameters::new();
            let mut body = Expressions::new();
            loop {
                if let Some(ptok) = self.lexer.next_token() {
                    if ptok.ttype == TokenType::Begin {
                        break;
                    }
                    self.lexer.check_reserved(&ptok, "function parameter definition")?;
                    params.push(ptok.tname);
                } else {
                    return Err(ParserError::new("Invalid function definition/parameters"));
                }
            }
            loop {
                if let Some(ctok) = self.lexer.peek_token() {
                    if ctok.ttype == TokenType::End {
                        self.lexer.next_token();
                        break;
                    }
                    body.push(self.make_code()?);
                } else {
                    return Err(ParserError::new("Invalid function definition/body"));
                }
            }
            Ok(Box::new(Defun::new(ftok.tname, params, body)))
        } else {
            Err(ParserError::new("Invalid function definition"))
        }
    }

    fn make_funcall(&mut self) -> ParserResult {
        if let Some(ftok) = self.lexer.next_token() {
            let mut args = Arguments::new();
            loop {
                if let Some(atok) = self.lexer.peek_token() {
                    if atok.ttype == TokenType::CEnd {
                        self.lexer.next_token();
                        break;
                    }
                    args.push(self.make_code()?);
                } else {
                    return Err(ParserError::new("Invalid function call/arguments"));
                }
            }
            Ok(Box::new(Funcall::new(ftok.tname, args)))
        } else {
            Err(ParserError::new("Invalid function call"))
        }
    }

    fn make_get_variable(&self, name: &str) -> ParserResult {
        Ok(Box::new(GetVar::new(String::from(name))))
    }

    fn make_conditional(&mut self, _name: &str) -> ParserResult {
        let (cond, _) = self.make_conditional_part(TokenType::Then, false)?;
        let (true_code, stop) = self.make_conditional_part(TokenType::Else, true)?;

        if stop {
            Ok(Box::new(Conditional::when(cond, true_code)))
        } else {
            let (false_code, _) = self.make_conditional_part(TokenType::Fi, false)?;
            Ok(Box::new(Conditional::new(cond, true_code, false_code)))
        }
    }

    fn make_conditional_part(&mut self, ends_with: TokenType, or_fi: bool) -> Result<(CodePtr, bool), ParserError> {
        let part = self.make_code()?;
        if let Some(tok) = self.lexer.peek_token() {
            if tok.ttype == ends_with {
                self.lexer.next_token();
                Ok((part, false))
            } else if or_fi && tok.ttype == TokenType::Fi {
                self.lexer.next_token();
                Ok((part, true))
            } else {
                Err(ParserError::new(&format!("Invalid if expression - expecting '{}'", ends_with.to_string())))
            }
        } else {
            Err(ParserError::new(&format!("Incomplete if expression - missing '{}'", ends_with.to_string())))
        }
    }

    fn make_binary_op(&mut self, name: &str) -> ParserResult {
        if let Some(ftn) = bop2ftn(name) {
            Ok(Box::new(BinaryOp::new(ftn, self.make_code()?, self.make_code()?)))
        } else {
            Err(ParserError::new(&format!("Unknown binary op - {}", name)))
        }
    }

    fn make_unary_op(&mut self, name: &str) -> ParserResult {
        if let Some(ftn) = uop2ftn(name) {
            Ok(Box::new(UnaryOp::new(ftn, self.make_code()?)))
        } else {
            Err(ParserError::new(&format!("Unknown unary op - {}", name)))
        }
    }

    fn make_special_ftn(&mut self, name: &str) -> ParserResult {
        match name {
            keywords::XPRINT => Ok(Box::new(XPrint::new(self.make_code()?))),
            _ => Err(ParserError::new(&format!("Unknown special ftn - {}", name)))
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

// --------------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pcalc_environment::Environment;

    #[test]
    fn test_parser_literal() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        test_parse(&mut parser, &mut env, "true", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "false", Value::from_bool(false));
        test_parse(&mut parser, &mut env, "5.0", Value::from_num(5.0));
        test_parse(&mut parser, &mut env, "-5.0", Value::from_num(-5.0));
    }

    #[test]
    fn test_parser_const() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        test_parse(&mut parser, &mut env, "pi", Value::from_num(consts::PI));
        test_parse(&mut parser, &mut env, "tau", Value::from_num(consts::TAU));
        test_parse(&mut parser, &mut env, "e", Value::from_num(consts::E));
        test_parse(&mut parser, &mut env, "phi", Value::from_num(1.6180339887498949f64));
    }

    #[test]
    fn test_parser_defvar() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        test_parse(&mut parser, &mut env, "var flag true", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "var num 10", Value::from_num(10.0));

        test_parse_error(&mut parser, "var bad", "Expecting token");
        test_parse_error(&mut parser, "var", "Incomplete variable definition");
        test_parse_error(&mut parser, "var true 5", "Invalid variable definition name - 'true'");
        test_parse_error(&mut parser, "var sqrt 5", "Invalid variable definition name - 'sqrt'");

        assert_eq!(env.len(), 2);
        assert_eq!(env.get_var("flag").unwrap(), Value::from_bool(true));
        assert_eq!(env.get_var("num").unwrap(), Value::from_num(10.0));
    }

    #[test]
    fn test_parser_setvar() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        test_parse(&mut parser, &mut env, "var flag true", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "var num 10", Value::from_num(10.0));

        test_parse(&mut parser, &mut env, "= flag false", Value::from_bool(false));
        test_parse(&mut parser, &mut env, "= num 20", Value::from_num(20.0));

        test_parse_error(&mut parser, "= bad", "Expecting token");
        test_parse_error(&mut parser, "=", "Incomplete set variable");
        test_parse_error(&mut parser, "= true 5", "Invalid set variable name - 'true'");

        assert_eq!(env.len(), 2);
        assert_eq!(env.get_var("flag").unwrap(), Value::from_bool(false));
        assert_eq!(env.get_var("num").unwrap(), Value::from_num(20.0));
    }

    #[test]
    fn test_parser_getvar() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        test_parse(&mut parser, &mut env, "var flag true", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "var num 10", Value::from_num(10.0));

        test_parse(&mut parser, &mut env, "flag", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "num", Value::from_num(10.0));

        assert_eq!(env.len(), 2);
        assert_eq!(env.get_var("flag").unwrap(), Value::from_bool(true));
        assert_eq!(env.get_var("num").unwrap(), Value::from_num(10.0));
    }

    #[test]
    fn test_parser_binary_op() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        test_parse(&mut parser, &mut env, "+ 2 3", Value::from_num(5.0));
        test_parse(&mut parser, &mut env, "- 4 2", Value::from_num(2.0));
        test_parse(&mut parser, &mut env, "* 2 3", Value::from_num(6.0));
        test_parse(&mut parser, &mut env, "/ 6 2", Value::from_num(3.0));
        test_parse(&mut parser, &mut env, "% 5 2", Value::from_num(1.0));
        test_parse(&mut parser, &mut env, "^ 2 3", Value::from_num(8.0));
        test_parse(&mut parser, &mut env, "max 2 4", Value::from_num(4.0));
        test_parse(&mut parser, &mut env, "min 2 4", Value::from_num(2.0));

        test_parse(&mut parser, &mut env, "== 1 1", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "!= 1 2", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "< 2 3", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "<= 2 3", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "> 3 2", Value::from_bool(true));
        test_parse(&mut parser, &mut env, ">= 3 2", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "and true true", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "or true false", Value::from_bool(true));

        test_parse_error(&mut parser, "+", "Expecting token");
        test_parse_error(&mut parser, "+ 1", "Expecting token");
    }

    #[test]
    fn test_parser_unary_op() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        let one: f64 = 1.0;
        let two: f64 = 2.0;
        let ten: f64 = 10.0;
        test_parse(&mut parser, &mut env, "sqrt 10", Value::from_num(ten.sqrt()));
        test_parse(&mut parser, &mut env, "exp 1", Value::from_num(one.exp()));
        test_parse(&mut parser, &mut env, "exp2 2", Value::from_num(two.exp2()));
        test_parse(&mut parser, &mut env, "ln 1", Value::from_num(one.ln()));
        test_parse(&mut parser, &mut env, "log2 10", Value::from_num(ten.log2()));
        test_parse(&mut parser, &mut env, "log10 10", Value::from_num(ten.log10()));
        test_parse(&mut parser, &mut env, "sin 1", Value::from_num(one.sin()));
        test_parse(&mut parser, &mut env, "cos 1", Value::from_num(one.cos()));
        test_parse(&mut parser, &mut env, "tan 1", Value::from_num(one.tan()));
        test_parse(&mut parser, &mut env, "sinh 1", Value::from_num(one.sinh()));
        test_parse(&mut parser, &mut env, "cosh 1", Value::from_num(one.cosh()));
        test_parse(&mut parser, &mut env, "tanh 1", Value::from_num(one.tanh()));
        test_parse(&mut parser, &mut env, "asin 1", Value::from_num(one.asin()));
        test_parse(&mut parser, &mut env, "acos 1", Value::from_num(one.acos()));
        test_parse(&mut parser, &mut env, "atan 1", Value::from_num(one.atan()));
        test_parse(&mut parser, &mut env, "asinh 1", Value::from_num(one.asinh()));
        test_parse(&mut parser, &mut env, "acosh 1", Value::from_num(one.acosh()));
        test_parse(&mut parser, &mut env, "atanh 1", Value::from_num(one.atanh()));
        test_parse(&mut parser, &mut env, "sign 10", Value::from_num(one));
        test_parse(&mut parser, &mut env, "abs -10", Value::from_num(ten));
        test_parse(&mut parser, &mut env, "recip 0.5", Value::from_num(two));
        test_parse(&mut parser, &mut env, "fract 25.0", Value::from_num(0.0));
        test_parse(&mut parser, &mut env, "trunc 25.2", Value::from_num(25.0));
        test_parse(&mut parser, &mut env, "ceil 1.2", Value::from_num(two));
        test_parse(&mut parser, &mut env, "floor 1.6", Value::from_num(one));
        test_parse(&mut parser, &mut env, "round 1.2", Value::from_num(one));
        test_parse(&mut parser, &mut env, "neg -10", Value::from_num(ten));
        test_parse(&mut parser, &mut env, "not true", Value::from_bool(false));

        test_parse_error(&mut parser, "sqrt", "Expecting token");
    }

    #[test]
    fn test_parser_special_ftn_xprint() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        test_parse(&mut parser, &mut env, "xprint 10", Value::from_num(10.0));
        test_parse(&mut parser, &mut env, "xprint true", Value::from_bool(true));
    }

    #[test]
    fn test_parser_defun() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        test_parse(&mut parser, &mut env, "def add x y begin + x y end", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "def add x y\nbegin\n+ x y\nend", Value::from_bool(true));

        test_parse_error(
            &mut parser,
            "def sqrt x begin ^ x 0.5 end",
            "Invalid reserved function name definition - 'sqrt'"
        );
        test_parse_error(
            &mut parser,
            "def mysqrt tau begin ^ tau 0.5 end",
            "Invalid reserved function parameter definition - 'tau'"
        );
    }

    #[test]
    fn test_parser_funcall() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        test_parse(&mut parser, &mut env, "def bar1 begin 1 end", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "call bar1 cend", Value::from_num(1.0));

        test_parse(&mut parser, &mut env, "def add x y z begin + x + y z end", Value::from_bool(true));
        test_parse(&mut parser, &mut env, "call add 1 2 3 cend", Value::from_num(6.0));
        test_parse(&mut parser, &mut env, "+ 1 call add + 2 3 1 - 5 3 cend", Value::from_num(9.0));
        test_parse(&mut parser, &mut env, "+ call add + 2 3 1 - 5 3 cend 1", Value::from_num(9.0));

        test_parse_error(&mut parser, "call bar1", "Invalid function call/arguments");
        test_parse_error(&mut parser, "call add 1 2 3", "Invalid function call/arguments");

        test_parse_eval_error(&mut parser, &mut env, "call bar1 1 cend", "Invalid arguments length");
        test_parse_eval_error(&mut parser, &mut env, "call add 1 2 cend", "Invalid arguments length");
        test_parse_eval_error(&mut parser, &mut env, "call sub 10 5 cend", "Unknown function 'sub'");
    }

    #[test]
    fn test_parser_conditional() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        test_parse(&mut parser, &mut env, "var x 5", Value::from_num(5.0));
        test_parse(&mut parser, &mut env, "var y 10", Value::from_num(10.0));

        test_parse(&mut parser, &mut env, "if true ? 1 : 2 fi", Value::from_num(1.0));
        test_parse(&mut parser, &mut env, "if false ? 1 : 2 fi", Value::from_num(2.0));

        test_parse(&mut parser, &mut env, "if <= x 5 ? x : y fi", Value::from_num(5.0));
        test_parse(&mut parser, &mut env, "if > x 5 ? x : y fi", Value::from_num(10.0));

        test_parse(&mut parser, &mut env, "if <= x 5 ? = x + x 1 : = y + y 1 fi", Value::from_num(6.0));
        test_parse(&mut parser, &mut env, "x", Value::from_num(6.0));

        test_parse(&mut parser, &mut env, "if < y 10 ? = x + x 1 : = y + y 1 fi", Value::from_num(11.0));
        test_parse(&mut parser, &mut env, "y", Value::from_num(11.0));

        test_parse_error(&mut parser, "if true 1 fi", "Invalid if expression - expecting 'Then'");
        test_parse_error(&mut parser, "if true ? 1 : 0", "Incomplete if expression - missing 'Fi'");
        test_parse_error(&mut parser, "if true ? 1 0 fi", "Invalid if expression - expecting 'Else'");
    }

    #[test]
    fn test_parser_conditional_when() {
        let mut env = Environment::new();
        let mut parser = Parser::new();
        test_parse(&mut parser, &mut env, "var x 5", Value::from_num(5.0));
        test_parse(&mut parser, &mut env, "var y 10", Value::from_num(10.0));

        test_parse(&mut parser, &mut env, "if true ? 1 fi", Value::from_num(1.0));
        test_parse(&mut parser, &mut env, "if false ? 1 fi", Value::from_bool(false));

        test_parse(&mut parser, &mut env, "if <= x 5 ? x fi", Value::from_num(5.0));
        test_parse(&mut parser, &mut env, "if > x 5 ? x fi", Value::from_bool(false));

        test_parse(&mut parser, &mut env, "if <= x 5 ? = x + x 1 fi", Value::from_num(6.0));
        test_parse(&mut parser, &mut env, "x", Value::from_num(6.0));

        test_parse(&mut parser, &mut env, "if < y 10 ? = y + y 1 fi", Value::from_bool(false));
        test_parse(&mut parser, &mut env, "y", Value::from_num(10.0));

        test_parse_error(&mut parser, "if true fi", "Invalid if expression - expecting 'Then'");
        test_parse_error(&mut parser, "if true ? 1", "Incomplete if expression - missing 'Else'");
    }

    fn test_parse(parser: &mut Parser, env: &mut Environment, expr: &str, value: Value) {
        let code = parser.parse(expr).unwrap();
        assert_eq!(code.eval(env).unwrap(), value);
    }

    fn test_parse_error(parser: &mut Parser, expr: &str, error: &str) {
        match parser.parse(expr) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(format!("{}", err), error)
        };
    }

    fn test_parse_eval_error(parser: &mut Parser, env: &mut Environment, expr: &str, error: &str) {
        let code = parser.parse(expr).unwrap();
        match code.eval(env) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(format!("{}", err), error)
        };
    }
}
