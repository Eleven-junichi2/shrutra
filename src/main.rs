use sha2::{Digest, Sha256};

mod shepatra;

fn main() {
    let mut hasher = Sha256::new();
    let data = b"Hello world!";
    hasher.update(data);
    // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    hasher.update("String data");
    // Note that calling `finalize()` consumes hasher
    let hash = hasher.finalize();
    let hash_value = format!("{:x}", hash);
    println!("{}", hash_value);
}
