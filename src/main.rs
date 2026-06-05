use std::borrow::Cow;
use std::error::Error;
use std::ffi::OsStr;
use std::path::PathBuf;

use argh::FromArgs;
use compio::BufResult;
use compio::fs::File;
use compio::io::AsyncReadAtExt;
use hex_simd::AsciiCase;
use ring::digest;

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

#[derive(FromArgs)]
/// Calculate file hashes.
struct Args {
    /// hash algorithm
    #[argh(positional)]
    algorithm: HashAlgorithm,

    /// files to hash
    #[argh(positional)]
    files: Vec<PathBuf>,
}

#[compio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = argh::from_env();

    let alg = args.algorithm.as_ring();

    if args.files.is_empty() {
        if let Err(e) = Args::from_args(&["fasthash"], &["--help"]) {
            eprintln!("{}", e.output);
            std::process::exit(1);
        }
    }

    for path in args.files {
        let Ok(file) = File::open(&path).await.inspect_err(|e| {
            eprintln!("Failed to open {path:?}: {e}");
        }) else {
            continue;
        };

        let BufResult(result, data) = file
            .read_to_end_at(Vec::with_capacity(file.metadata().await?.len() as usize), 0)
            .await;

        if let Err(e) = result {
            eprintln!("Failed to read {path:?}: {e}");
            continue;
        }

        let digest = ring::digest::digest(alg, &data);
        let hex = hex_simd::encode_to_string(&digest, AsciiCase::Lower);

        println!(
            "{hex} {}",
            path.file_name()
                .map(OsStr::to_string_lossy)
                .unwrap_or(Cow::Borrowed("-"))
        );
    }

    Ok(())
}
