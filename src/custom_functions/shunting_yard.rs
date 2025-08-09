type Number = f64;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Number(f64),
    Operator(Operator),
    LeftParen,
    RightParen,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Operator {
    token: char,
    precedence: u8,
    is_left_associative: bool,
    operation: fn(Number, Number) -> Number,
}

impl Operator {
    fn new(
        token: char,
        precedence: u8,
        is_left_associative: bool,
        operation: fn(Number, Number) -> Number,
    ) -> Token {
        Token::Operator(Operator {
            token: token,
            precedence: precedence,
            is_left_associative,
            operation: operation,
        })
    }

    fn apply(&self, x: Number, y: Number) -> Number {
        (self.operation)(x, y)
    }
}









pub fn run() {
    let a: Token = Token::Number(13.0);
    if let Token::Number(value) = a {println!("{}", value)}
}