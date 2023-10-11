use std::ops::ControlFlow;

use super::utils;
use crate::{process_img::models::ProcessImageObj, response::Response};
mod color_scale;
mod pixelate;
mod resize;

pub fn process_img(args: Vec<String>) -> Response {
    let (mut response, file_path, second_op, third_op) = match args_validation(&args) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let image = ProcessImageObj::from(file_path);
    if second_op == "--cs" {
        response = color_scale::process(third_op, &args, image);
    } else if second_op == "--p" {
        response = pixelate::process(third_op, &args, image);
    } else if second_op == "--r" {
        response = resize::process(third_op, &args, image);
    }

    if response.succeed {
        response.message = "Image processed and saved.".to_string();
    } else {
        response.message = "Unable to save image".to_string();
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
    if ["--cs", "--p", "--r"].contains(&second_op) {
        response.message = format!("'{}' is not a known parameter for this position", second_op);
        return Err(response);
    }
    let third_op: &str = args[4].as_ref();
    if ["--o", "--gs", "--bs", "--grs", "--rs", "--b", "--c"].contains(&third_op) {
        response.message = format!("'{}' is not a known parameter for this position", third_op);
        return Err(response);
    }
    Ok((response, file_path, second_op, third_op))
}

fn save_img(img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, save_path: String) -> bool {
    let saved = img.save(save_path);
    match saved {
        Ok(_) => true,
        Err(_) => false,
    }
}
