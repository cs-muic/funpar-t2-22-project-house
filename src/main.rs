use clap::Parser;
use cli::Args;

mod cli;

fn main() {
    let args: Args = Args::parse();
    let img = image::open(args.img_path).expect("no file found");

    dbg!(img);
}
