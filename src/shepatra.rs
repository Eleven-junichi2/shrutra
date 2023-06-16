// use blake2::{Blake2b512, Digests};
// use base64ct::{Base64, Encoding};
use digest::DynDigest;
use sha2::{Sha256, Sha512};

#[derive(Debug)]
pub enum HashFuncNames {
    SHA256,
    SHA512,
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
