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

impl HashAlgorithm {
    pub fn as_algo(self) -> Box<dyn DynDigest> {
        let algo: &dyn DynDigest = match self {
            Self::Sha224 => &Sha224::new(),
            Self::Sha256 => &Sha256::new(),
            Self::Sha384 => &Sha384::new(),
            Self::Sha512 => &Sha512::new(),
            Self::Sha512_224 => &Sha512_224::new(),
            Self::Sha512_256 => &Sha512_256::new(),
            Self::Sha3_224 => &Sha3_224::new(),
            Self::Sha3_256 => &Sha3_256::new(),
            Self::Sha3_384 => &Sha3_384::new(),
            Self::Sha3_512 => &Sha3_512::new(),
        };
        algo.box_clone()
    }
}

impl argh::FromArgValue for HashAlgorithm {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "sha224" => Ok(Self::Sha224),
            "sha256" => Ok(Self::Sha256),
            "sha384" => Ok(Self::Sha384),
            "sha512" => Ok(Self::Sha512),
            "sha512-224" => Ok(Self::Sha512_224),
            "sha512-256" => Ok(Self::Sha512_256),
            "sha3-224" => Ok(Self::Sha3_224),
            "sha3-256" => Ok(Self::Sha3_256),
            "sha3-384" => Ok(Self::Sha3_384),
            "sha3-512" => Ok(Self::Sha3_512),
            _ => Err(format!(
                "unknown hash algorithm: {value}
expected: sha224, sha256, sha384, sha512, sha512-224, sha512-256, sha3-224, sha3-256, sha3-384, sha3-512"
            )),
        }
    }
}
