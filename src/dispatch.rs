use std::fs::File;
use std::io::{BorrowedBuf, Read as _};
use std::sync::Arc;

use crossfire::flavor::Queue;
use ring::digest::{Algorithm, Context, SHA256, SHA384, SHA512, SHA512_256};

#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    Sha256,
    Sha384,
    Sha512,
    Sha512_256,
}

fn hash_file(
    algorithm: &'static Algorithm,
    mut file: File,
    buffer_size: usize,
    queue_len: usize,
) -> std::io::Result<Box<[u8]>> {
    let mut ctx = Context::new(algorithm);

    let queue = Arc::new(crossfire::spsc::Array::new(queue_len));
    let (readed_tx, readed_rx) = crossfire::spsc::bounded_blocking(queue_len);

    for _ in 0..queue_len {
        _ = queue.push(Vec::with_capacity(buffer_size));
    }

    let queue_ = queue.clone();
    let io_result = std::thread::spawn(move || {
        loop {
            let Some(mut buffer) = queue_.pop() else {
                std::thread::yield_now();
                continue;
            };

            let mut cursor = BorrowedBuf::from(buffer.spare_capacity_mut());

            file.read_buf(cursor.unfilled())?;

            if cursor.len() == 0 {
                break std::io::Result::Ok(());
            } else {
                let new_len = cursor.len();
                unsafe { buffer.set_len(new_len) };
                _ = readed_tx.send(buffer);
            }
        }
    });

    while let Ok(mut buffer) = readed_rx.recv() {
        ctx.update(&buffer);
        buffer.clear();
        _ = queue.push(buffer);
    }

    io_result.join().expect("file io thread panicked")?;

    Ok(ctx.finish().as_ref().to_vec().into_boxed_slice())
}

impl HashAlgorithm {
    /// Hashes the file and returns the original `File` back (reset to the beginning).
    ///
    /// The file would be read to end. Seek if you want to reuse it later.
    pub fn hash_file(
        self,
        file: File,
        buffer_size: usize,
        queue_len: usize,
    ) -> std::io::Result<Box<[u8]>> {
        // Compute hash
        let algorithm = match self {
            HashAlgorithm::Sha256 => &SHA256,
            HashAlgorithm::Sha384 => &SHA384,
            HashAlgorithm::Sha512 => &SHA512,
            HashAlgorithm::Sha512_256 => &SHA512_256,
        };

        hash_file(algorithm, file, buffer_size, queue_len)
    }
}
