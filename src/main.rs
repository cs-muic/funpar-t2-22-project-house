use crate::place_shape::make_square;
use clap::Parser;
use cli::Args;
use image::GenericImageView;
use show_image::{create_window, event, ImageInfo, ImageView};

mod cli;
mod compare_image;
mod place_shape;

// pre-commit run --all

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    let img = image::open(args.img_path).expect("no file found");
    let img1 = image::open("assets/img_1.png").expect("no file found");

    // Get the dimensions of the image
    let (width, height) = img.dimensions();
    /* [r.0[0], r.0[1], r.0[2]] */
    // )
    let pixel_data = img.pixels().flat_map(|(_, _, r)| r.0).collect::<Vec<u8>>();

    // Get a flattened array of u8 values representing the pixel data
    // let mut pixel_data: Vec<u8> = Vec::new();
    // for y in 0..height {
    //     for x in 0..width {
    //         let pixel = img.get_pixel(x, y);
    //         pixel_data.push(pixel[0]);
    //         pixel_data.push(pixel[1]);
    //         pixel_data.push(pixel[2]);
    //     }
    //
    // }

    let image = ImageView::new(ImageInfo::rgba8(width, height), &pixel_data);

    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;

    // Print keyboard events until Escape is pressed, then exit.
    // If the user closes the window, the channel is closed and the loop also exits.

    let compare_data = compare_image::compare(&img, &img1);
    dbg!(compare_data);

    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }
    // create_image::mse(&img, &img);
    make_square(img);

    Ok(())
}
