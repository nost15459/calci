use std::{
    fmt::Display,
    io::{self, Write},
};

fn read_input() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn parse_input(input: &str) -> Option<Input> {
    if input.chars().next()? == ':' {
        parse_command(&input[1..]).map(Input::Command)
    } else {
        parse_expression(input).map(Input::Expression)
    }
}

fn parse_command(input: &str) -> Option<Command> {
    match input {
        "q" | "quit" => Some(Command::Quit),
        "trace" => Some(Command::Trace),
        _ => None,
    }
}

fn parse_expression(input: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    for t in input.split_ascii_whitespace() {
        if let Ok(num) = t.parse::<f32>() {
            tokens.push(Token::Number(num));
        } else {
            match t {
                Operator::ADD => tokens.push(Token::Operator(Operator::Add)),
                Operator::SUB => tokens.push(Token::Operator(Operator::Sub)),
                Operator::MUL => tokens.push(Token::Operator(Operator::Mul)),
                Operator::DIV => tokens.push(Token::Operator(Operator::Div)),
                _ => return None,
            }
        }
    }
    Some(tokens)
}

fn eval(tokens: &[Token]) -> Option<(f32, Vec<Vec<f32>>)> {
    let mut stack = Vec::new();
    let mut history: Vec<Vec<f32>> = vec![];
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
        history.push(stack.clone());
    }
    stack.pop().map(|r| (r, history))
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
    Trace,
}

enum Token {
    Number(f32),
    Operator(Operator),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Operator(op) => write!(f, "{}", op),
        }
    }
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Self::Add => Self::ADD,
            Self::Sub => Self::SUB,
            Self::Mul => Self::MUL,
            Self::Div => Self::DIV,
        };
        write!(f, "{}", display)
    }
}

impl Operator {
    const ADD: &str = "+";
    const SUB: &str = "-";
    const MUL: &str = "*";
    const DIV: &str = "/";
}

fn print_stack_trace(expr: &[Token], stack_history: &[Vec<f32>]) {
    if !stack_history.is_empty() {
        println!("ip: <>");
    }
    for (history_idx, stack) in stack_history.iter().enumerate() {
        print!("stack:[",);
        for (idx, e) in stack.iter().enumerate() {
            if idx == stack.len() - 1 {
                print!("{}", e)
            } else {
                print!("{}, ", e)
            }
        }

        print!("]  expr[");
        for (instruction_idx, ip) in expr.iter().enumerate() {
            if history_idx + 1 == instruction_idx {
                print!("<{}>", ip);
            } else {
                print!("{}", ip);
            }
            if instruction_idx != expr.len() - 1 {
                print!(" ");
            }
        }
        println!("]");
    }
}

fn main() {
    let mut stack_history = Vec::new();
    let mut prev_expr = Vec::new();
    loop {
        let input = prompt();
        let Some(input) = parse_input(&input) else {
            println!("invalid input");
            continue;
        };

        match input {
            Input::Expression(tokens) => {
                let Some((result,history)) = eval(&tokens) else {
                    println!("invalid expression");
                    continue;
                };
                stack_history = history;
                prev_expr = tokens;
                println!("{}", result);
            }
            Input::Command(cmd) => {
                if cmd == Command::Quit {
                    break;
                }

                if cmd == Command::Trace {
                    print_stack_trace(&prev_expr, &stack_history);
                }
            }
        }
    }
}
