use {crate::custom_functions, std::f64::consts::PI};
use custom_functions::tokens;

use {tokens::Token, tokens::Operator, tokens::Function};
use custom_functions::preprocess::preprocess;

type Number = f64;

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
                    else if let Token::Func(o2) = stack.last().expect("") {
                        output.push(stack.pop().expect(""));
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

pub fn infix_to_postfix(infix: &String) -> String {
    let postfix: Vec<Token> = shunting_yard(tokens::tokenize(&preprocess(&infix.to_string())));
    let mut res: String = String::new();

    for token in postfix {
        let addition = match token {
            Token::Num(value) => value.to_string(),
            Token::Op(operator) => operator.symbol.to_string(),
            Token::Comma => ",".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::Func(function) => function.name,
        };
        res += addition.as_str();
        res += " "
    }
    return res;
}

fn evaluate_postfix(expression: Vec<Token>) -> (Number, bool) {
    let mut stack: Vec<Number> = Vec::new();
    let mut result_is_boolean: bool = false;
    for token in expression {
        match token {
            Token::Num(n) => stack.push(n),
            Token::Op(operator) => {
                match operator.symbol {
                    // unary
                    '~' | '!' => {
                        let operand = stack.pop();
                        match operand {
                            Some(v) => stack.push(operator.apply(v, 0.0)),
                            None => {
                                panic!("Evaluation error: unary operator '{}' doesn't have an operand", operator.symbol)
                            }
                        }
                    }
                    // binary
                    _ => {
                        let op_right = stack.pop();
                        let op_left = stack.pop();
                        match op_left {
                            Some(op_left) => {
                                stack.push(operator.apply(op_left, op_right.expect("")));
                            }
                            None => {

                                panic!("Evaluation error: binary operator '{}' lacks an operand", operator.symbol)
                            }
                        }
                    }
                };
                if !result_is_boolean & "|&!≠⪖⪕><".contains(operator.symbol) {
                    result_is_boolean = true;
                }
            }
            Token::Func(func) => match func.arg_amount {
                1 => {
                    let arg = stack.pop();

                    match arg {
                        Some(arg) => stack.push(func.apply(arg, 0.)),
                        None => {
                            panic!("Evaluation error: function '{}' lacks an argument", func.name)
                        }
                    }
                }
                2 => {
                    let arg1 = stack.pop();
                    let arg2 = stack.pop();
                    match arg2 {
                        Some(arg2) => stack.push(func.apply(arg1.expect(""), arg2)),
                        None => {
                            panic!("Evaluation error: function '{}' needs 2 arguments", func.name)
                        }
                    }
                }
                _ => {}
            },
            _ => panic!("Unsupported token in postfix"),
        }
    }
    match stack.pop() {
        Some(v) => return (v, result_is_boolean),
        None => panic!("Unsupported expression! Couldn't evaluate"),
    }
}

pub fn eval(s: &str) -> (Number, bool) {
    return evaluate_postfix(shunting_yard(tokens::tokenize(&preprocess(&s.to_string()))));
}
