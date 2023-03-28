use clap::builder::TypedValueParser;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rand::Rng;
use std::collections::HashSet;
// use lab::Lab;
use rayon::iter::*;

pub fn best_shapes(
    img1: &DynamicImage,
    shape_stats: &(u32, u32),
    colors: &Vec<[u8; 4]>,
    canvas: &DynamicImage,
) -> (Option<(Vec<(u32, u32)>, [u8; 4])>) {
    use deltae::*;
    use lab::Lab;

    let (img_width, img_height) = (img1.width(), img1.height());
    // println!("got image pixels");
    // Create image size
    let mut rng = rand::thread_rng();
    // let shape_size = rng.gen_range(1..((img1.height() + img1.width())/2));
    let shape_size = rng.gen_range(1..200);
    let shape_placement_coords = (shape_stats.0, shape_stats.1);
    // println!("generated image size");
    // Produce the coords of every pixel in the shape
    let mut shape_coords = Vec::new();
    for y in 0..shape_size {
        let mut current_row = Vec::new();
        for x in 0..shape_size {
            current_row.push((shape_placement_coords.0 + x, shape_placement_coords.1 + y));
        }
        shape_coords.push(current_row);
    }
    // println!("produced every pixel");
    // Filter out coords that are out of bounds
    let shape_coords_2 = shape_coords.into_par_iter().flatten();
    let shape_coords_3 = shape_coords_2
        .into_par_iter()
        .filter(|a| (a.0 < img_width && a.1 < img_height))
        .collect::<Vec<(u32, u32)>>();

    #[derive(Clone, Copy)]
    struct MyLab(f32, f32, f32);

    // Types that implement Into<LabValue> also implement the Delta trait
    impl From<MyLab> for LabValue {
        fn from(mylab: MyLab) -> Self {
            LabValue {
                l: mylab.0,
                a: mylab.1,
                b: mylab.2,
            }
        }
    }
    // Implement DeltaEq for your own types
    impl<D: Delta + Copy> DeltaEq<D> for MyLab {}

    let mut rng = rand::thread_rng();
    let current_rgb = colors[rng.gen_range(0..colors.len())];

    let compare_new = shape_coords_3.clone().into_par_iter().map(|a| {
        let pic1 = Lab::from_rgba(
            &img1
                .pixels()
                .find(|b| b.0 == a.0 && b.1 == a.1)
                .unwrap()
                .2
                 .0,
        );
        let pic2 = Lab::from_rgba(&current_rgb);
        let lab1 = LabValue {
            l: pic1.l,
            a: pic1.a,
            b: pic1.b,
        }
        .validate();

        let lab2 = LabValue {
            l: pic2.l,
            a: pic2.a,
            b: pic2.b,
        }
        .validate();
        DeltaE::new(lab1.unwrap(), lab2.unwrap(), DE2000)
        // delta
    });

    let compare_canvas = shape_coords_3.clone().into_par_iter().map(|a| {
        let pic1 = Lab::from_rgba(
            &img1
                .pixels()
                .find(|b| b.0 == a.0 && b.1 == a.1)
                .unwrap()
                .2
                 .0,
        );
        let pic2 = Lab::from_rgba(
            &img1
                .pixels()
                .find(|b| b.0 == a.0 && b.1 == a.1)
                .unwrap()
                .2
                 .0,
        );
        let lab1 = LabValue {
            l: pic1.l,
            a: pic1.a,
            b: pic1.b,
        }
        .validate();

        let lab2 = LabValue {
            l: pic2.l,
            a: pic2.a,
            b: pic2.b,
        }
        .validate();
        DeltaE::new(lab1.unwrap(), lab2.unwrap(), DE2000)
        // delta
    });

    let new_deltaE: f32 = compare_new.into_par_iter().map(|a| *a.value()).sum();
    let canvas_deltaE: f32 = compare_canvas.into_par_iter().map(|a| *a.value()).sum();
    if new_deltaE <= canvas_deltaE {
        return Some((shape_coords_3, current_rgb));
    }
    return None;
    // println!("whattttt{}",ans);
}

#[derive(Debug)]
pub struct ShapeInfo(u32, u32, u32, u32, [u8; 4]);

impl ShapeInfo {
    // TODO: Take in a palette
    pub fn make_random(
        max_x: u32,
        max_y: u32,
        original_cost: u32,
        current_cost: u32,
        options: &Vec<[u8; 4]>,
    ) -> ShapeInfo {
        let mut rng = rand::thread_rng();

        let weighted_max_width = (max_x as f64
            * ((current_cost as f64 * 2.0 as f64 / original_cost as f64) as f64))
            as u32;
        let weighted_max_height = (max_y as f64
            * ((current_cost as f64 * 2.0 as f64 / original_cost as f64) as f64))
            as u32;

        // println!("weightwidth: {}, weightheight: {}", weighted_max_width, weighted_max_height);
        let x = rng
            .gen_range((0 - (weighted_max_width as f32 / 2.0) as i32)..(max_x as i32))
            .max(0) as u32;
        let y = rng
            .gen_range((0 - (weighted_max_height as f32 / 2.0) as i32)..(max_y as i32))
            .max(0) as u32;

        let width = rng.gen_range(1..(weighted_max_width + 3));

        let width = if x + width >= max_x {
            max_x - x
        } else {
            width - 1
        };

        // println!("width: {}", width);
        let height = rng.gen_range(1..(weighted_max_height + 3));

        let height = if y + height >= max_y {
            max_y - y
        } else {
            height - 1
        };

        // println!("height: {}", height);
        // ShapeInfo(x, y, width, height, [r, g, b, 255])
        ShapeInfo(
            x,
            y,
            width,
            height,
            options[rng.gen_range(0..options.len())],
        )
    }
}

pub fn draw(canvas: &mut DynamicImage, shape: ShapeInfo) {
    for y in shape.1..(shape.1 + shape.3) {
        for x in shape.0..(shape.0 + shape.2) {
            // println!("draw");
            if x < canvas.width() && y < canvas.height() && x >= 0 {
                let rgb = shape.4;
                let r = rgb[0];
                let g = rgb[1];
                let b = rgb[2];
                canvas.put_pixel(x, y, Rgba([r, g, b, 255]))
            }
        }
    }
}

pub fn compare_imaginary(
    target_img: &DynamicImage,
    canvas: &DynamicImage,
    shape: &ShapeInfo,
) -> u32 {
    let mut diff: u32 = 0u32;

    let ShapeInfo(s_x, s_y, s_w, s_h, s_rgba) = shape;
    //
    // let shape_rgb_sum = s_rgba[0] + s_rgba[1] + s_rgba[2] + s_rgba[3];
    //
    // // x, y, width, height, rgb
    //
    // let mut diff_using_shape: f32 = 0.0;
    // let mut diff_using_canvas: f32 = 0.0;
    //
    // for y in shape.1..(shape.1 + shape.3) {
    //     for x in shape.0..(shape.0 + shape.2) {
    //         if x < target_img.width() && y < target_img.height() {
    //             // println!("get_pixel");
    //             let pos_rgb = target_img.get_pixel(x, y).0;
    //             let pos_rgb_sum = pos_rgb[0] + pos_rgb[1] + pos_rgb[2] + pos_rgb[3];
    //             let canvas_pos_rgb = canvas.get_pixel(x, y);
    //             let canvas_pos_rgb_sum = canvas_pos_rgb[0] + canvas_pos_rgb[1] + canvas_pos_rgb[2] + canvas_pos_rgb[3];
    //             diff_using_shape += shape_rgb_sum.abs_diff(pos_rgb_sum) as f32;
    //             diff_using_canvas += canvas_pos_rgb_sum.abs_diff(pos_rgb_sum) as f32;
    //         }
    //     }
    // }
    //
    // // If the cost on the canvas was lower than our new image, continue with the canvas
    // if diff_using_canvas < diff_using_shape {
    //     return 0.0;
    // }
    // // Else, send in the difference that we have to reduce the cost by.
    // return diff_using_canvas - diff_using_shape;

    //
    // for (x, y, rgba) in target_img.pixels().into_iter() {
    //     // let pos_rgb = target_img.get_pixel(x, y).0;
    //     if (s_x..&(s_x + s_w)).contains(&&x) && (s_y..&(s_y + s_h)).contains(&&y) {
    //         diff += rgb_difference(&rgba.0, s_rgba) as u64;
    //     } else {
    //         let canvas_pos_rgb = canvas.get_pixel(x, y);
    //         diff += rgb_difference(&rgba.0, &canvas_pos_rgb.0) as u64;
    //     }
    // }

    let mut diff_with_shape: u32 = 0;
    let mut diff_with_canvas: u32 = 0;

    for y in shape.1..(shape.1 + shape.3) {
        for x in shape.0..(shape.0 + shape.2) {
            let pixl = &target_img.get_pixel(x, y).0;
            diff_with_shape += rgb_difference(pixl, s_rgba) as u32;
            diff_with_canvas += rgb_difference(pixl, &canvas.get_pixel(x, y).0) as u32;
        }
    }

    if diff_with_shape < diff_with_canvas {
        diff_with_canvas - diff_with_shape
    } else {
        0
    }
    // diff
}

pub fn rgb_difference(img1: &[u8; 4], img2: &[u8; 4]) -> u32 {
    img1[0].abs_diff(img2[0]) as u32
        + img1[1].abs_diff(img2[1]) as u32
        + img1[2].abs_diff(img2[2]) as u32
        + img1[3].abs_diff(img2[3]) as u32
}

pub fn cost(img1: &DynamicImage, img2: &DynamicImage) -> u32 {
    let mut output: u32 = 0;
    for (x, y, rgba) in img1.pixels() {
        output += rgb_difference(&rgba.0, &img2.get_pixel(x, y).0) as u32;
    }
    output
}

pub fn compare(img1: &DynamicImage, img2: &DynamicImage) -> f32 {
    use deltae::*;
    use lab::Lab;

    // let (width, height) = img1.dimensions();
    // let mut sum = 0.0;
    let imag1 = img1.pixels();
    let imag2 = img2.pixels();
    // let imag1 = Lab::from_rgb(&[253, 120, 138]);
    // let imag2 = Lab::from_rgb()

    #[derive(Clone, Copy)]
    struct MyLab(f32, f32, f32);

    // Types that implement Into<LabValue> also implement the Delta trait
    impl From<MyLab> for LabValue {
        fn from(mylab: MyLab) -> Self {
            LabValue {
                l: mylab.0,
                a: mylab.1,
                b: mylab.2,
            }
        }
    }
    // Implement DeltaEq for your own types
    impl<D: Delta + Copy> DeltaEq<D> for MyLab {}
    let a: _ = imag1.zip(imag2).collect::<Vec<_>>();

    // let diff = a.iter().flat_map(|(a, b)| a.iter().zip(b.iter()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let compare = a.into_par_iter().map(|a| {
        let pic1 = Lab::from_rgba(&a.0 .2 .0);
        let pic2 = Lab::from_rgba(&a.1 .2 .0);
        let lab1 = LabValue {
            l: pic1.l,
            a: pic1.a,
            b: pic1.b,
        }
        .validate();

        let lab2 = LabValue {
            l: pic2.l,
            a: pic2.a,
            b: pic2.b,
        }
        .validate();
        DeltaE::new(lab1.unwrap(), lab2.unwrap(), DE2000)
        // delta
    });

    compare.into_par_iter().map(|a| *a.value()).sum()
    // println!("whattttt{}",ans);
}
