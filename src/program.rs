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
    let res: (f64, bool) = eval(&handle_args());
    if res.1 {
        println!("{:.10}", res.0 == 1.);
    } else {
        println!("{:.10}", res.0);
    };
}

pub fn test(with_postfix: bool) {
    let tests = [
        ("3 * 4", "3 4 * ", 12.),
        ("12*sin(23)", "12 23 sin * ", -10.1546),
        (
            "sin ( max ( 2, 3 ) ÷ 3 × π )",
            "2 3 max 3 / 3.141592653589793 * sin ",
            0.,
        ),
        ("2+3*4", "2 3 4 * + ", 14.),
        ("-2 * (3 + 4)", "2 ~ 3 4 + * ", -14.),
        ("2 / 1", "2 1 / ", 2.),
        ("2^3 * 3", "2 3 ^ 3 * ", 24.),
        ("-2^3 * 3 + 2**2", "2 3 ^ ~ 3 * 2 2 ^ + ", -20.),
        ("17 * -2 + 21**2", "17 2 ~ * 21 2 ^ + ", 407.),
        ("sin(2pi)", "", 0.),
        ("cos(pi/2)sin(2pi)", "", 0.),
        ("cos(pi) ^ 2 + sin(pi) ^ 2", "", 1.),
    ];

    print!("Testing...");
    for (i, test) in tests.iter().enumerate() {
        let infix: &'static str = test.0;
        let preprocessed: String = preprocess(&test.0.to_string());
        let postfix: String = infix_to_postfix(&infix.to_string());
        let result: (f64, bool) = eval(test.0);

        let real_postfix: &'static str = test.1;
        let real_value: f64 = test.2;
        const DELTA: f64 = 0.0001;

        println!("\ntest #{}", i + 1);
        println!("  input: {infix}");
        println!("  preprocessed: {preprocessed}");
        if with_postfix {
            println!("  postfix: {postfix}");
        }

        if result.1 {
            println!("  result: {}", result.0 == 1.);
        } else {
            println!("  result: {}", result.0);
        };
        if with_postfix {
            assert_eq!(postfix, real_postfix.to_string());
            print!("infix_to_postfix ✓\n");
        }

        match (result.0 - real_value).abs() <= DELTA {
            true => {
                print!("evaluation ✓\n")
            }
            false => {
                panic!("result '{}' != {real_value}", result.0)
            }
        }
    }
    println!("{:-<30}", "-");
    println!("All test are completed! ✓")
}
