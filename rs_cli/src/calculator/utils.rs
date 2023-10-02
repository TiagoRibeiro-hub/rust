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
        stack.push(operator);
    } else {
        // * all parentheses are closed pop everything to rpn until find the last close parenthese
        while let Some(op) = stack.pop() {
            if op == Operator::ParenthesesOpen {
                *parentheses_open_count -= *parentheses_open_count;
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
        || last_is_add_or_sub(&stack) && operator == Operator::Mul || operator == Operator::Div
    {
        stack.push(operator)
    } else {
        while let Some(op) = stack.pop() {
            if is_less_or_equal_than(&operator, &op) {
                rpn.push(Token::Operator(op));
            } else {
                break;
            }
        }
        stack.push(operator);
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
    println!("End rpn_result_1");
    
    println!("Start rpn_result_2");
    // ! "(1+2)*(3+4)" -> [Operand(1), Operand(2), Operator(Add), Operand(3), Operand(4), Operator(Add), Operator(Mul)]-> 1 2 + 3 4 + * = 21
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
    println!("End rpn_result_2");

    println!("Start rpn_result_3");
    // ! "1+2*3+4" -> [Operand(1), Operand(2), Operand(3), Operator(Mul), Operator(Add), Operand(4), Operator(Add)] -> 1 2 3 * + 4 + = 11
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
                    Token::Operator(Operator::Add),
                    Token::Operand(4),
                    Token::Operator(Operator::Add)
                ]
            )
        }
        Err(_) => {}
    };
    println!("End rpn_result_3");

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
    println!("End rpn_result_4");

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
    println!("End rpn_result_5");

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
    println!("End rpn_result_6");

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
    println!("End rpn_result_7");

    println!("Start rpn_result_8");
    // ! "3*5^2/5-8*9^(2-4)" 
    // ! -> [Operand(3), Operand(5), Operand(2), Operand(5), Operator(Div), Operator(Expoent), Operator(Mul), Operand(8), Operand(9), Operand(2), Operand(4), Operator(Sub), Operator(Expoent), Operator(Mul), Operator(Sub)]
    // ! -> 3 5 2 ^ * 5 / 8 9 2 4 - ^ * - => 
    let rpn_result_8 = Calculator::rpn("3*5^2/5-8*9^(2-4)");
    match rpn_result_8 {
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
            )
        }
        Err(_) => {}
    };
    println!("End rpn_result_8");
}
