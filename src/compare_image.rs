use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::Rng;

#[derive(Debug)]
pub struct ShapeInfo(u32, u32, u32, u32, [u8; 4]);

impl ShapeInfo {
    // Create a random shape
    pub fn make_random(
        max_x: u32,
        max_y: u32,
        original_cost: u32,
        current_cost: u32,
        options: &Vec<[u8; 4]>,
    ) -> ShapeInfo {
        // Create a random thread
        let mut rng = rand::thread_rng();

        // Calculate the weighted max width and height for the shape
        let weighted_max_width =
            (max_x as f64 * (current_cost as f64 * 2.0_f64 / original_cost as f64)) as u32;
        let weighted_max_height =
            (max_y as f64 * (current_cost as f64 * 2.0_f64 / original_cost as f64)) as u32;

        // Randomly select an (x, y) coordinate to place the shape
        let x = rng
            .gen_range((0 - (weighted_max_width as f32 / 2.0) as i32)..(max_x as i32)) // Generate random number
            .max(0) as u32; // Ensure that that number is larger than 0
        let y = rng
            .gen_range((0 - (weighted_max_height as f32 / 2.0) as i32)..(max_y as i32)) // Generate random number
            .max(0) as u32; // Ensure that that number is larger than 0

        // Randomly select the width and height of the shape
        let width = rng.gen_range(1..(weighted_max_width + 3));
        let width = if x + width >= max_x {
            max_x - x
        } else {
            width - 1
        };
        let height = rng.gen_range(1..(weighted_max_height + 3));
        let height = if y + height >= max_y {
            max_y - y
        } else {
            height - 1
        };

        // ShapeInfo(x, y, width, height, [r, g, b, 255])
        ShapeInfo(
            x,
            y,
            width,
            height,
            options[rng.gen_range(0..options.len())],
        )
    }
    // Create a random shape at a coordinate following the logic above
    /*
    pub fn make_using_edge(
        max_x: u32,
        max_y: u32,
        original_cost: u32,
        current_cost: u32,
        options: &Vec<[u8; 4]>,
        coords: (u32, u32),
    ) -> ShapeInfo {
        let mut rng = rand::thread_rng();

        let weighted_max_width =
            (max_x as f64 * (current_cost as f64 * 2.0_f64 / original_cost as f64)) as u32;
        let weighted_max_height =
            (max_y as f64 * (current_cost as f64 * 2.0_f64 / original_cost as f64)) as u32;
        let (x, y) = coords;
        let width = rng.gen_range(1..(weighted_max_width + 3));
        let width = if x + width >= max_x {
            max_x - x
        } else {
            width - 1
        };
        let height = rng.gen_range(1..(weighted_max_height + 3));
        let height = if y + height >= max_y {
            max_y - y
        } else {
            height - 1
        };
        ShapeInfo(
            x,
            y,
            width,
            height,
            options[rng.gen_range(0..options.len())],
        )
    }
     */
}

// Draw the shape onto the canvas
pub fn draw(canvas: &mut DynamicImage, shape: ShapeInfo) {
    for y in shape.1..(shape.1 + shape.3) {
        for x in shape.0..(shape.0 + shape.2) {
            if x < canvas.width() && y < canvas.height() {
                let rgb = shape.4;
                let r = rgb[0];
                let g = rgb[1];
                let b = rgb[2];
                canvas.put_pixel(x, y, Rgba([r, g, b, 255]))
            }
        }
    }
}

// Compare our proposed image to the target image
pub fn compare_imaginary(
    target_img: &DynamicImage,
    canvas: &DynamicImage,
    shape: &ShapeInfo,
) -> u32 {
    let ShapeInfo(_s_x, _s_y, _s_w, _s_h, s_rgba) = shape;
    let mut diff_with_shape: u32 = 0;
    let mut diff_with_canvas: u32 = 0;
    for y in shape.1..(shape.1 + shape.3) {
        for x in shape.0..(shape.0 + shape.2) {
            let pixl = &target_img.get_pixel(x, y).0;
            diff_with_shape += rgb_difference(pixl, s_rgba);
            diff_with_canvas += rgb_difference(pixl, &canvas.get_pixel(x, y).0);
        }
    }
    if diff_with_shape < diff_with_canvas {
        diff_with_canvas - diff_with_shape
    } else {
        0
    }
}

// Calculate the absolute differences in rgb values
pub fn rgb_difference(img1: &[u8; 4], img2: &[u8; 4]) -> u32 {
    img1[0].abs_diff(img2[0]) as u32
        + img1[1].abs_diff(img2[1]) as u32
        + img1[2].abs_diff(img2[2]) as u32
        + img1[3].abs_diff(img2[3]) as u32
}

// Calculate the absolute differences in rgb values between two images
pub fn cost(img1: &DynamicImage, img2: &DynamicImage) -> u32 {
    let mut output: u32 = 0;
    for (x, y, rgba) in img1.pixels() {
        output += rgb_difference(&rgba.0, &img2.get_pixel(x, y).0);
    }
    output
}
