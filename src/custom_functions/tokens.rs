use {crate::custom_functions, std::f64::consts::PI};

use custom_functions::preprocess::preprocess;

type Number = f64;
type N = f64;
fn factorial(x: Number) -> Number{
    const FACTORIAL_CEILING: Number = 40.;
    if x < 0.0 || !x.is_sign_positive() {
        panic!("Evaluation error: factorial of {x} - a negative number")
    }
    else if (x - x.floor()).abs() > crate::DELTA {
        panic!("Type error: factorial of {x} - a floating-point number")
    }
    else if x > FACTORIAL_CEILING {
        panic!("Overflow error: factorial of {x} - a number, higher than {FACTORIAL_CEILING}")
    }
    else {
        let n = x as u64;
        (1..=n).fold(1.0, |acc, i| acc * (i as f64))
    }
    
    
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(Number),
    Op(Operator),
    Func(Function),
    LeftParen,
    RightParen,
    Comma,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Operator {
    pub symbol: char,
    pub prec: u8,
    pub is_l_a: bool,
    pub oper: fn(Number, Number) -> Number,
}
impl Operator {
    pub fn new(symbol: char, prec: u8, is_l_a: bool, oper: fn(Number, Number) -> Number) -> Token {
        Token::Op(Operator {
            symbol,
            prec,
            is_l_a,
            oper,
        })
    }

    pub fn apply(&self, x: Number, y: Number) -> Number {
        (self.oper)(x, y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub arg_amount: u8,
    pub function: fn(arg1: Number, arg2: Number) -> Number,
}
impl Function {
    pub fn new(name: String, arg_amount: u8, function: fn(Number, Number) -> Number) -> Token {
        Token::Func(Function {
            name,
            arg_amount,
            function,
        })
    }

    pub fn apply(&self, arg1: Number, arg2: Number) -> Number {
        (self.function)(arg1, arg2)
    }
}

pub fn tokenize_operator(input: char) -> Token {
    match input {
        '&' => Operator::new('&', 0, false, |x, y| f64::from((x != 0.) && (y != 0.))),
        '|' => Operator::new('|', 0, false, |x, y| f64::from((x != 0.) || (y != 0.))),
        '!' => Operator::new('!', 0, false, |x, y| f64::from(x == 0.)),

        '=' => Operator::new('=', 1, true, |x, y| f64::from(x == y)), //  !=
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

pub fn tokenize_function(input: &String) -> Token {
    let function: (u8, fn(x: N, y: N) -> N) = match input.as_str() {
        "sin" => (1, |x: N, y: N| x.sin()),
        "cos" => (1, |x: N, y: N| x.cos()),
        "tan" | "tg" | "tang" => (1, |x: N, y: N| x.tan()),
        "ctan" | "ctg" | "cot" => (1, |x: N, y: N| 1. / x.tan()),

        "sec" | "sc" => (1, |x: N, y: N| 1. / x.sin()),
        "csc" | "csec" | "cosec" | "cosc" => (1, |x: N, y: N| 1. / x.cos()),

        "sinh" => (1, |x: N, y: N| x.sinh()),
        "cosh" => (1, |x: N, y: N| x.cosh()),
        "tanh" | "tgh" | "tangh" => (1, |x: N, y: N| x.tanh()),
        "ctanh" | "ctgh" | "coth" => (1, |x: N, y: N| 1. / x.tanh()),

        "asin" | "arcsin" => (1, |x: N, y: N| x.asin()),
        "acos" | "arccos" => (1, |x: N, y: N| x.acos()),
        "atan" | "atg" | "atang" | "arctan" | "arctg" | "arctang" => (1, |x, y| x.atan()),
        "actan" | "actg" | "acot" | "arcctan" | "arcctg" | "arccot" => {
            (1, |x, y| PI / 2. - x.atan())
        }

        "asinh" | "arcsinh" => (1, |x: N, y: N| x.asinh()),
        "acosh" | "arccosh" => (1, |x: N, y: N| x.acosh()),
        "atanh" | "atgh" | "atangh" | "arctanh" | "arctgh" | "arctangh" => (1, |x, y| x.atan()),
        "actanh" | "actgh" | "acoth" | "arcctanh" | "arcctgh" | "arccoth" => {
            (1, |x, y| (1. / x).atanh())
        }

        "abs" => (1, |x: N, y: N| x.abs()),

        "ln" => (1, |x: N, y: N| x.ln()),
        "lg" => (1, |x: N, y: N| x.log10()),
        "lb" => (1, |x: N, y: N| x.log2()),

        "sqrt" => (1, |x: N, y: N| x.sqrt()),
        "cbrt" => (1, |x: N, y: N| x.cbrt()),

        "fact" => (1, |x: N, y: N| factorial(x)),

        "log" => (2, |x: N, y: N| x.log(y)),
        "root" | "rt" => (2, |x: N, y: N| x.powf(1. / y)),

        "pow" => (2, |x: N, y: N| x.powf(y)),
        "max" => (2, |x: N, y: N| if x > y { x } else { y }),
        "min" => (2, |x: N, y: N| if x < y { x } else { y }),
        
        _ => panic!("Unknown function {input}"),
    };
    Function::new(input.clone(), function.0, function.1)
}

pub fn tokenize(input: &String) -> Vec<Token> {
    fn is_num(c: &char) -> bool {
        ((c >= &'0') && (c <= &'9')) | (c == &'.')
    }
    fn is_op(input: &char) -> bool {
        "`@#><+-*/%^(),⪖⪕≠=~'|&!".contains(*input)
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
