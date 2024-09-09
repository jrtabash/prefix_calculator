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

    pub fn reserved_name(what: &str, name: &str) -> Self {
        LexerError {
            error_msg: format!("Invalid reserved {} - '{}'", what, name)
        }
    }

    pub fn message(&self) -> &str {
        self.error_msg.as_str()
    }
}

// --------------------------------------------------------------------------------
// TokenType

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    BinaryOp,
    UnaryOp,
    SpecialFtn,
    Literal,
    Const,
    Define,
    Assign,
    Identifier,
    Defun,
    Funcall,
    Begin,
    End,
    CEnd,
    If,
    Then,
    Else,
    Fi
}

impl TokenType {
    pub fn to_string(&self) -> &str {
        match self {
            TokenType::BinaryOp => "BinaryOp",
            TokenType::UnaryOp => "UnaryOp",
            TokenType::SpecialFtn => "SpecialFtn",
            TokenType::Literal => "Literal",
            TokenType::Const => "Const",
            TokenType::Define => "Define",
            TokenType::Assign => "Assign",
            TokenType::Identifier => "Identifier",
            TokenType::Defun => "Defun",
            TokenType::Funcall => "Funcall",
            TokenType::Begin => "Begin",
            TokenType::End => "End",
            TokenType::CEnd => "CEnd",
            TokenType::If => "If",
            TokenType::Then => "Then",
            TokenType::Else => "Else",
            TokenType::Fi => "Fi"
        }
    }
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

    #[inline(always)]
    pub fn is_reserved(&self, name: &str) -> bool {
        self.table.get(name).is_some()
    }

    #[inline(always)]
    pub fn check_reserved(&self, tok: &Token, what: &str) -> Result<(), LexerError> {
        if self.is_reserved(&tok.tname) {
            return Err(LexerError::reserved_name(what, &tok.tname));
        }
        Ok(())
    }

    #[inline(always)]
    pub fn starts_with(&self, ttype: TokenType) -> bool {
        !self.tokens.is_empty() && self.tokens[0].ttype == ttype
    }

    #[inline(always)]
    pub fn ends_with(&self, ttype: TokenType) -> bool {
        !self.tokens.is_empty() && self.tokens[self.tokens.len() - 1].ttype == ttype
    }

    #[inline(always)]
    pub fn contains(&self, ttype: TokenType) -> bool {
        !self.tokens.is_empty() && self.tokens.iter().any(|t| t.ttype == ttype)
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.tokens.clear();
    }

    #[inline(always)]
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

        for sym in keywords::binary_ops() {
            table.insert(String::from(sym), TokenType::BinaryOp);
        }

        for sym in keywords::unary_ops() {
            table.insert(String::from(sym), TokenType::UnaryOp);
        }

        for sym in keywords::special_ftns() {
            table.insert(String::from(sym), TokenType::SpecialFtn);
        }

        for sym in keywords::constants() {
            table.insert(String::from(sym), TokenType::Const);
        }

        table.insert(String::from(keywords::TRUE), TokenType::Literal);
        table.insert(String::from(keywords::FALSE), TokenType::Literal);
        table.insert(String::from(keywords::DEFVAR), TokenType::Define);
        table.insert(String::from(keywords::SETVAR), TokenType::Assign);
        table.insert(String::from(keywords::DEFUN), TokenType::Defun);
        table.insert(String::from(keywords::FUNCALL), TokenType::Funcall);
        table.insert(String::from(keywords::BEGIN), TokenType::Begin);
        table.insert(String::from(keywords::END), TokenType::End);
        table.insert(String::from(keywords::CEND), TokenType::CEnd);
        table.insert(String::from(keywords::IF), TokenType::If);
        table.insert(String::from(keywords::THEN), TokenType::Then);
        table.insert(String::from(keywords::ELSE), TokenType::Else);
        table.insert(String::from(keywords::FI), TokenType::Fi);

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

        for sym in keywords::binary_ops() {
            assert_eq!(lexer.token_type(sym).unwrap(), TokenType::BinaryOp);
        }

        for sym in keywords::unary_ops() {
            assert_eq!(lexer.token_type(sym).unwrap(), TokenType::UnaryOp);
        }

        for sym in keywords::special_ftns() {
            assert_eq!(lexer.token_type(sym).unwrap(), TokenType::SpecialFtn);
        }

        for sym in keywords::constants() {
            assert_eq!(lexer.token_type(sym).unwrap(), TokenType::Const);
        }

        assert_eq!(lexer.token_type(keywords::DEFVAR).unwrap(), TokenType::Define);
        assert_eq!(lexer.token_type(keywords::SETVAR).unwrap(), TokenType::Assign);
        assert_eq!(lexer.token_type(keywords::DEFUN).unwrap(), TokenType::Defun);
        assert_eq!(lexer.token_type(keywords::FUNCALL).unwrap(), TokenType::Funcall);
        assert_eq!(lexer.token_type(keywords::BEGIN).unwrap(), TokenType::Begin);
        assert_eq!(lexer.token_type(keywords::END).unwrap(), TokenType::End);
        assert_eq!(lexer.token_type(keywords::CEND).unwrap(), TokenType::CEnd);
        assert_eq!(lexer.token_type(keywords::IF).unwrap(), TokenType::If);
        assert_eq!(lexer.token_type(keywords::THEN).unwrap(), TokenType::Then);
        assert_eq!(lexer.token_type(keywords::ELSE).unwrap(), TokenType::Else);
        assert_eq!(lexer.token_type(keywords::FI).unwrap(), TokenType::Fi);
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

    #[test]
    fn test_lexer_search() {
        let tokstr = "def add x y begin + x y end";
        let mut lexer = Lexer::new();
        lexer.tokenize(&tokstr).unwrap();

        assert!(lexer.starts_with(TokenType::Defun));
        assert!(lexer.ends_with(TokenType::End));
        assert!(lexer.contains(TokenType::Defun));
        assert!(lexer.contains(TokenType::BinaryOp));
        assert!(lexer.contains(TokenType::End));

        assert!(!lexer.starts_with(TokenType::Define));
        assert!(!lexer.ends_with(TokenType::Assign));
        assert!(!lexer.contains(TokenType::Define));
        assert!(!lexer.contains(TokenType::UnaryOp));
        assert!(!lexer.contains(TokenType::Assign));
    }
}
