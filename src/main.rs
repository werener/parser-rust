#![allow(unused_variables, dead_code, unused_imports)]

use std::env;

mod program;
use program::Program;

mod flags;
use flags::Flag;

mod stack;
use stack::Stack;

fn main() { 
    let pr = Program::new();
    println!("{:?}\n{}\n", pr.all_args, pr.expression)
}