use std::ops::ControlFlow;

use crate::{calculator::Calculator, response::Response};

use super::utils;

pub fn calculator(args: Vec<String>) -> Response {
    let mut response = Response::default();
    if let ControlFlow::Break(_) = utils::check_max_parameters(&args, &mut response, 4) {
        return response;
    }
    let expr: &str = args[2].as_ref();
    let rpn_result = Calculator::rpn(expr);
    match rpn_result {
        Ok(rpn) => {
            let result = Calculator::evaluate(rpn.clone());
            match result {
                Ok(final_result) => {
                    if args.len() == 4 {
                        let show_rpn: &str = args[3].as_ref();
                        if show_rpn == "--s" {
                            response.message = format!(
                                "Expression: {},\nRPN: {},\nResult: {}",
                                expr,
                                Calculator::display_rpn(rpn),
                                final_result
                            );
                            response.succeed = true;
                        } else {
                            response.message = format!("'{}' is not a known parameter for this position", show_rpn);
                        };
                    } else {
                        response.message =
                            format!("The result for '{}' is: {}", expr, final_result);
                        response.succeed = true;
                    };
                }
                Err(e) => response.message = format!("{}", e),
            }
        }
        Err(e) => response.message = format!("{}", e),
    }

    response
}