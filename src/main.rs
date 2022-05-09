#![allow(dead_code)]

use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::{ArgEnum, Parser};
use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator};

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
    #[clap(short = 'i', long, arg_enum, default_value_t = Version::V0_4)]
    input_type: Version,
    #[clap(short = 'o', long, arg_enum, default_value_t = OutputType::Latest)]
    output_type: OutputType,
}

#[derive(ArgEnum, Copy, Clone, Debug, EnumIter, PartialEq, Eq)]
enum Version {
    V0_4,
    V0_5,
}

#[derive(ArgEnum, Copy, Clone, Debug, EnumIter)]
enum OutputType {
    Json,
    PrettyJson,
    V0_4,
    V0_5,
    Latest,
}

impl OutputType {
    pub fn as_version(self) -> Option<Version> {
        match self {
            Self::V0_4 => Some(Version::V0_4),
            Self::V0_5 | Self::Latest => Some(Version::V0_5),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
enum RnoteDocument {
    V0_4(rnotev0_4::Sheet),
    V0_5(rnotev0_5::RnotefileWrapper),
}

impl RnoteDocument {
    pub fn version(&self) -> Version {
        match self {
            Self::V0_4(_) => Version::V0_4,
            Self::V0_5(_) => Version::V0_5,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>, version: Version) -> Result<RnoteDocument> {
        let bytes = String::from_utf8(decompress_from_gzip(&bytes)?)?;

        match version {
            Version::V0_4 => Ok(RnoteDocument::V0_4(serde_json::from_str(bytes.as_str())?)),
            Version::V0_5 => Ok(RnoteDocument::V0_5(serde_json::from_str(bytes.as_str())?)),
        }
    }

    pub fn into_version(mut self, version: Version) -> RnoteDocument {
        let start_version = self.version();
        let mut iter = Version::iter().skip_while(|&v| v != start_version);
        iter.next();

        for next_version in iter {
            let curr_version = self.version();
            match (self, version) {
                (Self::V0_4(val), Version::V0_5) => {
                    self = Self::V0_5(val.into());
                }
                _ => unimplemented!(
                    "converting from {curr_version:?} to {version:?} is not implemented"
                ),
            }
            if next_version == version {
                break;
            }
        }
        eprintln!(
            "converted from {start_version:?} to {:?} (expected {version:?})",
            self.version()
        );
        self
    }

    pub fn to_json(&self, pretty: bool) -> Result<String> {
        fn serialize<T: Serialize>(val: &T, pretty: bool) -> Result<String> {
            if pretty {
                serde_json::to_string_pretty(val)
            } else {
                serde_json::to_string(val)
            }
            .map_err(Into::into)
        }

        match self {
            Self::V0_4(val) => serialize(val, pretty),
            Self::V0_5(val) => serialize(val, pretty),
        }
    }

    pub fn into_output(self, output_type: OutputType, file_name: &str) -> Result<Vec<u8>> {
        if let Some(version) = output_type.as_version() {
            if version == self.version() {
                compress_to_gzip(self.to_json(false)?.as_bytes(), file_name)
            } else {
                compress_to_gzip(
                    self.into_version(version).to_json(false)?.as_bytes(),
                    file_name,
                )
            }
        } else {
            match output_type {
                OutputType::Json => Ok(self.to_json(false)?.into_bytes()),
                OutputType::PrettyJson => Ok(self.to_json(true)?.into_bytes()),
                _ => unreachable!(),
            }
        }
    }
}

fn main() -> Result<()> {
    let Args {
        file,
        dest_file,
        input_type,
        output_type,
    } = Args::parse();

    let dest_file = dest_file.unwrap_or_else(|| {
        let mut src_filename = file
            .file_stem()
            .expect("<file> must be a file")
            .to_os_string();
        src_filename.push("-converted");
        let mut dest_file = file.with_file_name(src_filename);
        if let Some(file_ext) = file.extension() {
            dest_file.set_extension(file_ext);
        }
        dest_file
    });

    let bytes =
        fs::read(&file).with_context(|| anyhow!("failed to read file '{}'", file.display()))?;

    let output_bytes = RnoteDocument::from_bytes(bytes, input_type)?.into_output(
        output_type,
        &dest_file.file_name().unwrap().to_string_lossy(),
    )?;

    std::fs::write(&dest_file, output_bytes)
        .with_context(|| anyhow!("failed to write '{}'", dest_file.display()))?;
    // let bytes = String::from_utf8(decompress_from_gzip(&bytes)?)?;
    // println!("{bytes}");

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
