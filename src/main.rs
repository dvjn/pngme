mod cli;
mod util;

use std::str::FromStr;

use anyhow::Context;
use clap::Parser;
use cli::{Cli, Decode, Encode, Print, Remove};
use fehler::throws;
use pngme_lib::{chunk::Chunk, chunk_type::ChunkType};
use util::{parse_png_from_file, save_png_to_file};

#[throws(anyhow::Error)]
fn main() {
    match Cli::parse() {
        Cli::Encode(args) => encode(args),
        Cli::Decode(args) => decode(args),
        Cli::Remove(args) => remove(args),
        Cli::Print(args) => print(args),
    }?
}

#[throws(anyhow::Error)]
fn encode(args: Encode) {
    let mut png = parse_png_from_file(&args.png_path)?;

    let chunk_type = ChunkType::from_str(&args.chunk_type).context("invalid chunk type")?;
    let data = args.message.into_bytes();
    let chunk = Chunk::new(chunk_type, data);

    png.append_chunk(chunk);

    let output_path = if let Some(path) = args.output_png_path {
        path
    } else {
        args.png_path
    };

    save_png_to_file(png, &output_path)?;
}

#[throws(anyhow::Error)]
fn decode(args: Decode) {
    let png = parse_png_from_file(&args.png_path)?;

    let chunk = png
        .chunk_by_type(&args.chunk_type)
        .context("chunk not found")?;

    println!("Found chunk: \"{}\"", chunk.data_as_string());
}

#[throws(anyhow::Error)]
fn remove(args: Remove) {
    let mut png = parse_png_from_file(&args.png_path)?;

    let chunk = png
        .remove_chunk(&args.chunk_type)
        .context("chunk not found")?;

    println!("Removed chunk with message: \"{}\"", chunk.data_as_string());

    save_png_to_file(png, &args.png_path)?;
}

#[throws(anyhow::Error)]
fn print(args: Print) {
    let png = parse_png_from_file(&args.png_path)?;

    for chunk in png.chunks() {
        println!(
            "Chunk \"{}\": \"{}\"",
            chunk.chunk_type(),
            chunk.data_as_string()
        )
    }
}
