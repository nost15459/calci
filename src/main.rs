use std::io::{self, Write};

fn read_input() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn parse_input(input: &str) -> Option<Input> {
    let mut iter = input.chars();
    let c = iter.next()?;
    if c == ':' {
        let cmd_input: String = iter.collect();
        parse_command(&cmd_input).map(Input::Command)
    } else {
        let expr_input: String = std::iter::once(c).chain(iter).collect();
        parse_expression(&expr_input).map(Input::Expression)
    }
}

fn parse_command(input: &str) -> Option<Command> {
    match input {
        "q" | "quit" => Some(Command::Quit),
        _ => None,
    }
}

fn parse_expression(input: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    for t in input.split_ascii_whitespace() {
        if let Ok(num) = t.parse::<i32>() {
            tokens.push(Token::Number(num));
        } else {
            match t {
                "+" => tokens.push(Token::Operator(Operator::Add)),
                "-" => tokens.push(Token::Operator(Operator::Sub)),
                "*" => tokens.push(Token::Operator(Operator::Mul)),
                "/" => tokens.push(Token::Operator(Operator::Div)),
                _ => return None,
            }
        }
    }
    Some(tokens)
}

fn eval(tokens: &[Token]) -> Option<i32> {
    let mut stack = Vec::new();
    for i in tokens {
        match i {
            Token::Number(num) => {
                stack.push(*num);
            }
            Token::Operator(op) => {
                let lhs = stack.pop()?;
                let rhs = stack.pop()?;
                let result = match op {
                    Operator::Add => rhs + lhs,
                    Operator::Sub => rhs - lhs,
                    Operator::Mul => rhs * lhs,
                    Operator::Div => rhs / lhs,
                };
                stack.push(result);
            }
        }
    }
    stack.pop()
}

fn prompt() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    read_input()
}

enum Input {
    Expression(Vec<Token>),
    Command(Command),
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Quit,
}

enum Token {
    Number(i32),
    Operator(Operator),
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    loop {
        let input = prompt();
        let Some(input) = parse_input(&input) else {
            println!("invailed input");
            continue;
        };
        match input {
            Input::Expression(tokens) => {
                let Some(result) = eval(&tokens) else {
                    println!("invailed expression");
                    continue;
                };
                println!("{}", result);
            }
            Input::Command(cmd) => {
                if cmd == Command::Quit {
                    break;
                }
            }
        }
    }
}
