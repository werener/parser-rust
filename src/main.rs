#![allow(unused_variables, dead_code, unused_imports)]
mod program;
mod custom_types;

use program::Program;

fn main() { 
    let pr = Program::new();
    println!("{:?}\n{}\n", pr.all_args, pr.expression)
}