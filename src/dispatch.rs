use digest::{Digest as _, DynDigest};
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

impl From<HashAlgorithm> for Box<dyn DynDigest> {
    fn from(value: HashAlgorithm) -> Box<dyn DynDigest> {
        let algo: &dyn DynDigest = match value {
            HashAlgorithm::Sha224 => &Sha224::new(),
            HashAlgorithm::Sha256 => &Sha256::new(),
            HashAlgorithm::Sha384 => &Sha384::new(),
            HashAlgorithm::Sha512 => &Sha512::new(),
            HashAlgorithm::Sha512_224 => &Sha512_224::new(),
            HashAlgorithm::Sha512_256 => &Sha512_256::new(),
            HashAlgorithm::Sha3_224 => &Sha3_224::new(),
            HashAlgorithm::Sha3_256 => &Sha3_256::new(),
            HashAlgorithm::Sha3_384 => &Sha3_384::new(),
            HashAlgorithm::Sha3_512 => &Sha3_512::new(),
        };
        algo.box_clone()
    }
}
