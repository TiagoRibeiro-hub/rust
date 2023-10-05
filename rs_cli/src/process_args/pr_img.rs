use std::ops::ControlFlow;

use super::utils;
use crate::{
    global,
    process_img::{ColorScale, Image},
    response::Response,
};

pub fn process_img(args: &Vec<String>) -> Response {
    let (mut response, file_path, second_op, third_op) = match args_validation(args) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let image = Image::from(file_path);
    let mut saved: Result<(), image::ImageError> = Ok(());

    if second_op == "--cs" {
        response = color_scale(third_op, args, image, &mut saved);
    } else if second_op == "--p" {
        response = pixelate(third_op, args, image, &mut saved);
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

fn color_scale(
    third_op: &str,
    args: &Vec<String>,
    mut image: Image,
    saved: &mut Result<(), image::ImageError>,
) -> Response {
    let mut response = Response::default();
    if third_op == "--gs" || third_op == "--bs" || third_op == "--grs" || third_op == "--rs" {
        let fourth_op: &str = args[5].as_ref();
        if fourth_op == "--G" {
            let gama_str: &str = args[6].as_ref();
            let gama_parse_res = gama_str.parse::<f64>();
            match gama_parse_res {
                Ok(gama) => {
                    image.gama = gama;
                }
                Err(_) => {
                    response.message = "Invalid value for gama".to_string();
                    return response;
                }
            }
        } else if fourth_op != "--o" {
            //* fourth_op must be --G or --o
            response.message =
                format!("'{}' is not a known parameter for this position", fourth_op);
            return response;
        }

        let mut color = ColorScale::defaut();
        if third_op == "--bs" {
            color = ColorScale::Blue();
        } else if third_op == "--grs" {
            color = ColorScale::Green();
        } else if third_op == "--rs" {
            color = ColorScale::Red();
        }
        let gray_img = image.color_scale(color);

        let mut save_path: String = String::default();
        //* here fourth_op was --o
        let mut arg_op = fourth_op;
        let mut arg_param = args[6].as_ref();
        if fourth_op != "--o" {
            //* here fourth_op was --G
            arg_op = args[7].as_ref();
            arg_param = args[8].as_ref();
        }

        if let ControlFlow::Break(_) =
            utils::file_path_output_is_empty(arg_op, arg_param, &mut response, &mut save_path)
        {
            return response;
        }
        save_path += "/gray_scale.png";
        *saved = gray_img.save(save_path);
        response.succeed = true;
    } else {
        response.message = format!("'{}' is not a known parameter for this position", third_op);
        return response;
    }

    response
}

fn pixelate(
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
                let width_str = global::slice_string(dimensions_str, ',', true);
                let width_parse_res = width_str.parse::<u32>();
                match width_parse_res {
                    Ok(width) => {
                        let height_str = global::slice_string(dimensions_str, ',', false);
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

    if third_op != "--o" {
        //* here third_op was --D
        arg_op = args[6].as_ref();
        arg_param = args[7].as_ref();
    }

    if let ControlFlow::Break(_) =
        utils::file_path_output_is_empty(arg_op, arg_param, &mut response, &mut save_path)
    {
        return response;
    }

    save_path += "/pixelated.png";
    *saved = pixelate_img.save(save_path);
    response.succeed = true;

    response
}

fn args_validation(args: &Vec<String>) -> Result<(Response, &str, &str, &str), Response> {
    let mut response = Response::default();
    if let ControlFlow::Break(_) = utils::check_parameters(args, &mut response, 9) {
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
