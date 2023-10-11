use std::ops::ControlFlow;

use crate::{
    global::str_utils::slice_string, process_args::utils, process_img::models::{ProcessImageObj, ResizeForm},
    response::Response,
};

use super::save_img;

pub fn process(third_op: &str, args: &Vec<String>, mut image: ProcessImageObj) -> Response {
    let mut response = Response::default();
    if let ControlFlow::Break(_) = utils::check_min_parameters(args, &mut response, 9) {
        return response;
    }
    if third_op == "--b" || third_op == "--c" {
        let fourth_op: &str = args[5].as_ref();
        if fourth_op == "--d" {
            let dimensions_str: &str = args[6].as_ref();
            let width_str = slice_string(dimensions_str, ',', true);
            let width_parse_res = width_str.parse::<u32>();
            match width_parse_res {
                Ok(width) => {
                    let height_str = slice_string(dimensions_str, ',', false);
                    let height_parse_res = height_str.parse::<u32>();
                    match height_parse_res {
                        Ok(height) => {
                            image.dimensions = (width, height);
                        }
                        Err(_) => {
                            response.message = "Invalid value for height".to_string();
                            return response;
                        }
                    }
                }
                Err(_) => {
                    response.message = "Invalid value for width".to_string();
                    return response;
                }
            }
        } else {
            //* fourth_op must be --d
            response.message =
                format!("'{}' is not a known parameter for this position", fourth_op);
            return response;
        }

        let mut save_path: String = String::default();
        if let ControlFlow::Break(_) = utils::file_path_output_is_empty(
            args[7].as_ref(),
            args[8].as_ref(),
            &mut response,
            &mut save_path,
        ) {
            return response;
        }

        let mut file_name: &str = "";
        if args.len() == 11 {
            let file_name_op: &str = args[9].as_ref();
            if file_name_op == "--n" {
                file_name = args[10].as_ref();
            }
        }

        if file_name.is_empty() {
            save_path += "/resize.png";
        } else {
            save_path += file_name; // TODO remove possible extension
            save_path += ".png";
        }
    
        let mut resize_form = ResizeForm::Bilinear();
        if third_op == "--c" {
            resize_form = ResizeForm::Bicubic();
        }

        let resize_img = image.resize(resize_form);
        save_img(resize_img, save_path, &mut response);

    } else {
        //* third_op must be --b || --c
        response.message = format!("'{}' is not a known parameter for this position", third_op);
    }

    response
}
