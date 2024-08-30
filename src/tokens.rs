use crate::TVal;

pub trait Token {
    fn is(&self, token: &str) -> bool;

    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType;
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Comma,
    Semicolon,
    Eq,
    FunctionDeclaration,
    ClassDeclaration,
    PhpOpenTag,
    Quote,
    VarName(String),
    Scalar(TVal),
    ArrayOpen,
    ArrayClose,
    ParOpen,
    ParClose,
    FunctionCall(String),
}

pub fn get_tokens() -> Vec<Box<dyn Token>> {
    let lang_tokens: Vec<Box<dyn Token>> = vec![
        Box::new(TokenComma {}),
        Box::new(TokenEq {}),
        Box::new(TokenPhpOpenTag {}),
        Box::new(TokenVar {}),
        Box::new(TokenSemicolon {}),
        Box::new(TokenQuote {}),
        Box::new(TokenArrayOpen {}),
        Box::new(TokenArrayClose {}),
        Box::new(TokenScalar {}),
        Box::new(TokenParOpen {}),
        Box::new(TokenParClose {}),
    ];

    lang_tokens
}

pub struct TokenComma {}
pub struct TokenEq {}
pub struct TokenPhpOpenTag {}
pub struct TokenVar {}
pub struct TokenSemicolon {}
pub struct TokenQuote {}
pub struct TokenScalar {}
pub struct TokenArrayOpen {}
pub struct TokenArrayClose {}
pub struct TokenParClose {}
pub struct TokenParOpen {}

impl Token for TokenParClose {
    fn is(&self, token: &str) -> bool {
        token == ")"
    }

    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType {
        TokenType::ParClose
    }
}

impl Token for TokenParOpen {
    fn is(&self, token: &str) -> bool {
        token == "("
    }

    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType {
        TokenType::ParOpen
    }
}

impl Token for TokenArrayClose {
    fn is(&self, token: &str) -> bool {
        token == "]"
    }

    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType {
        TokenType::ArrayClose
    }
}

impl Token for TokenArrayOpen {
    fn is(&self, token: &str) -> bool {
        token == "["
    }

    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType {
        TokenType::ArrayOpen
    }
}

impl Token for TokenScalar {
    fn is(&self, token: &str) -> bool {
        match token.parse::<i64>() {
            Ok(_) => true,
            Err(_) => {
                match token.parse::<f64>() {
                    Ok(_) => true,
                    Err(_) => false
                }
            }
        }
    }

    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType {
        match token.parse::<i64>() {
            Ok(i64) => TokenType::Scalar(TVal::from(i64)),
            Err(_) => {
                match token.parse::<f64>() {
                    Ok(f64) => TokenType::Scalar(TVal::from(f64)),
                    Err(_) => TokenType::Scalar(TVal::from(token))
                }
            }
        }
    }
}

impl Token for TokenQuote {
    fn is(&self, token: &str) -> bool {
        token == "'" || token == "\""
    }

    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType {
        TokenType::Quote
    }
}

impl Token for TokenSemicolon {
    fn is(&self, token: &str) -> bool {
        token == ";"
    }

    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType {
        TokenType::Semicolon
    }
}

impl Token for TokenVar {
    fn is(&self, token: &str) -> bool {
        token.len() > 1 && token.chars().next().unwrap() == '$'
    }

    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType {
        let var_name = &token[1..];
        TokenType::VarName(String::from(var_name))
    }
}

impl Token for TokenPhpOpenTag {
    fn is(&self, token: &str) -> bool {
        token == "<?php"
    }

    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType {
        TokenType::PhpOpenTag
    }
}

impl Token for TokenComma {
    fn is(&self, token: &str) -> bool {
        token.chars().next().unwrap_or('.') == ','
    }
    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType {
        TokenType::Comma
    }
}

impl Token for TokenEq {
    fn is(&self, token: &str) -> bool {
        token.chars().next().unwrap_or('.') == '='
    }

    fn process(&self, token: &str, tokens: &Vec<String>) -> TokenType {
        TokenType::Eq
    }
}