use std::fmt;

use crate::error::CustomError;
mod utils;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Expoent,
    ParenthesesOpen,
    ParenthesesClose,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Operand(u32),
    Operator(Operator),
}

struct Postfix(Vec<Token>);

impl fmt::Display for Postfix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = String::from("");
        let clone = self.0.clone();
        for token in clone {
            match token {
                Token::Operand(n) => {
                    display += &n.to_string();
                    display += " "
                },
                Token::Operator(Operator::Add) => {
                    display += "+";
                }
                Token::Operator(Operator::Sub) => {
                    display += "-";
                }
                Token::Operator(Operator::Mul) => {
                    display += "*";
                }
                Token::Operator(Operator::Div) => {
                    display += "/";
                }
                Token::Operator(Operator::Expoent) => {
                    display += "^";
                }
                _ =>{},
            }
        }
        write!(f, "{}", display)
    }
}

pub struct Calculator {}

impl Calculator {
    pub fn rpn(expr: &str) -> Result<Vec<Token>, CustomError> {
        let mut rpn: Vec<Token> = Vec::new();
        let mut stack: Vec<Operator> = Vec::new();

        let mut char_after_operator = false;
        let mut parentheses_open_count: i32 = 0;
        let mut parentheses_closed_count: i32 = 0;

        for c in expr.chars() {
            match c {
                '0'..='9' => match rpn.last_mut() {
                    Some(Token::Operand(n)) => {
                        if char_after_operator {
                            rpn.push(Token::Operand(c as u32 - 48)); // ascii
                            char_after_operator = false;
                        } else {
                            let can_make_op = n.checked_mul(10); 
                            match can_make_op {
                                Some(digit) => {
                                    let can_make_op = digit.checked_add(c as u32 - 48); 
                                    match can_make_op {
                                        Some(digit) => *n = digit, // add c to n, if n is 2 and c 5 will become 25
                                        None => return Err(CustomError::Generic("The numeric value that is outside of the range".to_string())),
                                    }
                                }
                                None => return Err(CustomError::Generic("The numeric value that is outside of the range".to_string())),
                            }
                        }
                    }
                    _ => {
                        char_after_operator = false;
                        rpn.push(Token::Operand(c as u32 - 48)); // ascii
                    }
                },
                '(' => {
                    parentheses_open_count += 1;
                    utils::stack_manipulation(
                        &mut rpn,
                        &mut stack,
                        Operator::ParenthesesOpen,
                        &mut parentheses_open_count,
                        &mut parentheses_closed_count,
                    );
                    char_after_operator = true;
                }
                ')' => {
                    parentheses_closed_count += 1;
                    utils::stack_manipulation(
                        &mut rpn,
                        &mut stack,
                        Operator::ParenthesesClose,
                        &mut parentheses_open_count,
                        &mut parentheses_closed_count,
                    );
                    char_after_operator = true;
                }
                '^' => {
                    utils::stack_manipulation(
                        &mut rpn,
                        &mut stack,
                        Operator::Expoent,
                        &mut parentheses_open_count,
                        &mut parentheses_closed_count,
                    );
                    char_after_operator = true;
                }
                '+' => {
                    utils::stack_manipulation(
                        &mut rpn,
                        &mut stack,
                        Operator::Add,
                        &mut parentheses_open_count,
                        &mut parentheses_closed_count,
                    );
                    char_after_operator = true;
                }
                '-' => {
                    utils::stack_manipulation(
                        &mut rpn,
                        &mut stack,
                        Operator::Sub,
                        &mut parentheses_open_count,
                        &mut parentheses_closed_count,
                    );
                    char_after_operator = true;
                }
                '*' => {
                    utils::stack_manipulation(
                        &mut rpn,
                        &mut stack,
                        Operator::Mul,
                        &mut parentheses_open_count,
                        &mut parentheses_closed_count,
                    );
                    char_after_operator = true;
                }
                '/' => {
                    utils::stack_manipulation(
                        &mut rpn,
                        &mut stack,
                        Operator::Div,
                        &mut parentheses_open_count,
                        &mut parentheses_closed_count,
                    );
                    char_after_operator = true;
                }
                ' ' => {}
                '\n' => {}
                _ => return Err(CustomError::Generic(format!("Character '{}' is not allowed.", c))),
            }
        }

        if !stack.is_empty() {
            while let Some(op) = stack.pop() {
                rpn.push(Token::Operator(op));
            }
        }

        Ok(rpn)
    }

    pub fn evaluate(mut rpn: Vec<Token>) -> Result<f64, CustomError> {
        rpn.reverse();
        let result = utils::evaluate(rpn);
        Ok(result)

    }

    pub fn display_rpn(rpn: Vec<Token>) -> String {
        Postfix(rpn).to_string()
    }
}



