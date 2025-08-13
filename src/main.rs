#![allow(unused_variables, dead_code, unused_imports, unused_mut)]
mod custom_functions;
mod program;

use custom_functions::shunting_yard;


fn main() {
    let prog = program::Program::new();
    program::Program::test();
}
