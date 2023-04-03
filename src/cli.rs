use clap_derive::Parser;

#[derive(Parser, Default, Debug)]
pub struct Args {
    pub img_path: String,
}
