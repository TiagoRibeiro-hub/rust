mod calculator;

use calculator::Calculator;

fn main() {
    let rpn_result = Calculator::rpn("((15/(7-(1+1)))*3)-((2+(1+1))+5)");

    match rpn_result {
        Ok(rpn) => println!("rpn: {rpn:?}"), 
        Err(e) => println!("error: {e:?}"),
    }
}

