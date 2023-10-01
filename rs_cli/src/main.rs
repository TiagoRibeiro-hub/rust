mod calculator;

use calculator::Calculator;

fn main() {
    let rpn_result = Calculator::rpn("3/(5+8*9)");

    match rpn_result {
        Ok(rpn) => println!("rpn: {rpn:?}"),  
        Err(e) => println!("error: {e:?}"),
    }
}

