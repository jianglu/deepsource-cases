// Insufficient RSA key size RS-S1005
// Security
// The strength of public-key-based cryptographic algorithm (like RSA) is determined by the time that it takes to derive the private key by using brute-force methods. 1024-bit keys are to be avoided since they are easy to brute-force. However, 2048-bit keys are said to be sufficient until 2030. Preferably use 4096-bit keys.

// Consider increasing the key size to atleast 2048 bits.

// BAD PRACTICE
// use rsa::RsaPrivateKey;

// let mut rng = rand::thread_rng();
// let priv_key = RsaPrivateKey::new(&mut rng, 1024);
// RECOMMENDED
// use rsa::RsaPrivateKey;

// let mut rng = rand::thread_rng();
// let priv_key = RsaPrivateKey::new(&mut rng, 4096);
// REFERENCES
// CWE-310: Cryptographic Issues

fn main() {
    use rsa::RsaPrivateKey;

    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, 1024);
}
