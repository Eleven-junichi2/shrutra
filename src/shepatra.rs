// use blake2::{Blake2b512, Digests};
// use base64ct::{Base64, Encoding};
use digest::DynDigest;

use blake2::{Blake2b512, Blake2s256};
use sha2::{Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(EnumIter, Debug, Display, EnumString)]
pub enum HashFuncNames {
    #[strum(serialize = "SHA-256")]
    SHA256,
    #[strum(serialize = "SHA-512")]
    SHA512,
    #[strum(serialize = "SHA3-256")]
    SHA3_256,
    #[strum(serialize = "SHA3-512")]
    SHA3_512,
    #[strum(serialize = "Blake2b512")]
    Blake2b512,
    #[strum(serialize = "Blake2s256")]
    Blake2s256,
}

#[derive(Debug)]
pub struct Recipe {
    pub layers: Vec<HashFuncNames>,
}

pub fn hash_with_recipe(str_to_be_hashed: &String, recipe: &Recipe) -> String {
    let mut str_to_be_hashed = str_to_be_hashed.clone();
    for hashfunc_name in recipe.layers.iter() {
        let mut hasher: Box<dyn DynDigest> = match hashfunc_name {
            HashFuncNames::SHA256 => Box::new(Sha256::default()),
            HashFuncNames::SHA512 => Box::new(Sha512::default()),
            HashFuncNames::SHA3_256 => Box::new(Sha3_256::default()),
            HashFuncNames::SHA3_512 => Box::new(Sha3_512::default()),
            HashFuncNames::Blake2s256 => Box::new(Blake2s256::default()),
            HashFuncNames::Blake2b512 => Box::new(Blake2b512::default()),
        };
        hasher.update(&str_to_be_hashed.as_bytes());
        str_to_be_hashed = hasher
            .finalize_reset()
            .iter()
            .map(|x| format!("{:x}", *x))
            .collect::<String>()
    }
    str_to_be_hashed
}

#[test]
fn test_hash_with_recipe() {
    use sha2::Digest;
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, "a");
    assert_eq!(
        format!("{:x}", hasher.finalize()),
        hash_with_recipe(
            &"a".to_string(),
            &Recipe {
                layers: vec![HashFuncNames::SHA256],
            },
        )
    );
}

#[test]
fn test_all_hash_func() {
    use strum::IntoEnumIterator;
    hash_with_recipe(
        &"a".to_string(),
        &Recipe {
            layers: HashFuncNames::iter().collect(),
        },
    );
}
