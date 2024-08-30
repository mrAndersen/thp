use std::collections::HashMap;
use crate::TVal;

pub trait Function<'a> {
    fn call(&self, args: Vec<TVal>) -> Option<TVal>;

    fn name(&self) -> String;
}


pub struct FunctionEcho {}

fn req_args(fn_name: &String, args: &Vec<TVal>, req_count: usize) {
    if args.len() < req_count {
        println!("Function {} requires at lease {} arguments", fn_name, req_count)
    }
}

pub fn get_functions() -> HashMap<String, Box<dyn Function>> {
    let list: Vec<Box<dyn Function>> = vec![
        Box::new(FunctionEcho {})
    ];

    let mut result = HashMap::new();

    list.iter().for_each(|n| {
        result.insert(n.name(), n.clone());
    });

    result
}

impl<'a> Function<'a> for FunctionEcho {
    fn call(&self, args: Vec<TVal>) -> Option<TVal> {
        req_args(&self.name(), &args, 1);
        println!("{}", args.first().unwrap());
        None
    }

    fn name(&self) -> String {
        "echo".to_string()
    }
}