use std::cmp;
use std::collections::HashMap;

// --------------------------------------------------------------------------------
// TokenType

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    BinaryOp,
    UnaryOp,
    Literal,
    Const,
    Define,
    Assign,
    Identifier
}

// --------------------------------------------------------------------------------
// Token

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub tname: String
}

impl Token {
    pub fn new(ttype: TokenType, tname: &str) -> Self {
        Token {
            ttype: ttype,
            tname: String::from(tname)
        }
    }
}

impl cmp::PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.ttype == other.ttype && self.tname == other.tname
    }
}

// --------------------------------------------------------------------------------
// Lexer

pub struct Lexer {
    table: HashMap<String, TokenType>,
    tokens: Vec<Token>
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            table: Lexer::make_token_types(),
            tokens: Vec::new()
        }
    }

    pub fn token_type(&self, token: &str) -> TokenType {
        if let Some(toktyp) = self.table.get(token) {
            *toktyp
        }
        else if token.parse::<f64>().is_ok() {
            TokenType::Literal
        }
        else {
            TokenType::Identifier
        }
    }

    pub fn tokenize(&mut self, expr: &str) {
        for tok in expr.split_whitespace() {
            self.tokens.push(Token::new(self.token_type(tok), tok));
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if !self.tokens.is_empty() {
            Some(self.tokens.remove(0))
        }
        else {
            None
        }
    }

    pub fn peek_token(&self) -> Option<&Token> {
        if !self.tokens.is_empty() {
            Some(&self.tokens[0])
        }
        else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.tokens.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    // --------------------------------------------------------------------------------
    // Private Functions

    fn make_token_types() -> HashMap<String, TokenType> {
        let mut table: HashMap<String, TokenType> = HashMap::new();

        let bops = vec!["+", "-", "*", "/", "%", "^",
                        "max", "min",
                        "==", "!=", "<", "<=", ">", ">=",
                        "and", "or"];
        for sym in bops.iter() {
            table.insert(String::from(*sym), TokenType::BinaryOp);
        }

        let uops = vec!["sqrt", "exp", "exp2", "ln", "log2", "log10",
                        "sin", "cos", "tan", "sinh", "cosh", "tanh",
                        "asin", "acos", "atan", "asinh", "acosh", "atanh",
                        "sign", "abs", "recip", "fract", "trunc",
                        "ceil", "floor", "round",
                        "neg", "not"];
        for sym in uops.iter() {
            table.insert(String::from(*sym), TokenType::UnaryOp);
        }

        for sym in vec!["pi", "tau", "e"].iter() {
            table.insert(String::from(*sym), TokenType::Const);
        }

        table.insert(String::from("true"), TokenType::Literal);
        table.insert(String::from("false"), TokenType::Literal);
        table.insert(String::from("var"), TokenType::Define);
        table.insert(String::from("="), TokenType::Assign);

        table
    }
}

// --------------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_token_type() {
        let lexer = Lexer::new();

        let bops = vec!["+", "-", "*", "/", "%", "^",
                        "max", "min",
                        "==", "!=", "<", "<=", ">", ">=",
                        "and", "or"];
        for sym in bops.iter() {
            assert_eq!(lexer.token_type(sym), TokenType::BinaryOp);
        }

        let uops = vec!["sqrt", "exp", "exp2", "ln", "log2", "log10",
                        "sin", "cos", "tan", "sinh", "cosh", "tanh",
                        "asin", "acos", "atan", "asinh", "acosh", "atanh",
                        "sign", "abs", "recip", "fract", "trunc",
                        "ceil", "floor", "round",
                        "neg", "not"];
        for sym in uops.iter() {
            assert_eq!(lexer.token_type(sym), TokenType::UnaryOp);
        }

        for sym in vec!["pi", "tau", "e"].iter() {
            assert_eq!(lexer.token_type(sym), TokenType::Const);
        }

        assert_eq!(lexer.token_type("var"), TokenType::Define);
        assert_eq!(lexer.token_type("="), TokenType::Assign);
        assert_eq!(lexer.token_type("5.0"), TokenType::Literal);
        assert_eq!(lexer.token_type("true"), TokenType::Literal);
        assert_eq!(lexer.token_type("foobar"), TokenType::Identifier);
    }

    #[test]
    fn test_lexer_tokenize() {
        fn test_a_plus5(lexer: &mut Lexer, a_plus5: &str) {
            lexer.tokenize(a_plus5);
            assert_eq!(lexer.next_token().unwrap(), Token::new(TokenType::BinaryOp, "+"));
            assert_eq!(lexer.next_token().unwrap(), Token::new(TokenType::Identifier, "a"));
            assert_eq!(lexer.next_token().unwrap(), Token::new(TokenType::Literal, "5"));
            assert!(lexer.next_token().is_none());
        }

        let mut lexer = Lexer::new();

        test_a_plus5(&mut lexer, "+ a 5");
        test_a_plus5(&mut lexer, "+  a  5 \n");
        test_a_plus5(&mut lexer, "\n+ \n  a  5 \n");

        lexer.tokenize("+ 10 5");
        assert_eq!(*lexer.peek_token().unwrap(), Token::new(TokenType::BinaryOp, "+"));
        assert!(!lexer.is_empty());

        lexer.clear();
        assert!(lexer.peek_token().is_none());
        assert!(lexer.is_empty());
    }
}
