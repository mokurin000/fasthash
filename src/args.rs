use std::path::PathBuf;

use argh::FromArgs;

use crate::dispatch::HashAlgorithm;

fn parse_size(s: &str) -> Result<usize, String> {
    parse_size::parse_size(s)
        .map(|n| n as usize)
        .map_err(|e| e.to_string())
}

#[derive(FromArgs)]
/// Calculate file hashes.
pub struct Args {
    /// buffer size, default:
    #[cfg_attr(not(target_os = "macos"), doc = " 256 KiB")]
    #[cfg_attr(target_os = "macos", doc = " 1 MiB")]
    #[argh(
        option,
        from_str_fn(parse_size),
        default = "if cfg!(target_os = \"macos\") { 1024*1024 } else { 256*1024 }"
    )]
    pub buf_size: usize,

    /// buffer queue length, fasthash will allocate such number of buffers
    #[argh(option, default = "8")]
    pub queue_len: usize,

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
