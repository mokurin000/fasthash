#![feature(core_io_borrowed_buf)]
#![feature(read_buf)]

use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BorrowedBuf, Read};

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt as _;
use std::path::Path;

use argh::FromArgs;
use hex_simd::AsciiCase;

use crate::args::Args;

mod args;
mod dispatch;

fn open_file(path: impl AsRef<Path>) -> std::io::Result<File> {
    let mut options = OpenOptions::new();
    let options = options.read(true);

    #[cfg(windows)]
    let options = options.custom_flags(
        // magic: Sequential Scan, potentianlly increasing seq read performance
        1 << 27,
    );
    options.open(&path)
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
        let mut ctx = alg.clone();

        let (tx, rx) = crossfire::spsc::bounded_blocking::<Vec<u8>>(8);

        let buf_size = args.buf_size as usize;

        let path_ = path.clone();
        let file_io = std::thread::spawn(move || {
            let mut file = open_file(&path_)?;
            let mut buffer = Vec::with_capacity(buf_size);
            let mut cursor = BorrowedBuf::from(buffer.spare_capacity_mut());

            loop {
                file.read_buf(cursor.unfilled())?;

                if cursor.len() == 0 {
                    break std::io::Result::Ok(());
                }

                _ = tx.send(cursor.filled().to_vec());
                cursor.clear();
            }
        });

        while let Ok(bytes) = rx.recv() {
            ctx.update(&bytes);
        }

        if let Err(e) = file_io.join().unwrap() {
            eprintln!("Failed reading {path:?}: {e}");
            break;
        }

        let hex = hex_simd::encode_to_string(ctx.finalize(), AsciiCase::Lower);

        println!("{hex} *{}", path.as_os_str().to_string_lossy());
    }

    Ok(())
}
