use std::ops::ControlFlow;

use crate::{process_img::{ProcessImageObj, models::Filter}, response::Response, process_args::utils};

use super::save_img;

pub fn process(third_op: &str, args: &Vec<String>, mut image: ProcessImageObj) -> Response {
    let mut response = Response::default();
    if let ControlFlow::Break(_) = utils::check_min_parameters(args, &mut response, 7) {
        return response;
    }
    if third_op == "--b" {
        let fourth_op: &str = args[5].as_ref();
        if fourth_op == "--s" {
            let fourth_op_arg: &str = args[6].as_ref();
            let n_size_res = fourth_op_arg.parse::<u32>();
            match n_size_res {
                Ok(size) => {
                    if size % 2 == 0 {
                        response.message = format!("'{}' must be an odd number", size);
                        return response;
                    }
                    image.n_size = size;
                },
                Err(_) => {
                    response.message = "Invalid value for size".to_string();
                    return response;
                },
            }
        }
        else if fourth_op != "--o" {
            //* fourth_op must be --s || --o
            response.message =
            format!("'{}' is not a known parameter for this position", fourth_op);
            return response;
        }

        let mut save_path: String = String::default();

        let mut output_arg = fourth_op;
        let output_value: &str;
        let mut file_name: &str = "";

        if output_arg == "--o" {
            output_value = args[6].as_ref();
            if args.len() == 9 {
                let file_name_op: &str = args[7].as_ref();
                if file_name_op == "--n" {
                    file_name = args[8].as_ref();
                }
            }
        }
        else {
            output_arg = args[7].as_ref();
            output_value = args[8].as_ref();
            if args.len() == 11 {
                let file_name_op: &str = args[9].as_ref();
                if file_name_op == "--n" {
                    file_name = args[10].as_ref();
                }
            }
        }
        if let ControlFlow::Break(_) = utils::file_path_output_is_empty(
            output_arg,
            output_value,
            &mut response,
            &mut save_path,
        ) {
            return response;
        }

        


        if file_name.is_empty() {
            save_path += "/filter.png";
        } else {
            save_path += file_name; // TODO remove possible extension
            save_path += ".png";
        }
    
        let filter = Filter::Box();


        let resize_img = image.filter(filter);
        save_img(resize_img, save_path, &mut response);
    }
    else {
        //* third_op must be --b
        response.message = format!("'{}' is not a known parameter for this position", third_op);
    }

    response
}