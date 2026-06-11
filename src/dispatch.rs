use std::fs::File;
use std::io::{BorrowedBuf, Read as _};

use digest::Digest;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512_224,
    Sha512_256,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

fn hash_file<D: Digest>(file: &mut File, buffer_size: usize) -> std::io::Result<Box<[u8]>> {
    let mut ctx = D::new();
    let mut buf = Vec::with_capacity(buffer_size);
    let mut cursor = BorrowedBuf::from(buf.spare_capacity_mut());

    loop {
        match file.read_buf(cursor.unfilled()) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }

        if cursor.len() == 0 {
            // We've reached EOF
            return Ok(ctx.finalize().to_vec().into_boxed_slice());
        }

        ctx.update(cursor.filled());
        cursor.clear();
    }
}

impl HashAlgorithm {
    /// Hashes the file and returns the original `File` back (reset to the beginning).
    ///
    /// The file would be read to end. Seek if you want to reuse it later.
    pub fn hash_file(self, file: &mut File, buffer_size: usize) -> std::io::Result<Box<[u8]>> {
        // Compute hash
        let hash = match self {
            HashAlgorithm::Sha224 => hash_file::<Sha224>,
            HashAlgorithm::Sha256 => hash_file::<Sha256>,
            HashAlgorithm::Sha384 => hash_file::<Sha384>,
            HashAlgorithm::Sha512 => hash_file::<Sha512>,
            HashAlgorithm::Sha512_224 => hash_file::<Sha512_224>,
            HashAlgorithm::Sha512_256 => hash_file::<Sha512_256>,
            HashAlgorithm::Sha3_224 => hash_file::<Sha3_224>,
            HashAlgorithm::Sha3_256 => hash_file::<Sha3_256>,
            HashAlgorithm::Sha3_384 => hash_file::<Sha3_384>,
            HashAlgorithm::Sha3_512 => hash_file::<Sha3_512>,
        };

        hash(file, buffer_size)
    }
}
