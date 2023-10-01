// TODO expoent
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
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

        for c in expr.chars() {
            match c {
                '0'..='9' => match rpn.last_mut() {
                    Some(Token::Operand(n)) => {
                        *n = *n * 10 + (c as u32 - 48); // add c to n, if n is 2 and c 5 will become 25
                    }
                    _ => {
                        let digit = c as u32 - 48; // ascii
                        rpn.push(Token::Operand(digit))
                    }
                },
                '(' => {
                    stack_manipulation(&mut rpn, &mut stack, Operator::ParenthesesOpen)
                }
                ')' => {
                    stack_manipulation(&mut rpn, &mut stack, Operator::ParenthesesClose)
                }
                '+' => stack_manipulation(&mut rpn, &mut stack, Operator::Add),
                '-' => stack_manipulation(&mut rpn, &mut stack, Operator::Sub),
                '*' => stack_manipulation(&mut rpn, &mut stack, Operator::Mul),
                '/' => stack_manipulation(&mut rpn, &mut stack, Operator::Div),
                ' ' => {}
                '\n' => {}
                _ => return Err(Error::BadToken(c)),
            }
        }

        if stack.len() > 0 {
            rpn_push(&mut stack, &mut rpn);
        }

        Ok(rpn)
    }
}

fn stack_manipulation(rpn: &mut Vec<Token>, stack: &mut Vec<Operator>, operator: Operator) {
    if stack.is_empty() || operator == Operator::ParenthesesOpen || stack[stack.len() - 1] == Operator::ParenthesesOpen {
        stack.push(operator)
    } else {
        if operator == Operator::ParenthesesClose {
            rpn_push(stack, rpn);
        } else {
            if stack[stack.len() - 1] == Operator::Add
                || stack[stack.len() - 1] == Operator::Sub
            {
                if operator == Operator::Add || operator == Operator::Sub {
                    rpn_push(stack, rpn);
                    stack.push(operator);
                } else if operator == Operator::Mul || operator == Operator::Div {
                    stack.push(operator);
                }
            } else if stack[stack.len() - 1] == Operator::Mul
                || stack[stack.len() - 1] == Operator::Div
            {
                match stack.pop() {
                    Some(op) => {
                        rpn.push(Token::Operator(op))
                    }
                    None => println!("error: operators_set_up"), // TODO error handling
                }
                stack.push(operator);
            }
        }
    }
}

fn rpn_push(stack: &mut Vec<Operator>, rpn: &mut Vec<Token>) {
    while let Some(op) = stack.pop() {
        if op == Operator::ParenthesesOpen {
            break;
        } else {
            rpn.push(Token::Operator(op));
        }
    }
}
