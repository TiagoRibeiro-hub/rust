use crate::{
    global::str_utils, process_args::utils, process_img::models::ProcessImageObj,
    response::Response,
};
use std::ops::ControlFlow;

use super::save_img;

pub fn process(third_op: &str, args: &Vec<String>, mut image: ProcessImageObj) -> Response {
    let mut response = Response::default();
    if third_op != "--o" {
        //* third_op must be --o
        response.message = format!("'{}' is not a known parameter for this position", third_op);
        return response;
    }
    let pixelate_img = image.pixelate();

    let arg_op = third_op;
    let arg_param = args[5].as_ref();

    let mut file_name: &str = "";
    if args.len() == 8 {
        let file_name_op: &str = args[6].as_ref();
        if file_name_op == "--n" {
            file_name = args[7].as_ref();
        }
    }

    let mut save_path: String = String::default();
    if let ControlFlow::Break(_) =
        utils::file_path_output_is_empty(arg_op, arg_param, &mut response, &mut save_path)
    {
        return response;
    }

    if file_name.is_empty() {
        save_path += "/pixelated.png";
    } else {
        save_path += file_name; // TODO remove possible extension
        save_path += ".png";
    }

    save_img(pixelate_img, save_path, &mut response);
    response
}
