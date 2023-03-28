//  cargo run --release "assets/house.png"
//  cargo run --release "assets/nature.jpg"

use crate::compare_image::{best_shapes, compare, compare_imaginary, cost, draw, ShapeInfo};
use crate::place_shape::*;
use clap::Parser;
use cli::Args;
use image::{GenericImage, GenericImageView, Rgba};
use rayon::iter::*;
use show_image::{create_window, event, ImageInfo, ImageView};
use std::collections::HashSet;
use std::f64::MAX;
use std::time;

mod cli;
mod compare_image;
mod place_shape;

// pre-commit run --all

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    let mut img = image::open(args.img_path).expect("no file found");
    let mut output = image::DynamicImage::new_rgba8(img.width(), img.height());

    let (width, height) = img.dimensions();
    /* [r.0[0], r.0[1], r.0[2]] */

    let colors = img
        .pixels()
        .map(|(_, _, r)| r.0)
        .collect::<HashSet<[u8; 4]>>()
        .iter()
        .copied()
        .collect::<Vec<[u8; 4]>>();

    let pixel_data = output
        .pixels()
        .flat_map(|(_, _, r)| r.0)
        .collect::<Vec<u8>>();

    let mut image = ImageView::new(ImageInfo::rgba8(width, height), &pixel_data);

    // // Create a window with default options and display the image.

    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;

    // let mut current_cost: f32 = f32::MAX;
    // let original_cost: u64 = 10000000000;
    // let original_cost = compare(&img, &output);
    // let original_cost = (img.height() * img.width()) as f32;
    let original_cost = cost(&img, &output);
    let mut current_cost = original_cost;

    // If the user closes the window, the channel is closed and the loop also exits.
    'outer: loop {
        let mut rng = rand::thread_rng();
        let mut update_loop = || {
            // println!("Cost: {}", current_cost);
            // use std::time::Instant;
            // let now = Instant::now();
            let v = (0..300)
                .into_par_iter()
                .map(|_| {
                    let shape =
                        ShapeInfo::make_random(width, height, original_cost, current_cost, &colors);
                    let diff = compare_imaginary(&img, &output, &shape);

                    // output

                    // img
                    (diff, shape)
                })
                // .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                .unwrap();

            // let elapsed = now.elapsed();
            // println!("Elapsed: {:.2?}", elapsed);

            // println!("Cost: {}", current_cost);
            // dbg!(&v);

            if v.0 > 0 {
                draw(&mut output, v.1);
                current_cost -= v.0;
                // println!("Change cost: {}", v.0);
                // current_cost = current_cost - v.0;

                let pixel_data = output
                    .pixels()
                    .flat_map(|(_, _, r)| r.0)
                    .collect::<Vec<u8>>();
                window.set_image(
                    "image-001",
                    ImageView::new(ImageInfo::rgba8(width, height), &pixel_data),
                ); // The issue is that my squares are smaller for some reason :/
            }
        };
        match window.event_channel() {
            Ok(_) => update_loop(),
            Err(_) => break 'outer,
        }
    }

    Ok(())
    //
    //
    //
    // for event in window.event_channel()? {
    //     let coordinates = vec![
    //                                     (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1), (9, 1), (10, 1), (11, 1), (12, 1), (13, 1), (14, 1), (15, 1), (16, 1), (17, 1), (18, 1), (19, 1)];
    //     let best_shape: (Vec<(u32, u32)>, [u8; 4], f32) = coordinates
    //         .into_par_iter()
    //         .map(|coord| compare_image::compare(&img, coord))
    //         .max_by_key(|input| input.2 as u16)
    //         .unwrap();
    //     // best_shape.0
    //     //     .into_iter()
    //     //     .for_each(|a| img.put_pixel(a.0, a.1, Rgba(best_shape.1)));
    //     let mut new_image = img.clone();
    //     for a in best_shape.0 {
    //         new_image.put_pixel(a.0, a.1, Rgba(best_shape.1));
    //     }
    //     let pixel_data = new_image.pixels().flat_map(|(_, _, r)| r.0).collect::<Vec<u8>>();
    //     window.set_image("image-001", ImageView::new(ImageInfo::rgba8(width, height), &pixel_data))?;
    //     if let event::WindowEvent::KeyboardInput(event) = event {
    //         println!("{:#?}", event);
    //         if event.input.key_code == Some(event::VirtualKeyCode::Escape)
    //             && event.input.state.is_pressed()
    //         {
    //             break;
    //         }
    //     }
    //     println!("test");
    // }
    // // create_image::mse(&img, &img);
    // // make_square(img);
    //
    // Ok(())
}
