use std::env;

use crate::custom_functions;

use {custom_functions::preprocess::preprocess, custom_functions::shunting_yard::infix_to_postfix};
pub struct Program {
    pub expression: String,
}

impl Program {
    pub fn new() -> Self {
        Program {
            expression: Program::handle_args(),
        }
    }
    fn handle_args() -> String {
        let all_args: Vec<String> = env::args().skip(1).collect();
        let mut expr: String = String::new();
        for arg in &all_args {
            expr.push_str(arg);
        }
        return expr;
    }

    pub fn run(&self) {}

    pub fn test() {
        let tests = [
            ("3 * 4", "3 4 * "),
            ("12*sin(23)", "12 23 sin * "),
            (
                "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3",
                "3 4 2 * 1 5 - 2 3 ^ ^ / + ",
            ),
            (
                "sin ( max ( 2, 3 ) ÷ 3 × π )",
                "2 3 max 3 / 3.141592653589793 * sin ",
            ),
            ("21 * (214 / 2) >= 7^3", "21 214 2 / * 7 3 ^ ⪖ "),
        ];

        println!("Testing:");
        for (i, test) in tests.iter().enumerate() {
            println!("#{}:  {}", i + 1, &preprocess(&test.0.to_string()));
        }
        println!();

        for (i, test) in tests.iter().enumerate() {
            let infix: &'static str = test.0;
            let postfix: &'static str = test.1;

            let mut check: String = infix_to_postfix(&infix.to_string());

            assert_eq!(check, postfix.to_string());
            println!("test #{}\n infix: {infix}\n postfix: {postfix}\n", i + 1)
        }
    }
}
