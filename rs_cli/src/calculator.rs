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

        let mut char_after_operator = false;
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
                        let digit = c as u32 - 48; // ascii
                        rpn.push(Token::Operand(digit))
                    }
                },
                '(' => {
                    stack_manipulation(&mut rpn, &mut stack, Operator::ParenthesesOpen);
                    char_after_operator = true;
                }
                ')' => {
                    stack_manipulation(&mut rpn, &mut stack, Operator::ParenthesesClose);
                    char_after_operator = true;
                }
                '+' => {
                    stack_manipulation(&mut rpn, &mut stack, Operator::Add);
                    char_after_operator = true;
                }
                '-' => {
                    stack_manipulation(&mut rpn, &mut stack, Operator::Sub);
                    char_after_operator = true;
                }
                '*' => {
                    stack_manipulation(&mut rpn, &mut stack, Operator::Mul);
                    char_after_operator = true;
                }
                '/' => {
                    stack_manipulation(&mut rpn, &mut stack, Operator::Div);
                    char_after_operator = true;
                }
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
    if stack.is_empty()
        || operator == Operator::ParenthesesOpen
        || stack[stack.len() - 1] == Operator::ParenthesesOpen
    {
        stack.push(operator)
    } else {
        if operator == Operator::ParenthesesClose {
            rpn_push(stack, rpn);
        } else {
            if stack[stack.len() - 1] == Operator::Add || stack[stack.len() - 1] == Operator::Sub {
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
                    Some(op) => rpn.push(Token::Operator(op)),
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

#[test]
fn calculator_rpn() {
    println!("Start rpn_result_1");
    // ! "6*3-(4-5)+2" -> [Operand(6), Operand(3), Operator(Mul), Operand(4), Operand(5), Operator(Sub), Operator(Sub), Operand(2), Operator(Add)] -> 63*45--2+ = 21
    let rpn_result_1 = Calculator::rpn("6*3-(4-5)+2");
    match rpn_result_1 {
        Ok(rpn) => {
            assert_eq!(
                rpn,
                [
                    Token::Operand(6),
                    Token::Operand(3),
                    Token::Operator(Operator::Mul),
                    Token::Operand(4),
                    Token::Operand(5),
                    Token::Operator(Operator::Sub),
                    Token::Operator(Operator::Sub),
                    Token::Operand(2),
                    Token::Operator(Operator::Add)
                ]
            )
        }
        Err(_) => {}
    };

    println!("Start rpn_result_2");
    // ! "(1+2)*(3+4)" -> [Operand(1), Operand(2), Operator(Add), Operand(3), Operand(4), Operator(Add), Operator(Mul)]-> 12+34+* = 21
    let rpn_result_2 = Calculator::rpn("(1+2)*(3+4)");
    match rpn_result_2 {
        Ok(rpn) => {
            assert_eq!(
                rpn,
                [
                    Token::Operand(1),
                    Token::Operand(2),
                    Token::Operator(Operator::Add),
                    Token::Operand(3),
                    Token::Operand(4),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Mul)
                ]
            )
        }
        Err(_) => {}
    };

    println!("Start rpn_result_3");
    // ! "1+2*3+4" -> [Operand(1), Operand(2), Operand(3), Operator(Mul), Operand(4), Operator(Add), Operator(Add)] -> 123*4++ = 11
    let rpn_result_3 = Calculator::rpn("1+2*3+4");
    match rpn_result_3 {
        Ok(rpn) => {
            assert_eq!(
                rpn,
                [
                    Token::Operand(1),
                    Token::Operand(2),
                    Token::Operand(3),
                    Token::Operator(Operator::Mul),
                    Token::Operand(4),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Add)
                ]
            )
        }
        Err(_) => {}
    };

    println!("Start rpn_result_4");
    // ! "1+2*(3+4)" -> [Operand(1), Operand(2), Operand(3), Operand(4), Operator(Add), Operator(Mul), Operator(Add)] -> 1234+*+ = 15
    let rpn_result_4 = Calculator::rpn("1+2*(3+4)");
    match rpn_result_4 {
        Ok(rpn) => {
            assert_eq!(
                rpn,
                [
                    Token::Operand(1),
                    Token::Operand(2),
                    Token::Operand(3),
                    Token::Operand(4),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Mul),
                    Token::Operator(Operator::Add)
                ]
            )
        }
        Err(_) => {}
    };

    println!("Start rpn_result_5");
    // ! "(1+2)*3+4" -> [Operand(1), Operand(2), Operator(Add), Operand(3), Operator(Mul), Operand(4), Operator(Add)] -> 12+3*4+ = 13
    let rpn_result_5 = Calculator::rpn("(1+2)*3+4");
    match rpn_result_5 {
        Ok(rpn) => {
            assert_eq!(
                rpn,
                [
                    Token::Operand(1),
                    Token::Operand(2),
                    Token::Operator(Operator::Add),
                    Token::Operand(3),
                    Token::Operator(Operator::Mul),
                    Token::Operand(4),
                    Token::Operator(Operator::Add)
                ]
            )
        }
        Err(_) => {}
    };

    println!("Start rpn_result_6");
    // ! "(2/4)*(5-6)" -> [Operand(2), Operand(4), Operator(Div), Operand(5), Operand(6), Operator(Sub), Operator(Mul)] -> 24/56-* = -0.5
    let rpn_result_6 = Calculator::rpn("(2/4)*(5-6)");
    match rpn_result_6 {
        Ok(rpn) => {
            assert_eq!(
                rpn,
                [
                    Token::Operand(2),
                    Token::Operand(4),
                    Token::Operator(Operator::Div),
                    Token::Operand(5),
                    Token::Operand(6),
                    Token::Operator(Operator::Sub),
                    Token::Operator(Operator::Mul)
                ]
            )
        }
        Err(_) => {}
    };

    println!("Start rpn_result_7");
    // ! "3/(5+8*9)" -> [Operand(3), Operand(5), Operand(8), Operand(9), Operator(Mul), Operator(Add), Operator(Div)] -> 3589*+/ = 7
    let rpn_result_7 = Calculator::rpn("3/(5+8*9)");
    match rpn_result_7 {
        Ok(rpn) => {
            assert_eq!(
                rpn,
                [
                    Token::Operand(3),
                    Token::Operand(5),
                    Token::Operand(8),
                    Token::Operand(9),
                    Token::Operator(Operator::Mul),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Div)
                ]
            )
        }
        Err(_) => {}
    };
}
