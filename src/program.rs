use std::env;

use crate::custom_functions::{self, shunting_yard::eval};

use {custom_functions::preprocess::preprocess, custom_functions::shunting_yard::infix_to_postfix};

fn handle_args() -> String {
    let all_args: Vec<String> = env::args().skip(1).collect();
    let mut expr: String = String::new();
    for arg in &all_args {
        expr.push_str(arg);
    }
    return expr;
}

pub fn run() {
    let (result, result_is_bool) = eval(&handle_args());
    let r = (result * 10_000_000_f64).round() / 10_000_000_f64;
    if result_is_bool {
        println!("{}", result == 1.);
    } 
    else {
        if r == 0.{
            println!("0");
        }
        else {
            println!("{r}");
        }
    };
}

