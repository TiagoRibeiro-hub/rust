use std::env::args;
use crate::response::Response;

mod utils;
mod calc;
mod pr_img;

pub fn process_args() -> Response {
    let args: Vec<String> = args().collect();
    let mut response = Response::default();
    
    if args.len() < 3 {
        response.message = "Must have at least 2 args".to_string();
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
                response.message = format!("There is no operation '{}'", op);
            };
        }
        None =>  response.message = "No parameters allowed".to_string(),
    }

    response
}

