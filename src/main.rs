#![allow(unused_variables, dead_code, unused_imports, unused_mut)]
mod program;
mod custom_types;
mod custom_functions;

use custom_functions::parse::parse;

fn main() { 
    let pr = program::Program::new();
    // println!("{:?}\n{}\n", pr.all_args, pr.expression);


    println!("{}", parse(pr.expression));
}