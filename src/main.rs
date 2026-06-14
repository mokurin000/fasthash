#![feature(core_io_borrowed_buf)]
#![feature(read_buf)]

use std::error::Error;
use std::fs::OpenOptions;

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt as _;

#[cfg(unix)]
use std::os::fd::AsFd;

use argh::FromArgs;
use hex_simd::AsciiCase;

use crate::args::Args;

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

    let buffer_size = args.buf_size as _;
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

        #[cfg(all(unix, not(target_os = "macos")))]
        if let Err(e) =
            rustix::fs::fadvise(AsFd::as_fd(&file), 0, None, rustix::fs::Advice::Sequential)
        {
            eprintln!(
                "Failed to advise sequential read optimizations, may result sub-optimal performance!\n{e}"
            );
        };

        let file_read_result = args.algorithm.hash_file(&mut file, buffer_size);
        match file_read_result {
            Ok(data) => {
                let hex = hex_simd::encode_to_string(data, AsciiCase::Lower);
                println!("{hex} *{}", path.as_os_str().to_string_lossy());
            }
            Err(e) => {
                eprintln!("Failed reading {path:?}: {e}");
                continue;
            }
        }
    }

    Ok(())
}
