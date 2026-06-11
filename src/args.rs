use std::path::PathBuf;

use argh::FromArgs;

use crate::dispatch::HashAlgorithm;

fn parse_size(s: &str) -> Result<u64, String> {
    parse_size::parse_size(s).map_err(|e| e.to_string())
}

#[derive(FromArgs)]
/// Calculate file hashes.
pub struct Args {
    /// buffer size, default: 32KiB
    #[argh(option, from_str_fn(parse_size), default = "32*1024")]
    pub buf_size: u64,

    /// hash algorithm
    #[argh(positional)]
    pub algorithm: HashAlgorithm,

    /// files to hash
    #[argh(positional)]
    pub files: Vec<PathBuf>,
}
