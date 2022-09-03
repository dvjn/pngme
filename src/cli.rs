use clap::{Args, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub enum Cli {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

#[derive(Args, Debug)]
pub struct Encode {
    #[clap(value_parser, value_name = "PNG_PATH")]
    pub png_path: PathBuf,

    #[clap(value_parser, value_name = "CHUNK_TYPE")]
    pub chunk_type: String,

    #[clap(value_parser, value_name = "MESSAGE")]
    pub message: String,

    #[clap(value_parser, value_name = "OUTPUT_PNG_PATH")]
    pub output_png_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct Decode {
    #[clap(value_parser, value_name = "PNG_PATH")]
    pub png_path: PathBuf,

    #[clap(value_parser, value_name = "CHUNK_TYPE")]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct Remove {
    #[clap(value_parser, value_name = "PNG_PATH")]
    pub png_path: PathBuf,

    #[clap(value_parser, value_name = "CHUNK_TYPE")]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct Print {
    #[clap(value_parser, value_name = "PNG_PATH")]
    pub png_path: PathBuf,
}
