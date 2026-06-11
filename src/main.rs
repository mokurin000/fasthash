#![feature(core_io_borrowed_buf)]
#![feature(read_buf)]

use std::borrow::Cow;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::{BorrowedBuf, Read};
use std::os::windows::fs::OpenOptionsExt as _;
use std::path::PathBuf;

use argh::FromArgs;
use hex_simd::AsciiCase;
use sha2::digest::DynDigest;
use sha2::{Digest as _, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

#[derive(Debug, Clone, Copy)]
enum HashAlgorithm {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512_224,
    Sha512_256,
}

impl HashAlgorithm {
    pub fn as_algo(self) -> Box<dyn DynDigest> {
        let algo: &dyn DynDigest = match self {
            Self::Sha224 => &Sha224::new(),
            Self::Sha256 => &Sha256::new(),
            Self::Sha384 => &Sha384::new(),
            Self::Sha512 => &Sha512::new(),
            Self::Sha512_224 => &Sha512_224::new(),
            Self::Sha512_256 => &Sha512_256::new(),
        };
        algo.box_clone()
    }
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
            _ => Err(format!(
                "unknown hash algorithm: {value}
expected: sha224, sha256, sha384, sha512, sha512-224, sha512-256"
            )),
        }
    }
}

fn parse_size(s: &str) -> Result<u64, String> {
    parse_size::parse_size(s).map_err(|e| e.to_string())
}

#[derive(FromArgs)]
/// Calculate file hashes.
struct Args {
    /// buffer size, default: 32KiB
    #[argh(option, from_str_fn(parse_size), default = "32*1024")]
    buf_size: u64,

    /// hash algorithm
    #[argh(positional)]
    algorithm: HashAlgorithm,

    /// files to hash
    #[argh(positional)]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = argh::from_env();

    let alg = args.algorithm.as_algo();

    if args.files.is_empty() {
        if let Err(e) = Args::from_args(&["fasthash"], &["--help"]) {
            eprintln!("{}", e.output);
            std::process::exit(1);
        }
    }

    for path in args.files {
        let Ok(mut file) = OpenOptions::new()
            .read(true)
            .custom_flags(if cfg!(windows) {
                // magic: Sequential Scan, potentianlly increasing seq read performance
                1 << 27
            } else {
                0
            })
            .open(&path)
            .inspect_err(|e| {
                eprintln!("Failed to open {path:?}: {e}");
            })
        else {
            continue;
        };

        let mut ctx = alg.clone();
        let mut buffer = Vec::with_capacity(args.buf_size as usize);

        let mut cursor = BorrowedBuf::from(buffer.spare_capacity_mut());

        let file_read_result = loop {
            if let Err(e) = file.read_buf(cursor.unfilled()) {
                break Err(e);
            }
            if cursor.len() == 0 {
                break Ok(());
            }

            ctx.update(cursor.filled());
            cursor.clear();
        };

        if let Err(e) = file_read_result {
            eprintln!("Failed reading {path:?}: {e}");
            break;
        }

        let hex = hex_simd::encode_to_string(ctx.finalize(), AsciiCase::Lower);

        println!(
            "{hex} {}",
            path.file_name()
                .map(OsStr::to_string_lossy)
                .unwrap_or(Cow::Borrowed("-"))
        );
    }

    Ok(())
}
