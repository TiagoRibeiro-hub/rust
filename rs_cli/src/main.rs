mod calculator;

use calculator::Calculator;

fn main() {
    let rpn_result = Calculator::rpn("3/(5+8*9)");

    match rpn_result {
        Ok(rpn) => {
            let result = Calculator::evaluate(rpn);
            match result {
                Ok(final_result) => println!("error: {final_result:?}"),
                Err(e) => println!("error: {e:?}"),
            }
        }, 
        Err(e) => println!("error: {e:?}"),
    }
}

