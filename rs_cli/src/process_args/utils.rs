use std::ops::ControlFlow;

use crate::{calculator::Calculator, process_img::Image, response::Response};

fn check_parameters(
    args: &Vec<String>,
    response: &mut Response,
    nr_args: usize,
) -> ControlFlow<()> {
    if args.len() > nr_args {
        response.message = "Too many parameters".to_string();
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

pub fn calculator(args: &Vec<String>, response: &mut Response) {
    if let ControlFlow::Break(_) = check_parameters(args, response, 4) {
        return;
    }
    let expr: &str = args[2].as_ref();
    let rpn_result = Calculator::rpn(expr);
    match rpn_result {
        Ok(rpn) => {
            let result = Calculator::evaluate(rpn.clone());
            match result {
                Ok(final_result) => {
                    if args.len() == 4 {
                        let show_rpn: &str = args[3].as_ref();
                        if show_rpn == "--s" {
                            response.message = format!(
                                "Expression: {},\nRPN: {},\nResult: {}",
                                expr,
                                Calculator::display_rpn(rpn),
                                final_result
                            );
                            response.succeed = true;
                        } else {
                            response.message = format!("'{}' is not a known parameter", show_rpn);
                        };
                    } else {
                        response.message =
                            format!("The result for '{}' is: {}", expr, final_result);
                        response.succeed = true;
                    };
                }
                Err(e) => response.message = format!("{}", e),
            }
        }
        Err(e) => response.message = format!("{}", e),
    }
}

pub fn process_img(args: &Vec<String>, response: &mut Response) {
    if let ControlFlow::Break(_) = check_parameters(args, response, 7) {
        return;
    }
    let file_path: &str = args[2].as_ref();
    if file_path.is_empty() {
        response.message = "File path is empty".to_string();
        return;
    }
    let second_op: &str = args[3].as_ref();
    if second_op != "--gs" || second_op != "--p" || second_op != "--a" {
        response.message = format!("'{}' is not a known parameter", second_op);
        return;
    }

    let third_op: &str = args[4].as_ref();
    if third_op != "--o" || third_op != "--G" || third_op != "--D" {
        response.message = format!("'{}' is not a known parameter", third_op);
        return;
    }

    let mut save_path: String = String::default();
    if let ControlFlow::Break(_) =
        file_path_output_is_empty(third_op, args[5].as_ref(), response, &mut save_path)
    {
        return;
    }

    let mut image = Image::from(file_path);
    let mut saved: Result<(), image::ImageError> = Ok(());

    if second_op == "--gs" {
        if third_op == "--G" {
            let gama_str: &str = args[5].as_ref();
            let gama_parse_res = gama_str.parse::<f64>();
            match gama_parse_res {
                Ok(gama) => image.gama = gama,
                Err(e) => {
                    response.message = e.to_string();
                    return;
                }
            }
        }
        let gray_img = image.gray_scale();
        if let ControlFlow::Break(_) =
            file_path_output_is_empty(args[6].as_ref(), args[7].as_ref(), response, &mut save_path)
        {
            return;
        }
        save_path += "/gray_scale.png";
        saved = gray_img.save(save_path);
    } else if second_op == "--p" {
        if third_op == "--D" {
            //let dimensions_str: &str = args[5].as_ref();
        }
        let pixelate_img = image.pixelate();
        if let ControlFlow::Break(_) =
            file_path_output_is_empty(args[6].as_ref(), args[7].as_ref(), response, &mut save_path)
        {
            return;
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

fn file_path_output_is_empty(
    arg: &str,
    arg_value: &str,
    response: &mut Response,
    save_path: &mut String,
) -> ControlFlow<()> {
    if arg == "--o" {
        let path: &str = arg_value;
        if path.is_empty() {
            response.message = "File path to save is empty".to_string();
            return ControlFlow::Break(());
        }
        *save_path = path.to_string();
    }
    ControlFlow::Continue(())
}
