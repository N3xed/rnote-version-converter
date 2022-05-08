#![allow(dead_code)]

use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::Parser;

mod rnotev0_4;
mod rnotev0_5;
mod slot;

extern crate nalgebra as na;
extern crate parry2d_f64 as p2d;

#[derive(Parser)]
struct Args {
    /// The file to convert.
    file: PathBuf,
    /// The destination file or `<file>-upgraded.rnote` per default.
    dest_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let dest_file = args.dest_file.unwrap_or_else(|| {
        let mut src_filename = args
            .file
            .file_stem()
            .expect("<file> must be a file")
            .to_os_string();
        src_filename.push("-converted");
        let mut dest_file = args.file.with_file_name(src_filename);
        if let Some(file_ext) = args.file.extension() {
            dest_file.set_extension(file_ext);
        }
        dest_file
    });

    let bytes = fs::read(&args.file)
        .with_context(|| anyhow!("failed to read file '{}'", args.file.display()))?;
    let bytes = String::from_utf8(decompress_from_gzip(&bytes)?)?;

    let rnotev4_sheet = serde_json::from_str::<rnotev0_4::Sheet>(bytes.as_str())?;
    let rnotev5_sheet: rnotev0_5::RnotefileWrapper = rnotev4_sheet.into();

    let compressed = compress_to_gzip(
        serde_json::to_string(&rnotev5_sheet)?.as_bytes(),
        &dest_file.file_name().unwrap().to_string_lossy(),
    )?;
    
    std::fs::write(dest_file, compressed)?;

    Ok(())
}

/// Decompress from gzip
fn decompress_from_gzip(compressed: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let mut decoder = flate2::read::MultiGzDecoder::new(compressed);
    let mut bytes: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut bytes)?;

    Ok(bytes)
}

/// Compress bytes with gzip
fn compress_to_gzip(to_compress: &[u8], file_name: &str) -> Result<Vec<u8>, anyhow::Error> {
    let compressed_bytes = Vec::<u8>::new();

    let mut encoder = flate2::GzBuilder::new()
        .filename(file_name)
        .write(compressed_bytes, flate2::Compression::default());

    encoder.write_all(to_compress)?;

    Ok(encoder.finish()?)
}
