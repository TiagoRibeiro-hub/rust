use crate::global::Response;
use std::env::args;

mod utils;

pub fn process_args() -> Response {
    let args: Vec<String> = args().collect();
    let mut response = Response { message: String::from("Something went wrong"), succeed: false };

    if args.len() != 3 {
        response.message = format!("At least 2 args");
        return response;
    }

    let type_op = args.get(1);

    match type_op {
        Some(op) => {
            if op == "calc" {
                let calc_res = utils::calculator(args);
                match calc_res {
                    Ok(res) => {
                        response.message = res; 
                        response.succeed = true;
                    },
                    Err(e) => {
                        response.message = format!("{}", e);
                    },
                };
            }
            else{
                response.message = format!("There is no operation {}", op);
            };
        }
        None =>  response.message = format!("There is no parameter"),
    }

    response
}
