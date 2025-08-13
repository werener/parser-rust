use crate::custom_types::queue;

use {crate::custom_functions, crate::custom_types, custom_functions::supported_functions as sf};

use custom_functions::preprocess::preprocess;

use {custom_types::queue::Queue, custom_types::stack::Stack};
type Number = f64;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Num(Number),
    Op(Operator),
    Func(Function),
    LeftParen,
    RightParen,
    Comma,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Operator {
    symbol: char,
    prec: u8,
    is_l_a: bool,
    oper: fn(Number, Number) -> Number,
}
impl Operator {
    fn new(symbol: char, prec: u8, is_l_a: bool, oper: fn(Number, Number) -> Number) -> Token {
        Token::Op(Operator {
            symbol,
            prec,
            is_l_a,
            oper,
        })
    }

    fn apply(&self, x: Number, y: Number) -> Number {
        (self.oper)(x, y)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Function {
    name: String,
    function: fn(args: Number) -> Number,
}
impl Function {
    fn new(name: String, function: fn(Number) -> Number) -> Token {
        Token::Func(Function { name, function })
    }

    fn apply(&self, arg: Number) -> Number {
        (self.function)(arg)
    }
}

fn tokenize_operator(input: char) -> Token {
    match input {
        '`' => Operator::new('`', 1, true, |x, y| f64::from(x != y)), //  !=
        '@' => Operator::new('@', 1, true, |x, y| f64::from(x >= y)), //  >=
        '#' => Operator::new('#', 1, true, |x, y| f64::from(x <= y)), //  <=
        '>' => Operator::new('>', 1, true, |x, y| f64::from(x > y)),
        '<' => Operator::new('<', 1, true, |x, y| f64::from(x < y)),

        '+' => Operator::new('+', 2, true, |x, y| x + y),
        '-' => Operator::new('-', 2, true, |x, y| x - y),

        '*' => Operator::new('*', 3, true, |x, y| x * y),
        '/' => Operator::new('/', 3, true, |x, y| x / y),
        '%' => Operator::new('%', 3, true, |x, y| x % y),

        '^' => Operator::new('^', 4, false, |x, y| x.powf(y)),
        '(' => Token::LeftParen,
        ')' => Token::RightParen,
        ',' => Token::Comma,
        _ => panic!("Unknown operator {input}"),
    }
}

fn tokenize_function(input: &String) -> Token {
    match input.as_str() {
        "sin" => Function::new(input.to_string(), |x| x.sin()),
        "cos" => Function::new(input.to_string(), |x| x.cos()),
        "tan" | "tg" | "tang" => Function::new(input.to_string(), |x| x.tan()),
        "ctan" | "ctg" | "cot" => Function::new(input.to_string(), |x| x.cos() / x.sin()),
        _ => panic!("Unknown function {input}"),
    }
}

fn tokenize(input: &String) -> Vec<Token> {
    fn is_num(c: &char) -> bool {
        ((c >= &'0') && (c <= &'9')) | (c == &'.')
    }
    fn is_op(input: &char) -> bool {
        "`@#><+-*/%^(),".contains(*input)
    }

    let len = input.len();
    let mut buf_num: String = String::with_capacity(8);
    let mut buf_func: String = String::with_capacity(4);

    let mut res: Vec<Token> = Vec::with_capacity(len);
    let s: Vec<char> = input.chars().collect();

    for (i, ch) in s.iter().enumerate() {
        // is part of a number
        if is_num(ch) {
            if buf_func != "" {
                res.push(tokenize_function(&buf_func));
                buf_func.clear();
            }
            buf_num.push(*ch);
        }
        // is an operator
        else if is_op(ch) {
            if buf_num != "" {
                res.push(Token::Num(buf_num.parse::<Number>().expect(&buf_num)));
                buf_num.clear();
            }
            if buf_func != "" {
                res.push(tokenize_function(&buf_func));
                buf_func.clear();
            }
            res.push(tokenize_operator(*ch))
        }
        // is part of a function
        else {
            if buf_num != "" {
                res.push(Token::Num(buf_num.parse::<Number>().expect(&buf_num)));
                buf_num.clear();
            }
            buf_func.push(*ch);
        }
    }
    if buf_num != "" {
        res.push(Token::Num(buf_num.parse::<Number>().expect(&buf_num)));
        buf_num.clear();
    }
    if buf_func != "" {
        res.push(tokenize_function(&buf_func));
        buf_func.clear();
    }
    return res;
}

fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Num(_) => output.push(token),
            Token::LeftParen => stack.push(token),
            Token::Func(_) => stack.push(token),
            Token::Comma => {
                while (stack.last() != None) && (stack.last().expect("") != &Token::LeftParen) {
                    output.push(stack.pop().expect(""));
                }
            }
            Token::Op(o1) => {
                while (stack.last() != None) && (stack.last().expect("") != &Token::LeftParen) {
                    if let Token::Op(o2) = stack.last().expect("") {
                        if (o2.prec > o1.prec) || ((o1.prec == o2.prec) && o1.is_l_a) {
                            output.push(stack.pop().expect(""));
                        } else {
                            break;
                        }
                    }
                }
                stack.push(token);
            }
            Token::RightParen => {
                while (stack.last() != None) && (stack.last().expect("") != &Token::LeftParen) {
                    output.push(stack.pop().expect(""));
                }
                assert!((stack.last() != None) && (stack.last().expect("") == &Token::LeftParen));
                stack.pop();
                if stack.last() != None {
                    if let Token::Func(_) = stack.last().expect("") {
                        output.push(stack.pop().expect(""))
                    }
                }
            }
        }
    }
    while stack.last() != None {
        output.push(stack.pop().expect(""));
    }
    return output;
}

pub fn run() {
    let tests = [
        ("3 * 4", "3 4 * "),
        ("12*sin(23)", "12 23 sin * "),
        (
            "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3",
            "3 4 2 * 1 5 - 2 3 ^ ^ / + ",
        ),
    ];

    for test in tests {
        println!("{}", &preprocess(&test.0.to_string()));
        // let b = tokenize(&preprocess(&test.0.to_string()));
        // for x in b {
        //     if let Token::Num(a) = x {
        //         println!("{}", a);
        //     }
        // }
    }
    println!();

    for (i, test) in tests.iter().enumerate() {
        let prefix: &'static str = test.0;
        let postfix: &'static str = test.1;

        let resulting_postfix: Vec<Token> =
            shunting_yard(tokenize(&preprocess(&prefix.to_string())));
        let mut check: String = String::new();

        for token in resulting_postfix {
            let addition = match token {
                Token::Num(value) => value.to_string(),
                Token::Op(operator) => operator.symbol.to_string(),
                Token::Comma => ",".to_string(),
                Token::LeftParen => "(".to_string(),
                Token::RightParen => ")".to_string(),
                Token::Func(function) => function.name,
            };
            check += addition.as_str();
            check += " "
        }
        assert_eq!(check, postfix.to_string());
        println!("test #{} {prefix} -> {postfix}", i + 1)
    }
}
