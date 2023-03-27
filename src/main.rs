//  cargo run --release "assets/house.png"
//  cargo run --release "assets/nature.jpg"

use crate::compare_image::{
    best_shapes, compare, compare_imaginary, draw, produce_next_best_shape, ShapeInfo,
};
use crate::place_shape::*;
use clap::Parser;
use cli::Args;
use image::{GenericImage, GenericImageView, Rgba};
use rayon::iter::*;
use show_image::{create_window, event, ImageInfo, ImageView};
use std::f64::MAX;

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

    // let rgb_values:  = img.pixels().map(|(_, _, r)| r.0).collect();
    // let rgb_values = rgb_bytes.map(|a| Rgba(a)).collect();

    let pixel_data = output
        .pixels()
        .flat_map(|(_, _, r)| r.0)
        .collect::<Vec<u8>>();

    let mut image = ImageView::new(ImageInfo::rgba8(width, height), &pixel_data);

    // // Create a window with default options and display the image.

    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;

    let mut current_cost: f32 = f32::MAX;

    // If the user closes the window, the channel is closed and the loop also exits.
    'outer: loop {
        let mut update_loop = || {
            // println!("Cost: {}", current_cost);

            let v = (0..1024)
                .into_par_iter()
                .map(|_| {
                    let shape = ShapeInfo::make_random(width, height);
                    let diff = compare_imaginary(&img, &output, &shape);

                    // output

                    // img
                    (diff, shape)
                })
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                .unwrap();

            // println!()

            if v.0 < current_cost {
                draw(&mut output, v.1);
                current_cost = v.0;

                let pixel_data = output
                    .pixels()
                    .flat_map(|(_, _, r)| r.0)
                    .collect::<Vec<u8>>();
                window.set_image(
                    "image-001",
                    ImageView::new(ImageInfo::rgba8(width, height), &pixel_data),
                );
            }

            // let coordinates: Vec<(u32, u32)> = vec![
            //     (1, 1),
            //     (2, 2),
            //     (3, 3),
            //     (4, 4),
            //     (5, 5),
            //     (6, 6),
            //     (7, 7),
            //     (8, 8),
            //     (9, 9),
            //     (10, 10),
            //     (11, 11),
            //     (12, 12),
            //     (13, 13),
            //     (14, 14),
            //     (15, 15),
            //     (16, 16),
            //     (17, 17),
            //     (18, 18),
            //     (19, 19),
            //     (20, 20),
            // ];
            // let every_coord = &img
            //     .pixels()
            //     .map(|(a, b, _)| (a, b))
            //     .collect::<Vec<(u32, u32)>>();
            // // let many_coords = &every_coord.split_at(80000).0.to_vec();
            // let best_shapes = produce_next_best_shape(every_coord, &img, &rgb_values, &output);
            // let mut new_image = output.clone();
            // best_shapes.iter().for_each(|shape| {
            //     if shape.is_some() {
            //         let unwrapped_shape = shape.as_ref().unwrap();
            //         let rgb = Rgba(unwrapped_shape.1);
            //         for a in &unwrapped_shape.0 {
            //             new_image.put_pixel(a.0, a.1, rgb);
            //         }
            //     }
            // });
            // let pixel_data = new_image
            //     .pixels()
            //     .flat_map(|(_, _, r)| r.0)
            //     .collect::<Vec<u8>>();
            // window.set_image(
            //     "image-001",
            //     ImageView::new(ImageInfo::rgba8(width, height), &pixel_data),
            // );
            // new_image
            // New canvas compared to output has a smaller cost than old canvas change the canvas to new canvas
            // ====
            // if compare(&new_image, &img) < compare(&output, &img){
            //     let pixel_data = new_image.pixels().flat_map(|(_, _, r)| r.0).collect::<Vec<u8>>();
            //     window.set_image("image-001", ImageView::new(ImageInfo::rgba8(width, height), &pixel_data));
            //     new_image
            // }
            // else {
            //     // println!("Done");
            //     output
            // }
            // ====
            // let pixel_data = new_image.pixels().flat_map(|(_, _, r)| r.0).collect::<Vec<u8>>();
            // window.set_image("image-001", ImageView::new(ImageInfo::rgba8(width, height), &pixel_data));
            // new_image
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
