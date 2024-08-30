#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use std::fmt::{Debug, Display, Formatter, Write};
use std::fs;
use crate::parser::Code;

mod functions;
mod parser;
mod tokens;
mod common;

#[derive(Clone)]
pub struct TVal {
    t_bool: Option<bool>,
    t_int: Option<i64>,
    t_float: Option<f64>,
    t_string: Option<String>,
    ty: Type,
}

impl Debug for TVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for TVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.ty {
            Type::String => f.write_str(format!("{}", self.t_string.as_ref().unwrap()).as_str()),
            Type::Integer => f.write_str(format!("{}", self.t_int.as_ref().unwrap()).as_str()),
            Type::Array => f.write_str("Array"),
        }
    }
}

impl From<i64> for TVal {
    fn from(value: i64) -> Self {
        Self {
            t_bool: None,
            ty: Type::Integer,
            t_int: Some(value),
            t_float: None,
            t_string: None,
        }
    }
}

impl From<f64> for TVal {
    fn from(value: f64) -> Self {
        Self {
            t_bool: None,
            ty: Type::Integer,
            t_int: None,
            t_float: Some(value),
            t_string: None,
        }
    }
}

impl From<&str> for TVal {
    fn from(value: &str) -> Self {
        Self {
            t_bool: None,
            t_int: None,
            t_float: None,
            t_string: Some(value.to_string()),
            ty: Type::String,
        }
    }
}

impl From<String> for TVal {
    fn from(value: String) -> Self {
        Self {
            t_bool: None,
            t_int: None,
            t_float: None,
            t_string: Some(value),
            ty: Type::String,
        }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Type {
    String,
    Integer,
    Array,
}

#[derive(Clone)]
pub struct TArray {
    v: Vec<TVal>,
}

fn main() {
    let source = String::from_utf8(fs::read("tests/one.thp").unwrap()).unwrap();
    let c = Code::tokenize(source);

    println!("{:?}", c);
}