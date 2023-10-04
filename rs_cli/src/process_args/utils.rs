use std::ops::ControlFlow;
use crate::response::Response;

use super::{calc, pr_img};


pub fn check_parameters(
    args: &Vec<String>,
    response: &mut Response,
    nr_args: usize,
) -> ControlFlow<()> {
    if args.len() > nr_args {
        response.message = "Too many parameters".to_string();
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

pub fn file_path_output_is_empty(
    arg: &str,
    arg_value: &str,
    response: &mut Response,
    save_path: &mut String,
) -> ControlFlow<()> {
    if arg == "--o" {
        let path: &str = arg_value;
        if path.is_empty() {
            response.message = "File path output is empty".to_string();
            return ControlFlow::Break(());
        }
        *save_path = path.to_string();
    }
    ControlFlow::Continue(())
}

pub fn calculator(args: &Vec<String>, response: &mut Response) {
    calc::calculator(args, response)
}

pub fn process_img(args: &Vec<String>, response: &mut Response) {
    pr_img::process_img(args, response)
}


