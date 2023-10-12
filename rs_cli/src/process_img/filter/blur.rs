use crate::process_img::models::{ProcessImageObj, ImageRgba, ImgDimensions};


pub fn gaussian(image: &ProcessImageObj) -> ImageRgba {
    let (old_img, dimensions, scale_factor, mut new_img) = image.set_props_for_processing();


    for y in 0..dimensions.new_dim.1 {
        for x in 0..dimensions.new_dim.0 {

            let (original_y, original_x) = ImgDimensions::map_original_coordinates(y, x, scale_factor);
            let (y1, y2, x1, x2) = dimensions.map_surrounding_coordinates(original_y, original_x);
            let (y0, y3) = dimensions.map_edge_coordinates(y1, y2, (dimensions.old_dim.1) - 1);
            let (x0, x3) = dimensions.map_edge_coordinates(x1, x2, (dimensions.old_dim.0) - 1);

            let pixel: &mut image::Rgba<u8> = new_img.get_pixel_mut(x, y);

        }
    }

    new_img
}


#[test]
fn filters() {
    // original 800 x 596
    let image =
        ProcessImageObj::from("/home/tiago/rust/projects/cli/imgs/chestnut_tailed_starling.jpg");
    // ! Gaussian
    let result = gaussian(&image);
    let _ = result.save("/home/tiago/rust/projects/cli/imgs/resize_bicubic_up_scaling.png");
}