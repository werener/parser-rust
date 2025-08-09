use std::env;

use super::custom_types;
use custom_types::flags::Flag;
use custom_types::stack::Stack;
pub struct Program {
    pub all_args: Vec<String>,
    pub flags: Vec<Flag>,
    pub expression: String,
}

impl Program {
    pub fn new() -> Self {
        let construct: (Vec<String>, Vec<Flag>, String) = Program::handle_args();
        Program {
            all_args: construct.0,
            flags: construct.1,
            expression: construct.2,
        }
    }

    fn handle_args() -> (Vec<String>, Vec<Flag>, String) {
        let all_args: Vec<String> = env::args().skip(1).collect();
        let mut flags: Vec<Flag> = Vec::with_capacity(2);
        let mut buffer: String = String::new();

        for arg in &all_args {
            if arg.starts_with("--") {
                println!("{}", arg);
                match Flag::get_flag(arg) {
                    Flag::Help => flags.push(Flag::Help),
                    Flag::UserInput => flags.push(Flag::UserInput),
                    Flag::Null => panic!("Unknown flag {arg}"),
                };
            } else {
                buffer.push_str(arg);
            }
        }
        return (all_args, flags, buffer);
    }

    fn help() {
        println!("Help lord penius");
        println!("Avaliable flags:")
    }

    fn input() {
        // continuous input mode
    }

    fn run() {}
}
