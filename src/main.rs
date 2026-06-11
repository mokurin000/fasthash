#![feature(core_io_borrowed_buf)]
#![feature(read_buf)]

use std::error::Error;
use std::fs::OpenOptions;

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt as _;

use argh::FromArgs;
use hex_simd::AsciiCase;

use crate::args::Args;
use crate::dispatch::{HashAlgo, HashAlgorithmTrait as _};

mod args;
mod dispatch;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = argh::from_env();

    if args.files.is_empty() {
        if let Err(e) = Args::from_args(&["fasthash"], &["--help"]) {
            eprintln!("{}", e.output);
            std::process::exit(1);
        }
    }

    for path in args.files {
        let mut options = OpenOptions::new();
        let options = options.read(true);

        #[cfg(windows)]
        let options = options.custom_flags(
            // magic: Sequential Scan, potentianlly increasing seq read performance
            1 << 27,
        );
        let Ok(file) = options.open(&path).inspect_err(|e| {
            eprintln!("Failed to open {path:?}: {e}");
        }) else {
            continue;
        };

        let algo = HashAlgo::from(args.algorithm);
        match algo.hash_file(file, args.buf_size as _) {
            Ok(data) => {
                let hex = hex_simd::encode_to_string(data, AsciiCase::Lower);
                println!("{hex} *{}", path.as_os_str().to_string_lossy());
            }
            Err(e) => {
                eprintln!("Failed reading {path:?}: {e}");
                break;
            }
        }
    }

    Ok(())
}
