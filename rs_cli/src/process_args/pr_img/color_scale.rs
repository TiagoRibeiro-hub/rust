use std::ops::ControlFlow;

use super::{utils, save_img};
use crate::{
    process_img::models::{ColorScale, ProcessImageObj},
    response::Response,
};

pub fn process(third_op: &str, args: &Vec<String>, mut image: ProcessImageObj) -> Response {
    let mut response = Response::default();
    if let ControlFlow::Break(_) = utils::check_min_parameters(args, &mut response, 9) {
        return response;
    }
    if third_op == "--gs" || third_op == "--bs" || third_op == "--grs" || third_op == "--rs" {
        let fourth_op: &str = args[5].as_ref();
        if fourth_op == "--g" {
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

        let mut file_name: &str = "";
        if fourth_op != "--o" {
            //* here fourth_op was --G
            arg_op = args[7].as_ref();
            arg_param = args[8].as_ref();
            if args.len() == 10 {
                let file_name_op: &str = args[9].as_ref();
                if file_name_op == "--n" {
                    file_name = args[10].as_ref();
                }
            }
        } else if args.len() == 8 {
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
        save_img(gray_img, save_path, &mut response);
    } else {
        //* third_op must be --gs || --bs || --grs || --rs
        response.message = format!("'{}' is not a known parameter for this position", third_op);
        return response;
    }

    response
}


