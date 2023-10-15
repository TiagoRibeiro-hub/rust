use std::ops::ControlFlow;

use super::{utils, save_img};
use crate::{
    process_img::{models::ColorsProcesses, ProcessImageObj},
    response::Response,
};

pub fn process(third_op: &str, args: &Vec<String>, mut image: ProcessImageObj) -> Response {
    let mut response = Response::default();
    if let ControlFlow::Break(_) = utils::check_min_parameters(args, &mut response, 9) {
        return response;
    }
    let fourth_op: &str = args[5].as_ref();
    if third_op == "--gs" || third_op == "--bs" || third_op == "--grs" || third_op == "--rs" || third_op == "--i" {
        if fourth_op != "--o" {
            //* fourth_op must be --o
            response.message =
                format!("'{}' is not a known parameter for this position", fourth_op);
            return response;
        }
    } 
    else if third_op == "--d" || third_op == "--l" || third_op == "--lc" || third_op == "--hc" {
        if fourth_op == "--g" {
            let gama_str: &str = args[6].as_ref();
            let gama_parse_res = gama_str.parse::<f64>();
            match gama_parse_res {
                Ok(gama_percentage) => {
                    image.gama = (gama_percentage * 255.0 / 100.0) as u8;
                }
                Err(_) => {
                    response.message = "Invalid value for gama".to_string();
                    return response;
                }
            }
        } else if fourth_op != "--o" {
            //* fourth_op must be --g or --o
            response.message =
                format!("'{}' is not a known parameter for this position", fourth_op);
            return response;
        }
    }
    else {
        //* third_op must be --gs || --bs || --grs || --rs
        response.message = format!("'{}' is not a known parameter for this position", third_op);
        return response;
    }

    let mut color = ColorsProcesses::Gray();
    if third_op == "--bs" {
        color = ColorsProcesses::Blue();
    } else if third_op == "--grs" {
        color = ColorsProcesses::Green();
    } else if third_op == "--rs" {
        color = ColorsProcesses::Red();
    }
    else if third_op == "--d" {
        color = ColorsProcesses::Darken();
    }
    else if third_op == "--l" {
        color = ColorsProcesses::Lighten();
    }
    else if third_op == "--i" {
        color = ColorsProcesses::Invert();
    }
    else if third_op == "--lc" {
        color = ColorsProcesses::LowContrast();
    }
    else if third_op == "--hc" {
        color = ColorsProcesses::HighContrast();
    }
    let color_img = image.color_scale(color);

    let mut save_path: String = String::default();
    //* here fourth_op was --o
    let mut arg_op = fourth_op;
    let mut arg_param = args[6].as_ref();

    let mut file_name: &str = "";
    if fourth_op != "--o" {
        //* here fourth_op was --G
        arg_op = args[7].as_ref();
        arg_param = args[8].as_ref();
        if args.len() == 11 {
            let file_name_op: &str = args[9].as_ref();
            if file_name_op == "--n" {
                file_name = args[10].as_ref();
            }
        }
    } else if args.len() == 9 {
        let file_name_op: &str = args[7].as_ref();
        if file_name_op == "--n" {
            file_name = args[8].as_ref();
        }
    }

    if let ControlFlow::Break(_) =
        utils::file_path_output_is_empty(arg_op, arg_param, &mut response, &mut save_path)
    {
        return response;
    }

    if file_name.is_empty() {
        save_path += "/color_scale.png";
    } else {
        save_path += file_name; // TODO remove possible extension
        save_path += ".png";
    }
    save_img(color_img, save_path, &mut response);

    response
}


