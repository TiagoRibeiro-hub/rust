use crate::calculator::*;

fn last_is_add_or_sub(stack: &Vec<Operator>) -> bool {
    stack[stack.len() - 1] == Operator::Add || stack[stack.len() - 1] == Operator::Sub
}

fn is_less_or_equal_than(operator: &Operator, operator_from_stack: &Operator) -> bool {
    // ! PEMDAS => Parentheses -> Expoent -> Mul or Div -> Add or Sub
    // * if true pop else push
    let mut result = false;
    if *operator == Operator::Mul || *operator == Operator::Div {
        //*operator_from_stack == Operator::Mul || *operator_from_stack == Operator::Div || *operator_from_stack == Operator::Expoent;
        result = *operator_from_stack != Operator::Add && *operator_from_stack != Operator::Sub
    } else if *operator == Operator::Add || *operator == Operator::Sub {
        //*operator_from_stack == Operator::Mul || *operator_from_stack == Operator::Div || *operator_from_stack == Operator::Expoent || *operator_from_stack == Operator::Add || *operator_from_stack == Operator::Sub;
        result = true;
    }

    result
}

fn inside_parentheses_manipulation(
    operator: Operator,
    parentheses_open_count: &mut i32,
    parentheses_closed_count: &mut i32,
    stack: &mut Vec<Operator>,
    rpn: &mut Vec<Token>,
) {
    if parentheses_open_count > parentheses_closed_count {
        if stack[stack.len() - 1] == Operator::ParenthesesClose
            && operator != Operator::ParenthesesClose
        {
            let mut to_closed = *parentheses_closed_count;

            while let Some(op) = stack.pop() {
                if op == Operator::ParenthesesOpen {
                    *parentheses_open_count -= 1;
                    to_closed -= 1;
                    if *parentheses_open_count == 0 || to_closed == 0 {
                        break;
                    }
                } else if op == Operator::ParenthesesClose {
                    *parentheses_closed_count -= 1
                } else {
                    rpn.push(Token::Operator(op));
                }
            }
        }
        stack.push(operator);
    } else {
        // * all parentheses are closed pop everything to rpn until find the last close parenthese
        while let Some(op) = stack.pop() {
            if op == Operator::ParenthesesOpen {
                *parentheses_open_count -= 1;
                if *parentheses_open_count == 0 {
                    *parentheses_closed_count = 0;
                    break;
                }
            } else if op == Operator::ParenthesesClose {
                continue;
            } else {
                rpn.push(Token::Operator(op));
            }
        }
    }
}

pub fn stack_manipulation(
    rpn: &mut Vec<Token>,
    stack: &mut Vec<Operator>,
    operator: Operator,
    parentheses_open_count: &mut i32,
    parentheses_closed_count: &mut i32,
) {
    if stack.contains(&Operator::ParenthesesOpen) {
        inside_parentheses_manipulation(
            operator,
            parentheses_open_count,
            parentheses_closed_count,
            stack,
            rpn,
        );
    } else if stack.is_empty()
        || operator == Operator::ParenthesesOpen
        || operator == Operator::Expoent
        || (last_is_add_or_sub(stack) && (operator == Op::Mul || operator == Op::Div))
    {
        stack.push(operator)
    } else {
        let mut stack_clone = stack.clone();
        while let Some(op) = stack_clone.pop() {
            if is_less_or_equal_than(&operator, &op) {
                rpn.push(Token::Operator(op));
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(operator);
    }
}

pub fn last_two_operands(stack: &mut Vec<f64>) -> (f64, f64) {
    let right_side = stack.pop().unwrap_or(0.0);
    let left_side = stack.pop().unwrap_or(0.0);
    (left_side, right_side)
}

pub fn evaluate(mut rpn: Vec<Token>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();
    let mut result: f64 = 0.0;
    while let Some(token) = rpn.pop() {
        match token {
            Token::Operand(n) => {
                stack.push(n as f64);
            }
            Token::Operator(op) => match op {
                Operator::Add => {
                    let operands = last_two_operands(&mut stack);
                    result = operands.0 + operands.1;
                    stack.push(result);
                }
                Operator::Sub => {
                    let operands = last_two_operands(&mut stack);
                    result = operands.0 - operands.1;
                    stack.push(result);
                }
                Operator::Mul => {
                    let operands = last_two_operands(&mut stack);
                    result = operands.0 * operands.1;
                    stack.push(result);
                }
                Operator::Div => {
                    let operands = last_two_operands(&mut stack);
                    result = operands.0 / operands.1;
                    stack.push(result);
                }
                Operator::Expoent => {
                    let operands = last_two_operands(&mut stack);
                    result = operands.0.powf(operands.1);
                    stack.push(result);
                }
                _ => {}
            },
        }
    }
    result
}




#[cfg(test)]
mod calc_tests {
    use crate::calculator::*;
    // ! https://paodayag.dev/reverse-polish-notation-js-parser/converter.html
    // ! https://www.dcode.fr/reverse-polish-notation
    // ! https://www.omnicalculator.com/math/polish-notation
    
    #[test]
    fn calculator_1() {
        let rpn_result = Calculator::rpn("2+4^2*1");
        match rpn_result {
            Ok(rpn) => {
                assert_eq!(Calculator::display(rpn.clone()), "2 4 2 ^ 1 * +".to_string());
                let evaluate_result = Calculator::evaluate(rpn);
                assert_eq!(evaluate_result, 18.0);
            }
            Err(_) => {}
        };
    }
    
    #[test]
    fn calculator_2() {
        let rpn_result = Calculator::rpn("3*5^2/5-8*9^(2-4)");
        match rpn_result {
            Ok(rpn) => {
                assert_eq!(Calculator::display(rpn.clone()), "3 5 2 ^ * 5 / 8 9 2 4 - ^ * -".to_string());
                let evaluate_result = Calculator::evaluate(rpn);
                assert_eq!(evaluate_result, 14.901234567901234);
            }
            Err(_) => {}
        };
    }
    
    #[test]
    fn calculator_3() {
        let rpn_result = Calculator::rpn("6*3-(4-5)+2");
        match rpn_result {
            Ok(rpn) => {
                assert_eq!(Calculator::display(rpn.clone()), "6 3 * 4 5 - - 2 +".to_string());
                let evaluate_result = Calculator::evaluate(rpn);
                assert_eq!(evaluate_result, 21.0);
            }
            Err(_) => {}
        };
    }
    
    #[test]
    fn calculator_4() {
        let rpn_result = Calculator::rpn("(1+2)*(3+4)");
        match rpn_result {
            Ok(rpn) => {
                assert_eq!(Calculator::display(rpn.clone()), "1 2 + 3 4 + *".to_string());
                let evaluate_result = Calculator::evaluate(rpn);
                assert_eq!(evaluate_result, 21.0);
            }
            Err(_) => {}
        };
    }
    
    #[test]
    fn calculator_5() {
        let rpn_result = Calculator::rpn("1+2*3+4");
        match rpn_result {
            Ok(rpn) => {
                assert_eq!(Calculator::display(rpn.clone()), "1 2 3 * + 4 +".to_string());
                let evaluate_result = Calculator::evaluate(rpn);
                assert_eq!(evaluate_result, 11.0);
            }
            Err(_) => {}
        };
    }
    
    #[test]
    fn calculator_6() {
        let rpn_result = Calculator::rpn("1+2*(3+4)");
        match rpn_result {
            Ok(rpn) => {
                assert_eq!(Calculator::display(rpn.clone()), "1 2 3 4 + * +".to_string());
                let evaluate_result = Calculator::evaluate(rpn);
                assert_eq!(evaluate_result, 15.0);
            }
            Err(_) => {}
        };
    }
    
    #[test]
    fn calculator_7() {
        let rpn_result = Calculator::rpn("(1+2)*3+4");
        match rpn_result {
            Ok(rpn) => {
                assert_eq!(Calculator::display(rpn.clone()), "1 2 + 3 * 4 +".to_string());
                let evaluate_result = Calculator::evaluate(rpn);
                assert_eq!(evaluate_result, 13.0);
            }
            Err(_) => {}
        };
    }
    
    #[test]
    fn calculator_8() {
        let rpn_result = Calculator::rpn("(2/4)*(5-6)");
        match rpn_result {
            Ok(rpn) => {
                assert_eq!(Calculator::display(rpn.clone()), "2 4 / 5 6 - *".to_string());
                let evaluate_result = Calculator::evaluate(rpn);
                assert_eq!(evaluate_result, -0.5);
            }
            Err(_) => {}
        };
    }
    
    #[test]
    fn calculator_9() {
        let rpn_result = Calculator::rpn("3/(5+8*9)");
        match rpn_result {
            Ok(rpn) => {
                assert_eq!(Calculator::display(rpn.clone()), "3 5 8 9 * + /".to_string());
                let evaluate_result = Calculator::evaluate(rpn);
                assert_eq!(evaluate_result, 0.03896103896103896);
            }
            Err(_) => {}
        };
    }
    
    #[test]
    fn calculator_10() {
        let rpn_result = Calculator::rpn("((15/(7-(1+1)))*3)-(2+(1+1))");
        match rpn_result {
            Ok(rpn) => {
                assert_eq!(Calculator::display(rpn.clone()), "15 7 1 1 + - / 3 * 2 1 1 + + -".to_string());
                let evaluate_result = Calculator::evaluate(rpn);
                assert_eq!(evaluate_result, 5.0);
            }
            Err(_) => {}
        };
    }
    
    #[test]
    fn calculator_11() {
        let rpn_result = Calculator::rpn("((15/(7-(1+2^5)))*3)-((2+(1+1))+5)");
        match rpn_result {
            Ok(rpn) => {
                assert_eq!(Calculator::display(rpn.clone()), "15 7 1 2 5 ^ + - / 3 * 2 1 1 + + 5 + -".to_string());
                let evaluate_result = Calculator::evaluate(rpn);
                assert_eq!(evaluate_result, -10.73076923076923);
            }
            Err(_) => {}
        };
    }
}
