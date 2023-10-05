use std::ops::ControlFlow;

use super::utils;
use crate::{process_img::Image, response::Response};
mod color_scale;
mod pixelate;

pub fn process_img(args: &Vec<String>) -> Response {
    let (mut response, file_path, second_op, third_op) = match args_validation(args) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let image = Image::from(file_path);
    let mut saved: Result<(), image::ImageError> = Ok(());

    if second_op == "--cs" {
        response = color_scale::process(third_op, args, image, &mut saved);
    } else if second_op == "--p" {
        response = pixelate::process(third_op, args, image, &mut saved);
    } else if second_op == "--a" {
        todo!("ascii");
    }

    if response.succeed {
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

    response
}

fn args_validation(args: &Vec<String>) -> Result<(Response, &str, &str, &str), Response> {
    let mut response = Response::default();
    if let ControlFlow::Break(_) = utils::check_parameters(args, &mut response, 11) {
        return Err(response);
    }
    let file_path: &str = args[2].as_ref();
    if file_path.is_empty() {
        response.message = "File path is empty".to_string();
        return Err(response);
    }
    let second_op: &str = args[3].as_ref();
    if second_op != "--cs" && second_op != "--p" && second_op != "--a" {
        response.message = format!("'{}' is not a known parameter for this position", second_op);
        return Err(response);
    }
    let third_op: &str = args[4].as_ref();
    if third_op != "--o"
        && third_op != "--gs"
        && third_op != "--bs"
        && third_op != "--grs"
        && third_op != "--rs"
        && third_op != "--D"
    {
        response.message = format!("'{}' is not a known parameter for this position", third_op);
        return Err(response);
    }
    Ok((response, file_path, second_op, third_op))
}
