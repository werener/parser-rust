// infix -> postfix via shunting yard
use crate::custom_types;
use custom_types::stack::Stack;
use custom_types::queue::Queue;

pub fn parse(s: String) -> String {
    let mut parsed = String::new();
    let mut queue: Queue<String> = Queue::new();
    let mut stack: Stack<String> = Stack::new();
    let mut tokens: String = String::new();


    return parsed;
}