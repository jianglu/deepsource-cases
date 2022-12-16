// Found usage of cryptographically insecure algorithm RS-S1004
// Security
// Certain cryptographic algorithms are known to be cryptographically insecure and should be avoided.

// Some insecure algorithms from the crypto crate are:

// crypto::rc4: known vulnerabilities, use AES instead
// crypto::sha1: known vulnerabilities, use SHA-256 or SHA3-512 (preferred)
// crypto::md5: known vulnerabilities, use SHA-256 or SHA3-512 (preferred)
// BAD PRACTICE
// use crypto::{sha1::Sha1, digest::Digest};

// let mut hasher = Sha1::new();
// hasher.input_str("hello world");

// assert_eq!(
//     hasher.result_str(),
//     "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"
// );
// RECOMMENDED
// use crypto::{sha3::Sha3, digest::Digest};

// let mut hasher = Sha3::sha3_256();
// hasher.input_str("hello world");

// assert_eq!(
//     hasher.result_str(),
//     "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
// );
// REFERENCES
// CWE-327: Use of a Broken or Risky Cryptographic Algorithm

fn main() {
    use crypto::{digest::Digest, sha1::Sha1};

    let mut hasher = Sha1::new();
    hasher.input_str("hello world");

    assert_eq!(
        hasher.result_str(),
        "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"
    );
}
