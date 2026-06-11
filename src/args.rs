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

impl argh::FromArgValue for HashAlgorithm {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "sha224" => Ok(Self::Sha224),
            "sha256" => Ok(Self::Sha256),
            "sha384" => Ok(Self::Sha384),
            "sha512" => Ok(Self::Sha512),
            "sha512-224" => Ok(Self::Sha512_224),
            "sha512-256" => Ok(Self::Sha512_256),
            "sha3-224" => Ok(Self::Sha3_224),
            "sha3-256" => Ok(Self::Sha3_256),
            "sha3-384" => Ok(Self::Sha3_384),
            "sha3-512" => Ok(Self::Sha3_512),
            _ => Err(format!(
                "unknown hash algorithm: {value}
expected: sha224, sha256, sha384, sha512, sha512-224, sha512-256,
          sha3-224, sha3-256, sha3-384, sha3-512"
            )),
        }
    }
}
