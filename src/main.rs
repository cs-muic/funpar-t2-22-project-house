use clap::Parser;
use cli::Args;
// use image::EncodableLayout;
// use show_image::{create_window, ImageInfo, ImageView};

mod cli;

fn main() {
    let args: Args = Args::parse();
    let _img = image::open(args.img_path).expect("no file found aa");
}

// pre-commit run --all
