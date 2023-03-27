use ::rayon::iter::*;
use image::*;
use rand::Rng;
use show_image::event::VirtualKeyCode::R;

pub fn make_square(input_image: DynamicImage) -> DynamicImage {
    fn random_number() -> u8 {
        let mut rng = rand::thread_rng();
        let output: u8 = rng.gen_range(0..=255);
        output
    }

    let mut input_image = input_image;

    let rgba_value = Rgba([
        random_number(),
        random_number(),
        random_number(),
        random_number(),
    ]);
    let mut rng = rand::thread_rng();
    let max_radius = std::cmp::min(input_image.height(), input_image.width());
    let radius = rng.gen_range(0..max_radius);
    let top_left_corner = (
        rng.gen_range(0..(input_image.width() - 1)),
        rng.gen_range(0..(input_image.height() - 1)),
    );

    // Draw the square on the image
    for y in 0..radius {
        for x in 0..radius {
            if (top_left_corner.0 + x < input_image.width() - 1)
                && (top_left_corner.1 + y < input_image.height() - 1)
            {
                // TODO: Fix this
                input_image.put_pixel(top_left_corner.0 + x, top_left_corner.1 + y, rgba_value);
            }
        }
    }

    // Save the image to a file
    println!("Saving!");
    input_image.save("tests/circle.png").unwrap();

    input_image
}

pub fn change_pixels(chunk_of_image: Rgba<u8>) -> Rgba<u8> {
    fn random_number() -> u8 {
        let mut rng = rand::thread_rng();
        let output: u8 = rng.gen_range(0..=255);
        output
    }
    let output = Rgba([
        random_number(),
        random_number(),
        random_number(),
        random_number(),
    ]);
    return output;
}
