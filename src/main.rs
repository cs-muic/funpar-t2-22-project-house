use crate::compare_image::{compare_imaginary, cost, draw, ShapeInfo};
use clap::Parser;
use cli::Args;
use image::{DynamicImage, GenericImageView};
// use image::{GrayImage, Rgba};
use rayon::iter::*;
use show_image::{create_window, ImageInfo, ImageView};
use std::collections::HashSet;
// use crate::edge_detection::edge_detection;
// use std::time;

mod cli;
mod compare_image;
// mod edge_detection;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    let img = image::open(args.img_path).expect("no file found");
    let mut output = DynamicImage::new_rgba8(img.width(), img.height());

    let (width, height) = img.dimensions();
    let colors = img
        .pixels()
        .map(|(_, _, r)| r.0)
        .collect::<HashSet<[u8; 4]>>()
        .iter()
        .copied()
        .collect::<Vec<[u8; 4]>>();

    // edge detection
    // let edge_detection: GrayImage = edge_detection(&img, width, height);
    // let edge_image: DynamicImage = edge_detection.into();
    // let (_w, _h) = edge_image.dimensions();
    // let _p_a = edge_image
    //     .pixels()
    //     .flat_map(|(_, _, r)| r.0)
    //     .collect::<Vec<u8>>();
    //
    // let coordinate_of_picture = &edge_image
    //     .pixels().filter(|(_,_,r)| r.0 != [0,0,0,0]).map(|(x, y, _)| (x,y)).collect::<Vec<(u32, u32)>>();
    // dbg!(coordinate_of_picture);

    let pixel_data = output
        .pixels()
        .flat_map(|(_, _, r)| r.0)
        .collect::<Vec<u8>>();
    let image = ImageView::new(ImageInfo::rgba8(width, height), &pixel_data);
    //this is for the edge detection.
    // let image = ImageView::new(ImageInfo::rgba8(_w, _h), &_p_a);

    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;
    let original_cost = cost(&img, &output);
    let mut current_cost = original_cost;
    // println!("Cost: {}", current_cost);
    /* for timing */
    // use std::time::Instant;
    // let now = Instant::now();

    // If the user closes the window, the channel is closed and the loop also exits.
    'outer: loop {
        let mut update_loop = || {
            let v = (0..300)
                .into_par_iter()
                .map(|_| {
                    let shape =
                        ShapeInfo::make_random(width, height, original_cost, current_cost, &colors);
                    let diff = compare_imaginary(&img, &output, &shape);
                    (diff, shape)
                })
                .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                .unwrap();
            /* for timing */
            // let elapsed = now.elapsed();
            // println!("Elapsed: {:.2?}", elapsed);
            // if elapsed.as_secs() == 10 {
            //     println!("Cost: {}", current_cost);
            // }
            // dbg!(&v);

            if v.0 > 0 {
                draw(&mut output, v.1);
                current_cost -= v.0;
                let pixel_data = output
                    .pixels()
                    .flat_map(|(_, _, r)| r.0)
                    .collect::<Vec<u8>>();
                window
                    .set_image(
                        "image-001",
                        ImageView::new(ImageInfo::rgba8(width, height), &pixel_data),
                    )
                    .expect("window can construct/window dies");
            }
        };
        match window.event_channel() {
            Ok(_) => update_loop(),
            Err(_) => break 'outer,
        }
    }

    Ok(())
}
