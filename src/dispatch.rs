use digest::Digest;
use enum_dispatch::enum_dispatch;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

use crate::args::HashAlgorithm;

#[enum_dispatch(HashAlgo)]
pub trait HashAlgorithmTrait {
    fn update(&mut self, data: &[u8]);
    fn finalize_boxed(self) -> Box<[u8]>;
}

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum HashAlgo {
    Sha224(Sha224),
    Sha256(Sha256),
    Sha384(Sha384),
    Sha512(Sha512),
    Sha512_224(Sha512_224),
    Sha512_256(Sha512_256),
    Sha3_224(Sha3_224),
    Sha3_256(Sha3_256),
    Sha3_384(Sha3_384),
    Sha3_512(Sha3_512),
}

impl From<HashAlgorithm> for HashAlgo {
    fn from(value: HashAlgorithm) -> Self {
        match value {
            HashAlgorithm::Sha224 => HashAlgo::Sha224(Sha224::new()),
            HashAlgorithm::Sha256 => HashAlgo::Sha256(Sha256::new()),
            HashAlgorithm::Sha384 => HashAlgo::Sha384(Sha384::new()),
            HashAlgorithm::Sha512 => HashAlgo::Sha512(Sha512::new()),
            HashAlgorithm::Sha512_224 => HashAlgo::Sha512_224(Sha512_224::new()),
            HashAlgorithm::Sha512_256 => HashAlgo::Sha512_256(Sha512_256::new()),
            HashAlgorithm::Sha3_224 => HashAlgo::Sha3_224(Sha3_224::new()),
            HashAlgorithm::Sha3_256 => HashAlgo::Sha3_256(Sha3_256::new()),
            HashAlgorithm::Sha3_384 => HashAlgo::Sha3_384(Sha3_384::new()),
            HashAlgorithm::Sha3_512 => HashAlgo::Sha3_512(Sha3_512::new()),
        }
    }
}

macro_rules! impl_hash_algo {
    ($($name:ident => $ty:ty),* $(,)?) => {
        $(
            impl HashAlgorithmTrait for $ty {
                fn update(&mut self, data: &[u8]) {
                    Digest::update(self, data);
                }

                fn finalize_boxed(self) -> Box<[u8]> {
                    Digest::finalize(self).to_vec().into_boxed_slice()
                }
            }
        )*
    };
}

impl_hash_algo! {
    Sha224 => Sha224,
    Sha256 => Sha256,
    Sha384 => Sha384,
    Sha512 => Sha512,
    Sha512_224 => Sha512_224,
    Sha512_256 => Sha512_256,
    Sha3_224 => Sha3_224,
    Sha3_256 => Sha3_256,
    Sha3_384 => Sha3_384,
    Sha3_512 => Sha3_512,
}
