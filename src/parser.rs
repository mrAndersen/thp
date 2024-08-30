use std::fmt::{Debug, Formatter};
use std::time;
use regex::Regex;
use crate::now;
use crate::tokens::{get_tokens, Token, TokenArrayClose, TokenArrayOpen, TokenComma, TokenEq, TokenParClose, TokenParOpen, TokenPhpOpenTag, TokenQuote, TokenScalar, TokenSemicolon, TokenType, TokenVar};
use time::SystemTime;
use crate::functions::get_functions;

pub struct Code {
    tokens: Vec<TokenType>,
    code: Vec<String>,
    current: usize,
    tokenize_time_ms: f64,
}

impl Debug for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}\n{}ms {:?}", self.code, self.tokenize_time_ms, self.tokens).as_str())
    }
}

impl Code {
    fn next(&mut self) {
        self.current += 1;
    }

    fn is_end(&self) -> bool {
        self.current < self.tokens.len()
    }

    pub fn eval(&self) {
        for (i, token) in self.tokens.iter().enumerate() {}
    }

    fn split(input: &str) -> Vec<String> {
        let re = Regex::new(r#"\s+|\n|\(|\)|;|,|'|""#).unwrap();
        let mut result = Vec::new();
        let mut last_end = 0;

        for mat in re.find_iter(input) {
            if mat.start() != last_end {
                result.push(input[last_end..mat.start()].to_string());
            }

            result.push(mat.as_str().to_string());
            last_end = mat.end();
        }

        if last_end < input.len() {
            result.push(input[last_end..].to_string());
        }

        result.iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub fn tokenize(code: String) -> Code {
        let start = now!();

        let mut result = vec![];
        let mut current = 0;
        let lang_tokens = get_tokens();
        let lang_functions = get_functions();
        let tokens: Vec<String> = Self::split(code.as_str());

        while current != tokens.len() {
            let token_str = &tokens[current];
            current += 1;
            let mut token_found = false;

            for lang_token in &lang_tokens {
                if lang_token.is(token_str.as_str()) {
                    result.push(lang_token.process(token_str.as_str(), &tokens));
                    token_found = true
                }
            }

            if !token_found {

            }
        }

        Code {
            tokens: result,
            current: 0,
            code: tokens,
            tokenize_time_ms: now!().duration_since(start).unwrap().as_micros() as f64 / 1000.0,
        }
    }
}