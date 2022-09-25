use crate::pcalc_keywords as keywords;
use std::cmp;
use std::collections::HashMap;

// --------------------------------------------------------------------------------
// Parser Error

#[derive(Debug, Clone)]
pub struct LexerError {
    error_msg: String
}

impl LexerError {
    pub fn invalid_identifier(name: &str) -> Self {
        LexerError {
            error_msg: format!("Invalid identifier - '{}'", name)
        }
    }

    pub fn message(&self) -> &str {
        self.error_msg.as_str()
    }
}

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
            ttype,
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

    pub fn token_type(&self, token: &str) -> Result<TokenType, LexerError> {
        if let Some(toktyp) = self.table.get(token) {
            Ok(*toktyp)
        } else if token.parse::<f64>().is_ok() {
            Ok(TokenType::Literal)
        } else if Self::is_valid_identifier(token) {
            Ok(TokenType::Identifier)
        } else {
            Err(LexerError::invalid_identifier(token))
        }
    }

    pub fn tokenize(&mut self, expr: &str) -> Result<(), LexerError> {
        for tok in expr.split_whitespace() {
            self.tokens.push(Token::new(self.token_type(tok)?, tok));
        }
        Ok(())
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if !self.tokens.is_empty() {
            Some(self.tokens.remove(0))
        } else {
            None
        }
    }

    pub fn peek_token(&self) -> Option<&Token> {
        if !self.tokens.is_empty() {
            Some(&self.tokens[0])
        } else {
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

    fn is_valid_identifier(token: &str) -> bool {
        token.starts_with(char::is_alphabetic) && token.find(|c: char| !c.is_alphanumeric() && c != '_').is_none()
    }

    fn make_token_types() -> HashMap<String, TokenType> {
        let mut table: HashMap<String, TokenType> = HashMap::new();

        for sym in keywords::binary_ops().iter() {
            table.insert(String::from(*sym), TokenType::BinaryOp);
        }

        for sym in keywords::unary_ops().iter() {
            table.insert(String::from(*sym), TokenType::UnaryOp);
        }

        for sym in keywords::constants().iter() {
            table.insert(String::from(*sym), TokenType::Const);
        }

        table.insert(String::from(keywords::TRUE), TokenType::Literal);
        table.insert(String::from(keywords::FALSE), TokenType::Literal);
        table.insert(String::from(keywords::DEFVAR), TokenType::Define);
        table.insert(String::from(keywords::SETVAR), TokenType::Assign);

        table
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

// --------------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_identifier() {
        assert!(Lexer::is_valid_identifier("a"));
        assert!(Lexer::is_valid_identifier("a_"));
        assert!(Lexer::is_valid_identifier("a__"));

        assert!(!Lexer::is_valid_identifier("_"));
        assert!(!Lexer::is_valid_identifier("_a"));

        assert!(!Lexer::is_valid_identifier("foo-bar"));
        assert!(!Lexer::is_valid_identifier("foo!"));
        assert!(!Lexer::is_valid_identifier("@foo"));
        assert!(!Lexer::is_valid_identifier("$foo"));
    }

    #[test]
    fn test_lexer_token_type() {
        let lexer = Lexer::new();

        for sym in keywords::binary_ops().iter() {
            assert_eq!(lexer.token_type(sym).unwrap(), TokenType::BinaryOp);
        }

        for sym in keywords::unary_ops().iter() {
            assert_eq!(lexer.token_type(sym).unwrap(), TokenType::UnaryOp);
        }

        for sym in keywords::constants().iter() {
            assert_eq!(lexer.token_type(sym).unwrap(), TokenType::Const);
        }

        assert_eq!(lexer.token_type(keywords::DEFVAR).unwrap(), TokenType::Define);
        assert_eq!(lexer.token_type(keywords::SETVAR).unwrap(), TokenType::Assign);
        assert_eq!(lexer.token_type(keywords::TRUE).unwrap(), TokenType::Literal);
        assert_eq!(lexer.token_type(keywords::FALSE).unwrap(), TokenType::Literal);
        assert_eq!(lexer.token_type("5.0").unwrap(), TokenType::Literal);
        assert_eq!(lexer.token_type("foobar").unwrap(), TokenType::Identifier);
    }

    #[test]
    fn test_lexer_tokenize() {
        fn test_a_plus5(lexer: &mut Lexer, a_plus5: &str) {
            lexer.tokenize(a_plus5).unwrap();
            assert_eq!(lexer.next_token().unwrap(), Token::new(TokenType::BinaryOp, "+"));
            assert_eq!(lexer.next_token().unwrap(), Token::new(TokenType::Identifier, "a"));
            assert_eq!(lexer.next_token().unwrap(), Token::new(TokenType::Literal, "5"));
            assert!(lexer.next_token().is_none());
        }

        let mut lexer = Lexer::new();

        test_a_plus5(&mut lexer, "+ a 5");
        test_a_plus5(&mut lexer, "+  a  5 \n");
        test_a_plus5(&mut lexer, "\n+ \n  a  5 \n");

        lexer.tokenize("+ 10 5").unwrap();
        assert_eq!(*lexer.peek_token().unwrap(), Token::new(TokenType::BinaryOp, "+"));
        assert!(!lexer.is_empty());

        lexer.clear();
        assert!(lexer.peek_token().is_none());
        assert!(lexer.is_empty());
    }
}
