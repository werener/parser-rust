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
    precedence: u8,
    is_left_associative: bool,
    operation: fn(Number, Number) -> Number,
}

impl Operator {
    fn new(
        symbol: char,
        precedence: u8,
        is_left_associative: bool,
        operation: fn(Number, Number) -> Number,
    ) -> Token {
        Token::Op(Operator {
            symbol,
            precedence,
            is_left_associative,
            operation,
        })
    }

    fn apply(&self, x: Number, y: Number) -> Number {
        (self.operation)(x, y)
    }
}

fn tokenize(input: &String) -> Vec<Token> {
    fn is_num(c: &char) -> bool {
        (c >= &'0') && (c <= &'9')
    }
    fn make_token(input: char) -> Token {
        match input {
            '+' => Operator::new('+', 1, true, |x, y| x + y),
            '-' => Operator::new('-', 1, true, |x, y| x - y),
            '*' => Operator::new('*', 2, true, |x, y| x * y),
            '/' => Operator::new('/', 2, true, |x, y| x / y),
            '^' => Operator::new('^', 3, false, |x, y| x.powf(y)),
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            _ => panic!("Unknown operator {input}"),
        }
    }

    let len = input.len();
    let mut buf: String = String::new();

    let mut res: Vec<Token> = Vec::with_capacity(len);
    let s: Vec<char> = input.chars().collect();

    for (i, c) in s.iter().enumerate() {
        if is_num(c) {
            buf.push(*c);
            if (i >= len) | (!is_num(&s[i + 1])) {
                res.push(Token::Num(buf.parse::<Number>().expect("never")));
                buf.clear();
            }
        } else {
            res.push(make_token(*c))
        }
    }
    return res;
}

pub fn run() {
    let a = tokenize(&"123+785*2(13)".to_string());
    for x in a {
        if let Token::Num(value) = x {
            println!("{}", value)
        }
        if let Token::Op(operator) = x {
            println!("{}", operator.symbol)
        }
    }
}
