#![allow(unused_variables, dead_code, unused_imports, unused_mut)]
mod custom_functions;
mod program;


use custom_functions::shunting_yard;
mod tests;
use crate::tests::*;


static DELTA: f64 = 0.00001;
fn main() {
    program::run();
   
}
