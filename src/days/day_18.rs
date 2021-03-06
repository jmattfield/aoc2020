use std::iter::Peekable;
use std::slice::Iter;
use std::str::Chars;

pub fn part_one(data: &[&str]) {
    let sum: i64 = data.iter().map(|s| evaluate(s, false)).sum();
    println!("Sum: {}", sum);
}

pub fn part_two(data: &[&str]) {
    let sum: i64 = data.iter().map(|s| evaluate(s, true)).sum();
    println!("Sum: {}", sum);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Number(i64),
    Op(Operation),
    OpenParen,
    CloseParen,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

fn parse(expression: &str) -> Vec<Token> {
    let mut result = vec![];

    let mut iter = expression.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                result.push(read_number(&mut iter));
            }
            '+' => {
                result.push(Token::Op(Operation::Add));
                iter.next();
            }
            '*' => {
                result.push(Token::Op(Operation::Multiply));
                iter.next();
            }
            '(' => {
                result.push(Token::OpenParen);
                iter.next();
            }
            ')' => {
                result.push(Token::CloseParen);
                iter.next();
            }
            _ => {
                iter.next();
            }
        }
    }

    result
}

fn read_number(iter: &mut Peekable<Chars>) -> Token {
    let mut token = String::new();

    while let Some(&c) = iter.peek() {
        if c.is_numeric() {
            token.push(c);
            iter.next();
        } else {
            break;
        }
    }

    let result: i64 = token.parse().unwrap();

    Token::Number(result)
}

fn evaluate(expression: &str, use_precedence: bool) -> i64 {
    let tokens = parse(expression);
    let mut iter = &mut tokens[..].iter().peekable();
    process(&mut iter, use_precedence, false)
}

fn process(iter: &mut Peekable<Iter<Token>>, use_precedence: bool, has_paren: bool) -> i64 {
    let mut stack: Vec<Token> = vec![];

    while let Some(&token) = iter.peek() {
        match token {
            Token::OpenParen => {
                stack.push(*token);
                iter.next();
                let t = Token::Number(process(iter, use_precedence, true));
                stack.push(t);
            }
            Token::CloseParen => {
                if has_paren {
                    iter.next();
                }
                break;
            }
            Token::Op(Operation::Multiply) => {
                if use_precedence {
                    stack.push(*token);
                    iter.next();
                    let t = Token::Number(process(iter, use_precedence, false));
                    stack.push(t);
                } else {
                    stack.push(*token);
                    iter.next();
                }
            }
            _ => {
                stack.push(*token);
                iter.next();
            }
        }
    }

    let mut result = 0;
    let mut current_op: Option<Operation> = None;

    for token in stack {
        match token {
            Token::Number(number) => {
                if let Some(op) = current_op {
                    match op {
                        Operation::Add => result += number,
                        Operation::Multiply => result *= number,
                    }
                } else {
                    result = number;
                }
            }
            Token::Op(op) => current_op = Some(op),
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_18_parses_simple_expression() {
        let tokens = vec![
            Token::Number(123),
            Token::Op(Operation::Add),
            Token::Number(345),
        ];

        assert_eq!(tokens, parse("123 + 345"));
    }

    #[test]
    fn day_18_parses_long_expression() {
        let tokens = vec![
            Token::Number(123),
            Token::Op(Operation::Add),
            Token::Number(345),
            Token::Op(Operation::Multiply),
            Token::Number(3),
            Token::Op(Operation::Add),
            Token::Number(42),
            Token::Op(Operation::Add),
            Token::Number(1),
            Token::Op(Operation::Multiply),
            Token::Number(234),
        ];

        assert_eq!(tokens, parse("123 + 345 * 3 + 42 + 1 * 234"));
    }

    #[test]
    fn day_18_parses_expression_with_parentheses() {
        let tokens = vec![
            Token::Number(123),
            Token::Op(Operation::Add),
            Token::OpenParen,
            Token::Number(345),
            Token::Op(Operation::Multiply),
            Token::Number(3),
            Token::Op(Operation::Add),
            Token::Number(42),
            Token::CloseParen,
            Token::Op(Operation::Add),
            Token::Number(1),
            Token::Op(Operation::Multiply),
            Token::Number(234),
        ];

        assert_eq!(tokens, parse("123 + (345 * 3 + 42) + 1 * 234"));
    }

    #[test]
    fn day_18_parses_expression_with_nested_parentheses() {
        let tokens = vec![
            Token::Number(123),
            Token::Op(Operation::Add),
            Token::OpenParen,
            Token::Number(345),
            Token::Op(Operation::Multiply),
            Token::OpenParen,
            Token::Number(3),
            Token::Op(Operation::Add),
            Token::Number(42),
            Token::CloseParen,
            Token::CloseParen,
            Token::Op(Operation::Add),
            Token::Number(1),
            Token::Op(Operation::Multiply),
            Token::Number(234),
        ];

        assert_eq!(tokens, parse("123 + (345 * (3 + 42)) + 1 * 234"));
    }

    #[test]
    fn day_18_evaluates_simple_expression() {
        assert_eq!(5, evaluate("2 + 3", false))
    }

    #[test]
    fn day_18_evaluates_long_expression() {
        assert_eq!(71, evaluate("1 + 2 * 3 + 4 * 5 + 6", false))
    }

    #[test]
    fn day_18_evaluates_long_expressions_with_parentheses() {
        assert_eq!(26, evaluate("2 * 3 + (4 * 5)", false));
        assert_eq!(437, evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", false));
    }

    #[test]
    fn day_18_evaluates_expressions_with_nested_parentheses() {
        assert_eq!(51, evaluate("1 + (2 * 3) + (4 * (5 + 6))", false));
        assert_eq!(
            12240,
            evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", false)
        );
        assert_eq!(
            13632,
            evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", false)
        );
    }

    #[test]
    fn day_18_evaluates_expressions_with_precedence() {
        // assert_eq!(231, evaluate("1 + 2 * 3 + 4 * 5 + 6", true));
        // assert_eq!(51, evaluate("1 + (2 * 3) + (4 * (5 + 6))", true));
        // assert_eq!(46, evaluate("2 * 3 + (4 * 5)", true));
        // assert_eq!(1445, evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", true));
        // assert_eq!(
        //     669060,
        //     evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", true)
        // );
        assert_eq!(
            23340,
            evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true)
        );
    }
}
