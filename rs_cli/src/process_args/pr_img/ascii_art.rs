use crate::{response::Response, process_img::Image};

#[allow(unused_variables, dead_code, unused_mut)]
pub fn process(
    third_op: &str,
    args: &Vec<String>,
    mut image: Image,
    saved: &mut Result<(), image::ImageError>,
) -> Response {
    let mut response = Response::default();
    let assci_art_res = image.ascii_art();
    match  assci_art_res {
        Ok(ascci_art) => {},
        Err(e) => {},
    }
    response
}