#![allow(unused_variables, dead_code, unused_imports, unused_mut)]
mod custom_functions;
mod custom_types;
mod program;

use custom_functions::shunting_yard;


fn main() {
    let pr = program::Program::new();
    
    shunting_yard::run();
}
