use crate::{process_args::utils, process_img::Image, response::Response, global::str_utils};
use std::ops::ControlFlow;

pub fn process(
    third_op: &str,
    args: &Vec<String>,
    mut image: Image,
    saved: &mut Result<(), image::ImageError>,
) -> Response {
    let mut response = Response::default();
    if third_op == "--D" {
        let dimensions_str: &str = args[5].as_ref();
        let idx_res = dimensions_str.find(',');
        match idx_res {
            Some(_) => {
                let width_str = str_utils::slice_string(dimensions_str, ',', true);
                let width_parse_res = width_str.parse::<u32>();
                match width_parse_res {
                    Ok(width) => {
                        let height_str = str_utils::slice_string(dimensions_str, ',', false);
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
            }
            None => {
                response.message = "Invalid value for dimension".to_string();
                return response;
            }
        }
    } else if third_op != "--o" {
        //* third_op must be or --D or --o
        response.message = format!("'{}' is not a known parameter for this position", third_op);
        return response;
    }
    let pixelate_img = image.pixelate();

    let mut save_path: String = String::default();
    //* here third_op was --o
    let mut arg_op = third_op;
    let mut arg_param = args[5].as_ref();

    let mut file_name: &str = "";
    if third_op != "--o" {
        //* here third_op was --D
        arg_op = args[6].as_ref();
        arg_param = args[7].as_ref();
        if args.len() == 9 {
            let file_name_op: &str = args[8].as_ref();
            if file_name_op == "--n" {
                file_name = args[9].as_ref();
            }
        }
    }
    else if args.len() == 7 {
        let file_name_op: &str  = args[6].as_ref();
        if file_name_op == "--n" {
            file_name = args[7].as_ref();
        }
    }

    if let ControlFlow::Break(_) =
        utils::file_path_output_is_empty(arg_op, arg_param, &mut response, &mut save_path)
    {
        return response;
    }

    if file_name.is_empty() {
        save_path += "/pixelated.png";
    }
    else {
        save_path += file_name; // TODO remove possible extension
        save_path += ".png";
    }

    *saved = pixelate_img.save(save_path);
    response.succeed = true;

    response
}
