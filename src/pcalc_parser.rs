use crate::pcalc_binary_ops::bop2ftn;
use crate::pcalc_code::{BinaryOp, CodePtr, DefVar, GetVar, Literal, SetVar, UnaryOp};
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
        Parser { lexer: Lexer::new() }
    }

    pub fn parse(&mut self, expr: &str) -> ParserResult {
        if let Err(err) = self.lexer.tokenize(expr) {
            self.lexer.clear();
            return Err(err.into());
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

    // --------------------------------------------------------------------------------
    // Private Functions

    fn make_code(&mut self) -> ParserResult {
        if let Some(first) = self.lexer.next_token() {
            match first.ttype {
                TokenType::Literal => self.make_literal(&first.tname),
                TokenType::Const => self.make_const(&first.tname),
                TokenType::Define => self.make_variable(),
                TokenType::Assign => self.make_set_variable(),
                TokenType::BinaryOp => self.make_binary_op(&first.tname),
                TokenType::UnaryOp => self.make_unary_op(&first.tname),
                TokenType::Identifier => self.make_get_variable(&first.tname)
            }
        } else {
            Err(ParserError::new("Expecting token"))
        }
    }

    fn make_literal(&self, tname: &str) -> ParserResult {
        let value = match tname {
            "true" => Value::from_bool(true),
            "false" => Value::from_bool(false),
            _ => Value::from_num(tname.parse::<f64>()?)
        };
        Ok(Box::new(Literal::new(value)))
    }

    fn make_const(&self, tname: &str) -> ParserResult {
        let value = match tname {
            "pi" => Some(Value::from_num(consts::PI)),
            "tau" => Some(Value::from_num(consts::TAU)),
            "e" => Some(Value::from_num(consts::E)),
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

    fn make_get_variable(&self, name: &str) -> ParserResult {
        Ok(Box::new(GetVar::new(String::from(name))))
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

        assert_eq!(env.len(), 2);
        assert_eq!(env.get("flag").unwrap(), Value::from_bool(true));
        assert_eq!(env.get("num").unwrap(), Value::from_num(10.0));
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
        assert_eq!(env.get("flag").unwrap(), Value::from_bool(false));
        assert_eq!(env.get("num").unwrap(), Value::from_num(20.0));
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
        assert_eq!(env.get("flag").unwrap(), Value::from_bool(true));
        assert_eq!(env.get("num").unwrap(), Value::from_num(10.0));
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

    fn test_parse(parser: &mut Parser, env: &mut Environment, expr: &str, value: Value) {
        let code = parser.parse(expr).unwrap();
        assert_eq!(code.eval(env).unwrap(), value);
    }

    fn test_parse_error(parser: &mut Parser, expr: &str, error: &str) {
        match parser.parse(expr) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(format!("{}", err), error)
        }
    }
}
