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

#[derive(Debug)]
pub enum Error {
    BadToken(char),
}

pub struct Calculator {}

impl Calculator {
    pub fn rpn(expr: &str) -> Result<Vec<Token>, Error> {
        let mut rpn: Vec<Token> = Vec::new();
        let mut stack: Vec<Operator> = Vec::new();

        let mut char_after_operator = false;
        let mut parentheses_open_count: i32 = 0;
        let mut parentheses_closed_count: i32 = 0;

        for c in expr.chars() {
            match c {
                '0'..='9' => match rpn.last_mut() {
                    Some(Token::Operand(n)) => {
                        if char_after_operator == true {
                            let digit = c as u32 - 48; // ascii
                            rpn.push(Token::Operand(digit));
                            char_after_operator = false;
                        } else {
                            *n = *n * 10 + (c as u32 - 48); // add c to n, if n is 2 and c 5 will become 25
                        }
                    }
                    _ => {
                        char_after_operator = false;
                        let digit = c as u32 - 48; // ascii
                        rpn.push(Token::Operand(digit))
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
                _ => return Err(Error::BadToken(c)),
            }
        }

        if stack.len() > 0 {
            while let Some(op) = stack.pop() {
                rpn.push(Token::Operator(op));
            }
        }

        Ok(rpn)
    }

    #[allow(unused_variables, dead_code)]
    pub fn evaluate(expr: Vec<Token>) -> Result<f64, Error> {
        Ok(f64::MIN)
    }
}
