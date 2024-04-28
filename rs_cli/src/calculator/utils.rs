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





#[allow(unused_imports)]
use colored::Colorize;
#[test]
fn calculator() {
    // ! https://www.omnicalculator.com/math/polish-notation
    println!("{}", "Start rpn_result_1".blue());
    // * "6*3-(4-5)+2"
    // * [Operand(6), Operand(3), Operator(Mul), Operand(4), Operand(5), Operator(Sub), Operator(Sub), Operand(2), Operator(Add)]
    // * 63*45--2+ = 21
    let rpn_result = Calculator::rpn("6*3-(4-5)+2");
    match rpn_result {
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
            );
            println!("{}", "End rpn_result_1".green());
            println!("{}", "Start evaluate_result_1".yellow());
            let evaluate_result = Calculator::evaluate(rpn);
            assert_eq!(evaluate_result.unwrap(), 21.0);
        }
        Err(_) => {}
    };
    println!("{}", "End evaluate_result_1".green());

    println!("{}", "Start rpn_result_2".blue());
    // * "(1+2)*(3+4)"
    // * Operand(1), Operand(2), Operator(Add), Operand(3), Operand(4), Operator(Add), Operator(Mul)]
    // * 1 2 + 3 4 + * = 21
    let rpn_result = Calculator::rpn("(1+2)*(3+4)");
    match rpn_result {
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
            );
            println!("{}", "End rpn_result_2".green());
            println!("{}", "Start evaluate_result_2".yellow());
            let evaluate_result = Calculator::evaluate(rpn);
            assert_eq!(evaluate_result.unwrap(), 21.0);
        }
        Err(_) => {}
    };
    println!("{}", "End evaluate_result_2".green());

    println!("{}", "Start rpn_result_3".blue());
    // * "1+2*3+4" 
    // * [Operand(1), Operand(2), Operand(3), Operator(Mul), Operator(Add), Operand(4), Operator(Add)] 
    // * 1 2 3 * + 4 + = 11
    let rpn_result = Calculator::rpn("1+2*3+4");
    match rpn_result {
        Ok(rpn) => {
            assert_eq!(
                rpn,
                [
                    Token::Operand(1),
                    Token::Operand(2),
                    Token::Operand(3),
                    Token::Operator(Operator::Mul),
                    Token::Operator(Operator::Add),
                    Token::Operand(4),
                    Token::Operator(Operator::Add)
                ]
            );
            println!("{}", "End rpn_result_3".green());
            println!("{}", "Start evaluate_result_3".yellow());
            let evaluate_result = Calculator::evaluate(rpn);
            assert_eq!(evaluate_result.unwrap(), 11.0);
        }
        Err(_) => {}
    };
    println!("{}", "End evaluate_result_3".green());

    println!("{}", "Start rpn_result_4".blue());
    // * "1+2*(3+4)"
    // * [Operand(1), Operand(2), Operand(3), Operand(4), Operator(Add), Operator(Mul), Operator(Add)] 
    // * 1234+*+ = 15
    let rpn_result = Calculator::rpn("1+2*(3+4)");
    match rpn_result {
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
            );
            println!("{}", "End rpn_result_4".green());
            println!("{}", "Start evaluate_result_4".yellow());
            let evaluate_result = Calculator::evaluate(rpn);
            assert_eq!(evaluate_result.unwrap(), 15.0);
        }
        Err(_) => {}
    };
    println!("{}", "End evaluate_result_4".green());

    println!("{}", "Start rpn_result_5".blue());
    // * "(1+2)*3+4" 
    // * [Operand(1), Operand(2), Operator(Add), Operand(3), Operator(Mul), Operand(4), Operator(Add)]
    // * 12+3*4+ = 13
    let rpn_result = Calculator::rpn("(1+2)*3+4");
    match rpn_result {
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
            );
            println!("{}", "End rpn_result_5".green());
            println!("{}", "Start evaluate_result_5".yellow());
            let evaluate_result = Calculator::evaluate(rpn);
            assert_eq!(evaluate_result.unwrap(), 13.0);
        }
        Err(_) => {}
    };
    println!("{}", "End evaluate_result_5".green());

    println!("{}", "Start rpn_result_6".blue());
    // * "(2/4)*(5-6)" 
    // * [Operand(2), Operand(4), Operator(Div), Operand(5), Operand(6), Operator(Sub), Operator(Mul)] 
    // * 24/56-* = -0.5
    let rpn_result = Calculator::rpn("(2/4)*(5-6)");
    match rpn_result {
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
            );
            println!("{}", "End rpn_result_6".green());
            println!("{}", "Start evaluate_result_6".yellow());
            let evaluate_result = Calculator::evaluate(rpn);
            assert_eq!(evaluate_result.unwrap(), -0.5);
        }
        Err(_) => {}
    };
    println!("{}", "End evaluate_result_6".green());

    println!("{}", "Start rpn_result_7".blue());
    // * "3/(5+8*9)"
    // * [Operand(3), Operand(5), Operand(8), Operand(9), Operator(Mul), Operator(Add), Operator(Div)]
    // *  3589*+/ = 0.03896103896103896
    let rpn_result = Calculator::rpn("3/(5+8*9)");
    match rpn_result {
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
            );
            println!("{}", "End rpn_result_7".green());
            println!("{}", "Start evaluate_result_7".yellow());
            let evaluate_result = Calculator::evaluate(rpn);
            assert_eq!(evaluate_result.unwrap(), 0.03896103896103896);
        }
        Err(_) => {}
    };
    println!("{}", "End evaluate_result_7".green());

    println!("{}", "Start rpn_result_8".blue());
    // * "3*5^2/5-8*9^(2-4)"
    // * -> [Operand(3), Operand(5), Operand(2), Operand(5), Operator(Div), Operator(Expoent), Operator(Mul), Operand(8), Operand(9), Operand(2), Operand(4), Operator(Sub), Operator(Expoent), Operator(Mul), Operator(Sub)]
    // * -> 3 5 2 ^ * 5 / 8 9 2 4 - ^ * - => 5.61219638404887
    let rpn_result = Calculator::rpn("3*5^2/5-8*9^(2-4)");
    match rpn_result {
        Ok(rpn) => {
            assert_eq!(
                rpn,
                [
                    Token::Operand(3),
                    Token::Operand(5),
                    Token::Operand(2),
                    Token::Operand(5),
                    Token::Operator(Operator::Div),
                    Token::Operator(Operator::Expoent),
                    Token::Operator(Operator::Mul),
                    Token::Operand(8),
                    Token::Operand(9),
                    Token::Operand(2),
                    Token::Operand(4),
                    Token::Operator(Operator::Sub),
                    Token::Operator(Operator::Expoent),
                    Token::Operator(Operator::Mul),
                    Token::Operator(Operator::Sub)
                ]
            );
            println!("{}", "End rpn_result_8".green());
            println!("{}", "Start evaluate_result_8".yellow());
            let evaluate_result = Calculator::evaluate(rpn);
            assert_eq!(evaluate_result.unwrap(), 5.61219638404887);
        }
        Err(_) => {}
    };
    println!("{}", "End evaluate_result_8".green());

    println!("{}", "Start rpn_result_9".blue());
    // * "((15/(7-(1+1)))*3)-(2+(1+1))"
    // * -> [Operand(15), Operand(7), Operand(1), Operand(1), Operator(Add), Operator(Sub), Operator(Div), Operand(3), Operator(Mul), Operand(2), Operand(1), Operand(1), Operator(Add), Operator(Add), Operator(Sub)]
    // * -> 15 7 1 1 + − / 3 * 2 1 1 + + − => 5
    let rpn_result = Calculator::rpn("((15/(7-(1+1)))*3)-(2+(1+1))");
    match rpn_result {
        Ok(rpn) => {
            assert_eq!(
                rpn,
                [
                    Token::Operand(15),
                    Token::Operand(7),
                    Token::Operand(1),
                    Token::Operand(1),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Sub),
                    Token::Operator(Operator::Div),
                    Token::Operand(3),
                    Token::Operator(Operator::Mul),
                    Token::Operand(2),
                    Token::Operand(1),
                    Token::Operand(1),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Sub)
                ]
            );
            println!("{}", "End rpn_result_9".green());
            println!("{}", "Start evaluate_result_9".yellow());
            let evaluate_result = Calculator::evaluate(rpn);
            assert_eq!(evaluate_result.unwrap(), 5.0);
        }
        Err(_) => {}
    };
    println!("{}", "End evaluate_result_9".green());

    println!("{}", "Start rpn_result_10".blue());
    // * "((15/(7-(1+1)))*3)-((2+(1+1))+5)"
    // * -> [Operand(15), Operand(7), Operand(1), Operand(1), Operator(Add), Operator(Sub), Operator(Div), Operand(3), Operator(Mul), Operand(2), Operand(1), Operand(1), Operator(Add), Operator(Add), Operand(5), Operator(Add), Operator(Sub)]
    // * -> 15 7 1 1 + - / 3 * 2 1 1 + + 5 + - => 0
    let rpn_result = Calculator::rpn("((15/(7-(1+1)))*3)-((2+(1+1))+5)");
    match rpn_result {
        Ok(rpn) => {
            assert_eq!(
                rpn,
                [
                    Token::Operand(15),
                    Token::Operand(7),
                    Token::Operand(1),
                    Token::Operand(1),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Sub),
                    Token::Operator(Operator::Div),
                    Token::Operand(3),
                    Token::Operator(Operator::Mul),
                    Token::Operand(2),
                    Token::Operand(1),
                    Token::Operand(1),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Add),
                    Token::Operand(5),
                    Token::Operator(Operator::Add),
                    Token::Operator(Operator::Sub)
                ]
            );
            println!("{}", "End rpn_result_10".green());
            println!("{}", "Start evaluate_result_10".yellow());
            let evaluate_result = Calculator::evaluate(rpn);
            assert_eq!(evaluate_result.unwrap(), 0.0);
        }
        Err(_) => {}
    };
    println!("{}", "End evaluate_result_10".green());
}
