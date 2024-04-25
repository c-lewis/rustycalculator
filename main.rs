use std::io;
extern crate regex;
extern crate rug;
use regex::Regex;
use rug::{Assign, Float};

#[derive(Debug)]
enum Token {
    Num(Float),
    Plus,
    Minus,
    Multiply,
    Divide,
    Leftparen,
    Rightparen,
}
fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();
    let dft_prc = 512;

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                let mut number = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_digit(10) || ch == '.' {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                let number = number.parse::<f64>().unwrap();
                let number = Float::with_val(dft_prc, number);
                tokens.push(Token::Num(number));
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '(' => {
                tokens.push(Token::Leftparen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::Rightparen);
                chars.next();
            }
            _ => panic!("FUCKED YOU TYPE IN IDIOT?!?!?!"),
        }
    }

    tokens
}

fn postfixer(mut tokens: Vec<Token>) -> Vec<Token> {
    tokens.reverse(); //c
    let mut tobeoutput: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    while let Some(last) = tokens.pop() {
        match last {
            //c
            Token::Num(_) => tobeoutput.push(last),
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Leftparen => {
                operators.push(last);
            } //c
            Token::Rightparen => {
                while let Some(op) = operators.pop() {
                    if let Token::Leftparen = op {
                        break;
                    } else {
                        tobeoutput.push(op);
                    }
                }
            }
        }
    }
    while !operators.is_empty() {
        tobeoutput.push(operators.pop().unwrap());
    }
    tobeoutput
}

fn solver(mut tokens: Vec<Token>) -> String {
    tokens.reverse();
    let mut resultoutput: Vec<Token> = Vec::new();
    let mut cureentrunner: Vec<Token> = Vec::new();
    let mut anumber: Float = Float::new(512);
    let mut secondnumber: Float = Float::new(512);
    let mut finalresult: String = String::new();
    let mut hasbeenrun: i32 = 0;
    while let Some(current) = tokens.pop() {
        match current {
            Token::Num(_) => cureentrunner.push(current),
            Token::Plus => {
                while let Some(curentoken) = cureentrunner.pop() {
                    match curentoken {
                        Token::Num(value) => {
                            if hasbeenrun == 0 {
                                anumber = Float::with_val(512, value);
                                hasbeenrun = 1;
                            } else {
                                secondnumber = Float::with_val(512, value);
                                hasbeenrun = 2;
                            }
                            if hasbeenrun == 2 {
                                cureentrunner
                                    .push(Token::Num(anumber.clone() + secondnumber.clone()));
                                hasbeenrun = 0;
                                break;
                            }
                        }
                        Token::Plus
                        | Token::Minus
                        | Token::Multiply
                        | Token::Divide
                        | Token::Leftparen
                        | Token::Rightparen => {}
                    }
                }
                let finalresult = anumber.clone() + secondnumber.clone();
                println!(
                    "first number: {:?}, second number: {:?}, result: {:?}",
                    anumber, secondnumber, finalresult
                );
            }
            Token::Minus => {
                while let Some(curentoken) = cureentrunner.pop() {
                    match curentoken {
                        Token::Num(value) => {
                            if hasbeenrun == 0 {
                                anumber = Float::with_val(512, value);
                                hasbeenrun = 1;
                            } else {
                                secondnumber = Float::with_val(512, value);
                                hasbeenrun = 2;
                            }
                            if hasbeenrun == 2 {
                                cureentrunner
                                    .push(Token::Num(anumber.clone() - secondnumber.clone()));
                                hasbeenrun = 0;
                                break;
                            }
                        }
                        Token::Plus
                        | Token::Minus
                        | Token::Multiply
                        | Token::Divide
                        | Token::Leftparen
                        | Token::Rightparen => {}
                    }
                }
                let finalresult = anumber.clone() - secondnumber.clone();
                println!(
                    "first number: {:?}, second number: {:?}, result: {:?}",
                    anumber, secondnumber, finalresult
                );
            }
            Token::Multiply => {
                while let Some(curentoken) = cureentrunner.pop() {
                    match curentoken {
                        Token::Num(value) => {
                            if hasbeenrun == 0 {
                                anumber = Float::with_val(512, value);
                                hasbeenrun = 1;
                            } else {
                                secondnumber = Float::with_val(512, value);
                                hasbeenrun = 2;
                            }
                            if hasbeenrun == 2 {
                                cureentrunner
                                    .push(Token::Num(anumber.clone() * secondnumber.clone()));
                                hasbeenrun = 0;
                                break;
                            }
                        }
                        Token::Plus
                        | Token::Minus
                        | Token::Multiply
                        | Token::Divide
                        | Token::Leftparen
                        | Token::Rightparen => {}
                    }
                }
                let finalresult = anumber.clone() * secondnumber.clone();
                println!(
                    "first number: {:?}, second number: {:?}, result: {:?}",
                    anumber, secondnumber, finalresult
                );
            }
            Token::Divide => {
                while let Some(curentoken) = cureentrunner.pop() {
                    match curentoken {
                        Token::Num(value) => {
                            if hasbeenrun == 0 {
                                anumber = Float::with_val(512, value);
                                hasbeenrun = 1;
                            } else {
                                secondnumber = Float::with_val(512, value);
                                hasbeenrun = 2;
                            }
                            if hasbeenrun == 2 {
                                cureentrunner
                                    .push(Token::Num(anumber.clone() / secondnumber.clone()));
                                hasbeenrun = 0;
                                break;
                            }
                        }
                        Token::Plus
                        | Token::Minus
                        | Token::Multiply
                        | Token::Divide
                        | Token::Leftparen
                        | Token::Rightparen => {}
                    }
                }
                let finalresult = anumber.clone() / secondnumber.clone();
                println!(
                    "first number: {:?}, second number: {:?}, result: {:?}",
                    anumber, secondnumber, finalresult
                );
            }
            Token::Leftparen | Token::Rightparen => {}
        }
    }
    finalresult
}

fn main() {
    let mut input = String::new();
    println!("enter equation");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    let trinput = input.trim();
    let re = Regex::new(r"\)\(").unwrap();
    let trinput = re.replace_all(trinput, ")*(");
    let trinput = trinput.as_ref();
    let output = solver(postfixer(tokenizer(trinput)));
    println!("{:?}", output);
}
