#![feature(core_io_borrowed_buf)]
#![feature(read_buf)]

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BorrowedBuf, Read};

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
        let Ok(mut file) = options.open(&path).inspect_err(|e| {
            eprintln!("Failed to open {path:?}: {e}");
        }) else {
            continue;
        };

        let mut ctx = HashAlgo::from(args.algorithm);
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

        let hex = hex_simd::encode_to_string(ctx.finalize_boxed(), AsciiCase::Lower);

        println!("{hex} *{}", path.as_os_str().to_string_lossy());
    }

    Ok(())
}
