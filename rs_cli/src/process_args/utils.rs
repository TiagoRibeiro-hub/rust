use crate::{calculator::Calculator, global::CustomError};

pub fn calculator(args: Vec<String>) -> Result<String, CustomError> {
    let expr: &str = args[2].as_ref();
    let rpn_result = Calculator::rpn(&expr);
    match rpn_result {
        Ok(rpn) => {
            let result = Calculator::evaluate(rpn);
            match result {
                Ok(final_result) => return Ok(format!("The result is: {}", final_result)),
                Err(e) => return Err(e),
            }
        }
        Err(e) => return Err(e),
    }
}
