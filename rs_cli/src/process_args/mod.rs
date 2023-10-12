use std::{env::args, ops::ControlFlow};
use crate::response::Response;

mod utils;
mod calc;
mod pr_img;

pub fn process_args() -> Response {
    let args: Vec<String> = args().collect();
    let mut response = Response::default();
    
    if let ControlFlow::Break(_) = utils::check_min_parameters(&args, &mut response, 3) {
        return response;
    }

    match args.get(1) {
        Some(op) => {
            if op == "--calc" {
                response = utils::calculator(args);
            }
            else if op == "--img" {
                response = utils::process_img(args);
            }
            else{
                response.message = format!("There is not an operation '{}'", op);
            };
        }
        None =>  response.message = "There is no operation".to_string(),
    }

    response
}

