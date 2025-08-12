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
        _ => panic!("Unknown operator {input}"),
    }
}

fn tokenize_function(input: &String) -> Token{
    match input.as_str() {
        "sin"  => Function::new(input.to_string(), |x| x.sin()),
        _ => panic!("Unknown function {input}"),
    }
}


fn tokenize(input: &String) -> Vec<Token> {
    fn is_num(c: &char) -> bool {
        ((c >= &'0') && (c <= &'9')) | (c == &'.')
    }
    fn is_op(input: &char) -> bool {
        "`@#><+-*/%^()".contains(*input)
    }
    // fn is_char(c: &char) -> bool {
    //     ((c >= &'a') && (c <= &'z')) | ((c >= &'A') && (c <= &'Z'))
    // }

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
    return res;
}

pub fn run() {
    println!("shunting yard:");
    let a = tokenize(&preprocess(&"12*sin(3)".to_string()));
    for x in a {
        if let Token::Num(value) = x {
            print!("{}", value);
        }
        if let Token::Op(operator) = x {
            print!("{}", operator.symbol)
        }
        if let Token::LeftParen = x {
            print!("(");
        }
        if let Token::RightParen = x {
            print!(")");
        }
        if let Token::Func(function) = x {
            print!("{}", function.name);
        }
    }
    println!();
}
