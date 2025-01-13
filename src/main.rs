//
// Rust Project: Rust Mnemonic Generator
// Author:  TheSageMaster
// Date:  2025-01-13
// Description:  A bip-39 mnemonic generator for rust
//

use bitcoin::Network;
use bitcoin::bip32::DerivationPath;
use bitcoin::bip32::Xpriv;
use bitcoin::bip32::Xpub;
use bitcoin::key::PrivateKey;
use bitcoin::secp256k1;
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2;
use rand::RngCore;
use secp256k1::Secp256k1;
use sha2::{Digest, Sha256, Sha512};
use std::fs;
use std::str::FromStr;
use std::time::Instant;
pub type ExtendedPrivKey = Xpriv;
pub type ExtendedPubKey = Xpub;
pub type HmacSha512 = Hmac<Sha512>;

/// Generates 128 bits of random entropy and prints it to the console.
fn generate128bitentropy() -> [u8; 16] {
    let mut entropy = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut entropy);
    println!(
        "Entropy: {}",
        entropy
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    );
    entropy
}

/// Calculates the checksum of a 128-bit entropy as per BIP-39 standard.
///
/// # Parameters
/// - `entropy`: A 128-bit entropy.
///
/// # Returns
/// A tuple containing the checksum as a binary string and the number of bits it occupies.
///
fn calculate_checksum(entropy: [u8; 16]) -> (String, usize) {
    let hash = Sha256::digest(&entropy);
    let checksum_bit_count = entropy.len() * 8 / 32;
    let checksum = (hash[0] >> (8 - checksum_bit_count)) & ((1 << checksum_bit_count) - 1);
    let checksum_binary = format!("{:0width$b}", checksum, width = checksum_bit_count);
    (checksum_binary, checksum_bit_count)
}

/// Creates a bit stream from a 128-bit entropy and a checksum.
///
/// # Parameters
/// - `entropy`: A 128-bit entropy.
/// - `checksum_binary`: A string representing the checksum in binary form.
/// - `checksum_bits`: The number of bits the checksum occupies.
///
/// # Returns
/// A vector of u8 representing the bit stream.
fn bit_stream(entropy: [u8; 16], checksum_binary: String, checksum_bits: usize) -> Vec<u8> {
    let mut bit_stream = Vec::with_capacity((entropy.len() * 8) + checksum_bits);
    for byte in &entropy {
        for i in (0..8).rev() {
            bit_stream.push((byte >> i) & 1);
        }
    }
    for bit in checksum_binary.chars() {
        bit_stream.push(bit.to_digit(10).unwrap() as u8);
    }
    bit_stream
}

/// Splits a bit stream into 11-bit groups and converts each group to a 16-bit integer.
///
/// # Parameters
/// - `bit_stream`: A vector of bits (as `u8`) representing the entropy and checksum combined.
///
/// # Returns
/// A vector of 16-bit integers, each representing an 11-bit group extracted from the bit stream.
///
/// # Panics
/// This function assumes that the length of `bit_stream` is a multiple of 11. If the length is
/// not a multiple of 11, the behavior is not defined.
fn split_bit_stream(bit_stream: Vec<u8>) -> Vec<u16> {
    let groups: Vec<u16> = bit_stream
        .chunks(11)
        .map(|chunk| chunk.iter().fold(0, |acc, &bit| (acc << 1) | bit as u16))
        .collect();
    groups
}

/// Generates a BIP-39 mnemonic phrase from a vector of 11-bit indices.
///
/// # Parameters
/// - `groups`: A vector of 11-bit indices, each corresponding to a word in the BIP-39 wordlist.
///
/// # Returns
/// A string representing the mnemonic phrase, consisting of words separated by spaces.
///
/// # Panics
/// This function will panic if an index in `groups` is out of bounds for the BIP-39 wordlist.
fn generate_mnemonic(groups: Vec<u16>) -> String {
    let wordlist = fs::read_to_string("english.txt").expect("Failed to read wordlist");
    let words: Vec<&str> = wordlist.lines().collect();

    let mnemonic: Vec<&str> = groups
        .iter()
        .map(|&index| {
            words.get(index as usize).unwrap_or_else(|| {
                eprintln!("Index {} out of bounds for wordlist", index);
                std::process::exit(1);
            })
        })
        .map(|s| *s)
        .collect();
    let mnemonic_phrase = mnemonic.join(" ");
    println!("Mnemonic: {}", mnemonic_phrase);
    mnemonic_phrase
}

/// Converts a BIP-39 mnemonic phrase to a 64-byte seed using PBKDF2-HMAC-SHA512.
///
/// # Parameters
/// - `mnemonic_phrase`: A BIP-39 mnemonic phrase.
///
/// # Returns
/// A 64-byte seed.
///
/// # Notes
/// - The passphrase is the BIP-39 standard prefix for passphrase, which is "mnemonic".
/// - The PBKDF2 iteration count is 2048.
fn bip39_mnemonic_to_seed(mnemonic_phrase: &str) -> [u8; 64] {
    let passphrase = "mnemonic"; // BIP-39 standard prefix for passphrase
    let mut hmac =
        HmacSha512::new_from_slice(passphrase.as_bytes()).expect("HMAC can take key of any size");
    hmac.update(mnemonic_phrase.as_bytes());
    let mut seed = [0u8; 64];
    let _ = pbkdf2::<HmacSha512>(
        mnemonic_phrase.as_bytes(),
        passphrase.as_bytes(),
        2048,
        &mut seed,
    );
    println!(
        "Seed: {}",
        seed.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("")
    );
    seed
}

/// Derives a private key from a seed using BIP-32.
///
/// # Parameters
/// - `seed`: A 64-byte seed used to derive the private key.
///
/// # Returns
/// A tuple containing the Extended Private Key and derived private key.
///
fn private_key_from_seed(seed: [u8; 64]) -> (ExtendedPrivKey, PrivateKey) {
    let secp = Secp256k1::new();
    let network = Network::Bitcoin;
    let xpriv =
        ExtendedPrivKey::new_master(network, &seed).expect("Failed to create master private key");
    let derivation_path = "m/44'/0'/0'/0/0"; // Example derivation path for Bitcoin
    let path = DerivationPath::from_str(derivation_path).expect("Invalid derivation path");
    let child_xpriv = xpriv
        .derive_priv(&secp, &path)
        .expect("Failed to derive child private key");
    let private_key = PrivateKey::new(child_xpriv.private_key, network);
    (xpriv, private_key) // Return both Extended Private Key and private_key
}

/// Main function for generating a BIP-39 mnemonic and converting it to a Bitcoin private key.
///
/// 1. Generate entropy
/// 2. Calculate checksum
/// 3. Append checksum to end of entropy
/// 4. Split entropy into 11-bit groups
/// 5. Generate mnemonic phrase
/// 6. Convert mnemonic phrase to seed
/// 7. Derive private key from seed
fn main() {
    let start_time = Instant::now(); // Start timing
    let entropy = generate128bitentropy(); // Step 1: Generate Entropy
    let (checksum_binary, checksum_bits) = calculate_checksum(entropy); // Step 2: Create Checksum
    let bit_stream = bit_stream(entropy, checksum_binary, checksum_bits); // Step 3: Append Checksum to end of entropy
    let groups = split_bit_stream(bit_stream); // Step 4: Split Entropy into 11-bit groups
    let mnemonic_phrase = generate_mnemonic(groups); // Step 5: Generate Mnemonic Phrase
    let seed = bip39_mnemonic_to_seed(&mnemonic_phrase); // Step 6: Convert Mnemonic Phrase to Seed
    let (xpriv, private_key) = private_key_from_seed(seed); // Step 7: Derive Private Key

    let duration = start_time.elapsed(); // End timing
    println!("Extended Private Key: {}", xpriv);
    println!("Bitcoin Private Key: {}", private_key);
    println!("Total Generation Time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the generate128bitentropy function returns 16 bytes of entropy (128 bits).
    #[test]
    fn test_entropy_generation() {
        let entropy = generate128bitentropy();
        assert_eq!(entropy.len(), 16, "Entropy must be 128 bits (16 bytes)");
    }

    /// Tests that the checksum calculation function produces the correct checksum given a known example
    /// of entropy. If the checksum calculation logic changes, this test will fail.
    #[test]
    fn test_checksum_calculation() {
        // Example entropy and expected checksum
        let entropy: [u8; 16] = [
            0x8d, 0xdd, 0xe6, 0x80, 0xd9, 0x48, 0xf9, 0xe2, 0xcc, 0xde, 0x2f, 0xed, 0x90, 0x08,
            0x96, 0x8e,
        ];
        let (checksum_binary, _) = calculate_checksum(entropy);
        assert_eq!(
            checksum_binary,
            format!("{:b}", 0b1011),
            "Checksum must match the expected value"
        );
    }

    /// Test that the mnemonic generated from entropy and checksum matches the expected mnemonic.
    /// This test ensures that the mnemonic generation function produces the correct sequence of words
    /// by using known entropy, checksum, and expected mnemonic output. If the generation logic changes,
    /// this test will fail.

    #[test]
    fn test_mnemonic_generation() {
        let entropy: [u8; 16] = [
            0x8d, 0xdd, 0xe6, 0x80, 0xd9, 0x48, 0xf9, 0xe2, 0xcc, 0xde, 0x2f, 0xed, 0x90, 0x08,
            0x96, 0x8e,
        ];
        let (checksum_binary, checksum_bits) = calculate_checksum(entropy);
        let bit_stream = bit_stream(entropy, checksum_binary, checksum_bits);
        let groups = split_bit_stream(bit_stream);
        let mnemonic = generate_mnemonic(groups);

        assert_eq!(
            mnemonic,
            vec![
                "miss", "upset", "parent", "raw", "moon", "vapor", "cricket", "shine", "unique",
                "leopard", "certain", "buddy"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" "),
            "Generated mnemonic must match expected output"
        );
    }

    /// Test that the seed derived from a mnemonic matches the expected seed.
    /// This test ensures that the seed derivation function produces a valid seed.
    /// The expected seed is hardcoded in the test, so if the seed derivation
    /// function changes, this test will fail.
    #[test]
    fn test_seed_derivation() {
        let mnemonic =
            "miss upset parent raw moon vapor cricket shine unique leopard certain buddy";
        let seed = bip39_mnemonic_to_seed(mnemonic);
        let expected_seed = hex::decode("8e22fce2b12535bbe0c8ad7cbe5bde60a917f4dd74fd5896b4522f016edeb4085c5c3cb0cc00a0970f3c260c9a22850e21f2d7ef9705bf2359943038370e1c06").unwrap();
        assert_eq!(
            seed.to_vec(),
            expected_seed,
            "Derived seed must match expected value"
        );
    }
}
