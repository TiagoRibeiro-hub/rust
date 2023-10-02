mod calculator;

use calculator::Calculator;

fn main() {
    let rpn_result = Calculator::rpn("((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1))");

    match rpn_result {
        Ok(rpn) => println!("rpn: {rpn:?}"),  // 15 7 1 1 + − / 3 * 2 1 1 + + −
        Err(e) => println!("error: {e:?}"),
    }
}

// 4 - (3 * 4 + 5 / 2 ) + 4 => 
// 4 - (3 * 4 + (5 / 2 - 2) ) + 4 => 
// ((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1)) => 15 7 1 1 + − / 3 * 2 1 1 + + − => 5


// https://www.omnicalculator.com/math/polish-notation