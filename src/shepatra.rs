// use blake2::{Blake2b512, Digests};
// use base64ct::{Base64, Encoding};

// #[derive(Debug)]
// pub enum HashFuncNames {
//     BLAKE2b,
// }

// #[derive(Debug)]
// pub struct Recipe {
//     layers: Vec<HashFuncNames>,
// }

// pub fn hash_with_recipe(str_to_be_hashed: &mut String, recipe: &Recipe) {
//     for hashfunc_name in recipe.layers.iter() {
//         let mut hasher = match hashfunc_name {
//             HashFuncNames::BLAKE2b => blake2::Blake2b512::default(),
//             // HashFuncNames::SHA3_256 => Box::new(sha3::Sha3_256::default()),
//             // HashFuncNames::SHA3_512 => Box::new(sha3::Sha3_512::default()),
//             // HashFuncNames::SHAKE_128 => Box::new(sha3::Shake128::default()),
//             // HashFuncNames::SHAKE_256 => Box::new(sha3::Shake256::default()),
//             _ => {
//                 unimplemented!("The given recipe contains unknown hash func")
//             }
//         };
//         hasher.update(&str_to_be_hashed);
//         let base64_hash = Base64::encode_string(&hasher.finalize());
//         // str_to_be_hashed = ;
//     }
// }
