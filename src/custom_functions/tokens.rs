use {crate::custom_functions, std::f64::consts::PI};

use custom_functions::preprocess::preprocess;

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
    arg_amount: u8,
    function: fn(arg1: Number, arg2: Number) -> Number,
}
impl Function {
    fn new(name: String, arg_amount: u8, function: fn(Number, Number) -> Number) -> Token {
        Token::Func(Function {
            name,
            arg_amount,
            function,
        })
    }

    fn apply(&self, arg1: Number, arg2: Number) -> Number {
        (self.function)(arg1, arg2)
    }
}
fn tokenize_operator(input: char) -> Token {
    match input {
        '&' => Operator::new('&', 0, false, |x, y| f64::from((x != 0.) && (y != 0.))),
        '|' => Operator::new('|', 0, false, |x, y| f64::from((x != 0.) || (y != 0.))),
        '!' => Operator::new('!', 0, false, |x, y| f64::from(x == 0.)),

        '≠' => Operator::new('≠', 1, true, |x, y| f64::from(x != y)), //  !=
        '⪖' => Operator::new('⪖', 1, true, |x, y| f64::from(x >= y)), //  >=
        '⪕' => Operator::new('⪕', 1, true, |x, y| f64::from(x <= y)), //  <=
        '>' => Operator::new('>', 1, true, |x, y| f64::from(x > y)),
        '<' => Operator::new('<', 1, true, |x, y| f64::from(x < y)),

        '+' => Operator::new('+', 2, true, |x, y| x + y),
        '-' => Operator::new('-', 2, true, |x, y| x - y),

        '*' => Operator::new('*', 3, true, |x, y| x * y),
        '/' => Operator::new('/', 3, true, |x, y| x / y),
        '%' => Operator::new('%', 3, true, |x, y| x % y),

        '^' => Operator::new('^', 4, false, |x, y| x.powf(y)),
        '~' => Operator::new('~', 4, false, |x, y| -x),

        '(' => Token::LeftParen,
        ')' => Token::RightParen,
        ',' => Token::Comma,
        _ => panic!("Unknown operator {input}"),
    }
}

fn tokenize_function(input: &String) -> Token {
    match input.as_str() {
        "sin" => Function::new(input.to_string(), 1, |x: f64, y| x.sin()),
        "cos" => Function::new(input.to_string(), 1, |x: f64, y| x.cos()),
        "tan" | "tg" | "tang" => Function::new(input.to_string(), 1, |x, y| x.tan()),
        "ctan" | "ctg" | "cot" => Function::new(input.to_string(), 1, |x, y| 1. / x.tan()),

        "sec" | "sc"  => Function::new(input.to_string(), 1, |x: f64, y| 1. / x.sin()),
        "csc" | "csec" | "cosec" | "cosc" => Function::new(input.to_string(), 1, |x: f64, y| 1. / x.cos()),

        "sinh" => Function::new(input.to_string(), 1, |x: f64, y| x.sinh()),
        "cosh" => Function::new(input.to_string(), 1, |x: f64, y| x.cosh()),
        "tanh" | "tgh" | "tangh" => Function::new(input.to_string(), 1, |x, y| x.tanh()),
        "ctanh" | "ctgh" | "coth" => Function::new(input.to_string(), 1, |x, y| 1. / x.tanh()),

        "asin" | "arcsin" => Function::new(input.to_string(), 1, |x: f64, y| x.asin()),
        "acos" | "arccos" => Function::new(input.to_string(), 1, |x: f64, y| x.acos()),
        "atan" | "atg" | "atang" | "arctan" | "arctg" | "arctang" => {
            Function::new(input.to_string(), 1, |x, y| x.atan())
        }
        "actan" | "actg" | "acot" | "arcctan" | "arcctg" | "arccot" => {
            Function::new(input.to_string(), 1, |x, y| PI / 2. - x.atan())
        }

        "asinh" | "arcsinh" => Function::new(input.to_string(), 1, |x: f64, y| x.asinh()),
        "acosh" | "arccosh" => Function::new(input.to_string(), 1, |x: f64, y| x.acosh()),
        "atanh" | "atgh" | "atangh" | "arctanh" | "arctgh" | "arctangh" => {
            Function::new(input.to_string(), 1, |x, y| x.atan())
        }
        "actanh" | "actgh" | "acoth" | "arcctanh" | "arcctgh" | "arccoth" => {
            Function::new(input.to_string(), 1, |x, y| (1. / x).atanh())
        }

        "abs" => Function::new(input.to_string(), 1, |x: f64, y: f64| x.abs()),

        "ln" => Function::new(input.to_string(), 1, |x: f64, y: f64| x.ln()),
        "lg" => Function::new(input.to_string(), 1, |x: f64, y: f64| x.log10()),
        "lb" => Function::new(input.to_string(), 1, |x: f64, y: f64| x.log2()),

        "sqrt" => Function::new(input.to_string(), 1, |x: f64, y: f64| x.sqrt()),
        "cbrt" => Function::new(input.to_string(), 1, |x: f64, y: f64| x.cbrt()),

        "log" => Function::new(input.to_string(), 2, |x: f64, y: f64| x.log(y)),
        "root" | "rt" => Function::new(input.to_string(), 2, |x: f64, y: f64| x.powf(1. / y)),

        "pow" => Function::new(input.to_string(), 2, |x: f64, y: f64| x.powf(y)),
        "max" => Function::new(
            input.to_string(),
            2,
            |x: f64, y: f64| if x > y { x } else { y },
        ),
        "min" => Function::new(
            input.to_string(),
            2,
            |x: f64, y: f64| if x < y { x } else { y },
        ),

        _ => panic!("Unknown function {input}"),
    }
}

fn tokenize(input: &String) -> Vec<Token> {
    fn is_num(c: &char) -> bool {
        ((c >= &'0') && (c <= &'9')) | (c == &'.')
    }
    fn is_op(input: &char) -> bool {
        "`@#><+-*/%^(),⪖⪕≠~'".contains(*input)
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
