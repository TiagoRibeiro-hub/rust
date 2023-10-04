use std::ops::ControlFlow;

use super::utils;
use crate::{global, process_img::Image, response::Response};

pub fn process_img(args: &Vec<String>, response: &mut Response) {
    if let ControlFlow::Break(_) = utils::check_parameters(args, response, 8) {
        return;
    }
    let file_path: &str = args[2].as_ref();
    if file_path.is_empty() {
        response.message = "File path is empty".to_string();
        return;
    }
    let second_op: &str = args[3].as_ref();
    if second_op != "--gs" && second_op != "--p" && second_op != "--a" {
        response.message = format!("'{}' is not a known parameter", second_op);
        return;
    }

    let third_op: &str = args[4].as_ref();
    if third_op != "--o" && third_op != "--G" && third_op != "--D" {
        response.message = format!("'{}' is not a known parameter", third_op);
        return;
    }

    let mut save_path: String = String::default();
    if let ControlFlow::Break(_) =
        utils::file_path_output_is_empty(third_op, args[5].as_ref(), response, &mut save_path)
    {
        return;
    }

    let mut image = Image::from(file_path);
    let mut saved: Result<(), image::ImageError> = Ok(());
    let mut needs_save_path = false;

    if second_op == "--gs" {
        if third_op == "--G" {
            let gama_str: &str = args[5].as_ref();
            let gama_parse_res = gama_str.parse::<f64>();
            match gama_parse_res {
                Ok(gama) => {
                    image.gama = gama;
                    needs_save_path = true
                }
                Err(_) => {
                    response.message = "Invalid value for gama".to_string();
                    return;
                }
            }
        }
        let gray_img = image.gray_scale();
        if needs_save_path {
            if let ControlFlow::Break(_) = get_last_arg_saved_path(args, response, &mut save_path) {
                return;
            }
        }
        save_path += "/gray_scale.png";
        saved = gray_img.save(save_path);
    } else if second_op == "--p" {
        if third_op == "--D" {
            let dimensions_str: &str = args[5].as_ref();
            let idx_res = dimensions_str.find(',');
            match idx_res {
                Some(_) => {
                    let width_str = global::slice_string(dimensions_str, ',', true);
                    let width_parse_res = width_str.parse::<u32>();
                    match width_parse_res {
                        Ok(width) => {
                            let height_str = global::slice_string(dimensions_str, ',', false);
                            let height_parse_res = height_str.parse::<u32>();
                            match height_parse_res {
                                Ok(height) => {
                                    image.dimensions = (width, height);
                                    needs_save_path = true
                                }
                                Err(_) => {
                                    response.message = "Invalid value for height".to_string();
                                    return;
                                }
                            }
                        }
                        Err(_) => {
                            response.message = "Invalid value for width".to_string();
                            return;
                        }
                    }
                }
                None => {
                    response.message = "Invalid value for dimension".to_string();
                    return;
                }
            }
        }
        let pixelate_img = image.pixelate();
        if needs_save_path {
            if let ControlFlow::Break(_) = get_last_arg_saved_path(args, response, &mut save_path) {
                return;
            }
        }
        save_path += "/pixelated.png";
        saved = pixelate_img.save(save_path);
    } else if second_op == "--a" {
        todo!("ascii");
    }

    match saved {
        Ok(_) => {
            response.message = "Image processed and saved.".to_string();
            response.succeed = true;
        }
        Err(_) => {
            response.message = "Unable to save image".to_string();
        }
    }
}

fn get_last_arg_saved_path(args: &Vec<String>, response: &mut Response, save_path: &mut String) -> ControlFlow<()> {
    if let ControlFlow::Break(_) = utils::file_path_output_is_empty(
        args[6].as_ref(),
        args[7].as_ref(),
        response,
        save_path,
    ) {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}
