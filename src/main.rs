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
use ring::digest::{self, Context};

#[derive(Debug, Clone, Copy)]
enum HashAlgorithm {
    Sha1,
    Sha256,
    Sha384,
    Sha512,
    Sha512_256,
}

impl HashAlgorithm {
    pub const fn as_ring(self) -> &'static digest::Algorithm {
        match self {
            Self::Sha1 => &digest::SHA1_FOR_LEGACY_USE_ONLY,
            Self::Sha256 => &digest::SHA256,
            Self::Sha384 => &digest::SHA384,
            Self::Sha512 => &digest::SHA512,
            Self::Sha512_256 => &digest::SHA512_256,
        }
    }
}

impl argh::FromArgValue for HashAlgorithm {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "sha1" => Ok(Self::Sha1),
            "sha256" => Ok(Self::Sha256),
            "sha384" => Ok(Self::Sha384),
            "sha512" => Ok(Self::Sha512),
            "sha512-256" => Ok(Self::Sha512_256),
            _ => Err(format!(
                "unknown hash algorithm: {value} \
(expected: sha1, sha256, sha384, sha512, sha512-256)"
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

    let alg = args.algorithm.as_ring();

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

        let mut ctx = Context::new(alg);
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

        let hex = hex_simd::encode_to_string(ctx.finish(), AsciiCase::Lower);

        println!(
            "{hex} {}",
            path.file_name()
                .map(OsStr::to_string_lossy)
                .unwrap_or(Cow::Borrowed("-"))
        );
    }

    Ok(())
}
