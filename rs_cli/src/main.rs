mod calculator;

use calculator::Calculator;

fn main() {
    let rpn_result = Calculator::rpn("3/(5+8*9)");

    match rpn_result {
        Ok(rpn) => println!("rpn: {rpn:?}"),  
        Err(e) => println!("error: {e:?}"),
    }
}

// TODO TESTS:
// TODO "6*3-(4-5)+2" -> [Operand(63), Operator(Mul), Operand(45), Operator(Sub), Operator(Sub), Operand(2), Operator(Add)] -> 63*45--2+ = 21
// TODO "(1+2)*(3+4)" -> [Operand(12), Operator(Add), Operand(34), Operator(Add), Operator(Mul)] -> 12+34+* = 21
// TODO "1+2*3+4" -> [Operand(123), Operator(Mul), Operand(4), Operator(Add), Operator(Add)] -> 123*4++ = 11
// TODO "1+2*(3+4)" -> [Operand(1234), Operator(Add), Operator(Mul), Operator(Add)] -> 1234+*+ = 15
// TODO "(1+2)*3+4" -> [Operand(12), Operator(Add), Operand(3), Operator(Mul), Operand(4), Operator(Add)] -> 12+3*4+ = 13
// TODO "(2/4)*(5-6)" -> [Operand(24), Operator(Div), Operand(56), Operator(Sub), Operator(Mul)] -> 24/56-* = -0.5
// TODO "3/(5+8*9)" -> [Operand(3589), Operator(Mul), Operator(Add), Operator(Div)] -> 3589*+/ = 7