use super::preprocess::preprocess;

type Number = f64;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Num(Number),
    Op(Operator),
    LeftParen,
    RightParen,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Operator {
    symbol: char,
    prec: u8,
    is_l_a: bool,
    is_unary: bool,
    oper: fn(Number, Number) -> Number,
}

impl Operator {
    fn new(
        symbol: char,
        prec: u8,
        is_l_a: bool,
        is_unary: bool,
        oper: fn(Number, Number) -> Number,
    ) -> Token {
        Token::Op(Operator {
            symbol,
            prec,
            is_l_a,
            is_unary,
            oper,
        })
    }

    fn apply(&self, x: Number, y: Number) -> Number {
        (self.oper)(x, y)
    }
}

fn tokenize(input: &String) -> Vec<Token> {
    fn is_num(c: &char) -> bool {
        ((c >= &'0') && (c <= &'9')) | (c == &'.')
    }
    fn tokenize_char(input: char) -> Token {
        match input {
            '`' => Operator::new('`', 1, true, false, |x, y| f64::from(x != y)), //  !=
            '@' => Operator::new('@', 1, true, false, |x, y| f64::from(x >= y)), //  >=
            '#' => Operator::new('#', 1, true, false, |x, y| f64::from(x <= y)), //  <=
            '>' => Operator::new('>', 1, true, false, |x, y| f64::from(x > y)),
            '<' => Operator::new('<', 1, true, false, |x, y| f64::from(x < y)),

            '+' => Operator::new('+', 2, true, false, |x, y| x + y),
            '-' => Operator::new('-', 2, true, false, |x, y| x - y),

            '*' => Operator::new('*', 3, true, false, |x, y| x * y),
            '/' => Operator::new('/', 3, true, false, |x, y| x / y),
            '%' => Operator::new('%', 3, true, false, |x, y| x % y),

            '^' => Operator::new('^', 4, false, false, |x, y| x.powf(y)),
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            _ => panic!("Unknown operator {input}"),
        }
    }

    let len = input.len();
    let mut buf: String = String::with_capacity(8);

    let mut res: Vec<Token> = Vec::with_capacity(len);
    let s: Vec<char> = input.chars().collect();

    for (i, c) in s.iter().enumerate() {
        if is_num(c) {
            buf.push(*c);
            if i >= len - 1 {
                res.push(Token::Num(buf.parse::<Number>().expect("never")));
                buf.clear();
            } else {
                if !is_num(&s[i + 1]) {
                    res.push(Token::Num(buf.parse::<Number>().expect("never")));
                    buf.clear();
                }
            }
        } else {
            res.push(tokenize_char(*c))
        }
    }
    return res;
}

pub fn run() {
    println!("shunting yard:");
    let a = tokenize(&preprocess(&"12((2.3%3).3 + 12.2) - 17.(12)".to_string()));
    for x in a {
        if let Token::Num(value) = x {
            print!("{}", value)
        }
        if let Token::Op(operator) = x {
            print!("{}", operator.symbol)
        }
        if let Token::LeftParen = x {
            print!("(")
        }
        if let Token::RightParen = x {
            print!(")")
        }
    }
    println!();
}
