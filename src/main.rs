//
// Rust Project: Rust Mnemonic Generator
// Author:  TheSageMaster
// Date:  2025-04-10
// Description:  A bip-39 mnemonic generator for rust
//

use bitcoin::Network;
use bitcoin::bip32::DerivationPath;
use bitcoin::bip32::Xpriv;
use bitcoin::bip32::Xpub;
use bitcoin::key::PrivateKey;
use bitcoin::secp256k1;
use clap::Parser;
use hmac::{Hmac, Mac};
use bs58::encode;
use pbkdf2::pbkdf2;
use rand::RngCore;
use ripemd::Ripemd160;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256, Sha512};
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use std::vec;
pub type ExtendedPrivKey = Xpriv;
pub type ExtendedPubKey = Xpub;
pub type HmacSha512 = Hmac<Sha512>;

#[derive(Parser, Debug)]
struct Args {
    /// Number of words to generate
    #[arg(short, long, default_value_t = 12)]
    words: usize,
    /// Password to use for mnemonic security
    #[arg(short, long, default_value_t = String::from(""))]
    password: String,
}

/// Generates 128 bits of random entropy and prints it to the console.
fn generate_entropy(entropy_bytes: usize) -> Vec<u8> {
    let mut entropy = vec![0u8; entropy_bytes];
    let mut rng = rand::rngs::ThreadRng::default();
    rng.fill_bytes(&mut entropy);
    println!(
        "Entropy (hex): {}",
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
fn calculate_checksum(entropy: &[u8]) -> (String, usize) {
    let hash = Sha256::digest(entropy);
    let checksum_bit_count = entropy.len() * 8 / 32; // Number of checksum bits (entropy bits / 32)

    // Convert the first byte(s) of the hash to binary and take the leftmost checksum_bit_count bits
    let checksum_binary: String = hash
        .iter()
        .flat_map(|byte| format!("{:08b}", byte).chars().collect::<Vec<char>>()) // Convert hash to binary string
        .take(checksum_bit_count) // Take only the required number of bits
        .collect();
    assert_eq!(
        checksum_binary.len(),
        checksum_bit_count,
        "Checksum binary length mismatch"
    );

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
fn bit_stream(entropy: &[u8], checksum_binary: String, checksum_bits: usize) -> Vec<u8> {
    let mut bit_stream = Vec::with_capacity((entropy.len() * 8) + checksum_bits);
    for byte in entropy {
        for i in (0..8).rev() {
            bit_stream.push((byte >> i) & 1);
        }
    }
    for bit in checksum_binary.chars() {
        bit_stream.push(bit.to_digit(10).unwrap() as u8);
    }
    assert_eq!(
        bit_stream.len(),
        entropy.len() * 8 + checksum_bits,
        "Bit stream length mismatch"
    );
    assert_eq!(
        bit_stream.len() % 11,
        0,
        "Bit stream length should be a multiple of 11"
    );
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
    assert_eq!(groups.len() * 11, bit_stream.len());
    assert_eq!(groups.len(), bit_stream.len() / 11);
    assert_eq!(bit_stream.len() % 11, 0);
    assert_eq!(bit_stream.len() / 11, groups.len());
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
    println!("-----------------------------------------------------------------");
    println!("Mnemonic: {}", mnemonic_phrase);
    println!("-----------------------------------------------------------------");
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
fn bip39_mnemonic_to_seed(mnemonic_phrase: &str, mnemonic_passphrase: &str) -> [u8; 64] {
    let mnemonic_passphrase = format!("{}{}", "mnemonic", mnemonic_passphrase);
    let mut hmac = HmacSha512::new_from_slice(mnemonic_passphrase.as_bytes())
        .expect("HMAC can take key of any size");
    hmac.update(mnemonic_phrase.as_bytes());
    let mut seed = [0u8; 64];
    let _ = pbkdf2::<HmacSha512>(
        mnemonic_phrase.as_bytes(),
        mnemonic_passphrase.as_bytes(),
        2048,
        &mut seed,
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

/// Converts a PrivateKey to a compressed public key.
///
/// # Parameters
/// - `private_key`: A PrivateKey
///
/// # Returns
/// A vector of 33 bytes representing the compressed public key, or an error string if the private key is invalid.
///
/// # Notes
/// - The public key is generated using the Secp256k1 library.
/// - The public key is serialized in compressed form.
fn generate_public_key(private_key: &PrivateKey) -> Result<Vec<u8>, String> {
    // Step 1: Decode the hex private key to bytes
    let private_key_bytes = private_key.to_bytes();

    // Step 2: Create a SecretKey from bytes
    let secret_key = SecretKey::from_slice(&private_key_bytes)
        .map_err(|e| format!("Invalid private key: {}", e))?;

    // Step 3: Generate the public key using Secp256k1
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // Step 4: Serialize it in compressed form
    let compressed = public_key.serialize(); // [u8; 33]

    Ok(compressed.to_vec())
}

/// Generates a Bitcoin address from a given public key.
///
/// # Parameters
/// - `public_key`: A compressed public key (33 bytes)
///
/// # Returns
/// A Bitcoin address as a string, or an error message if the input is invalid.
///
/// # Notes
/// - The address is generated using the standard Bitcoin address format.
/// - The address is encoded in base58check.
fn generate_address(public_key: Vec<u8>) -> Result<String, String> {
    let stage1 = Sha256::digest(&public_key);
    let stage2 = Ripemd160::digest(&stage1);
    let versioned_payload = [&[0x00], &stage2[..]].concat();
    let checksum = &Sha256::digest(&Sha256::digest(&versioned_payload))[0..4];
    let address_bytes = [&versioned_payload[..], checksum].concat();
    Ok(encode(address_bytes).into_string())
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
    // Added command line argument to specify number of words to generate
    let args = Args::parse();

    let num_words = args.words;
    eprintln!("-----------------------------------------------------------------");
    eprintln!("--------------- Rust Mnemonic Generator [v1.0.4] ----------------");
    eprintln!("-----------------------------------------------------------------");
    println!("Number of words: {}", num_words);
    eprintln!("-----------------------------------------------------------------");

    let entropy_bits = num_words * 11 - (num_words / 3);
    let entropy_bytes = entropy_bits / 8;

    if entropy_bits % 8 != 0 {
        panic!("Entropy bits must be divisible by 8");
    }

    if entropy_bytes > 32 {
        panic!("Entropy bytes must be less than or equal to 32");
    }

    if !(12..=24).contains(&num_words) {
        panic!("Number of words must be between 12 and 24");
    }

    if num_words % 3 != 0 {
        panic!("Number of words must be divisible by 3");
    }

    let start_time = Instant::now(); // Start timing
    let entropy = generate_entropy(entropy_bytes); // Step 1: Generate Entropy
    let (checksum_binary, checksum_bits) = calculate_checksum(&entropy); // Step 2: Create Checksum
    let bit_stream = bit_stream(&entropy, checksum_binary, checksum_bits); // Step 3: Append Checksum to end of entropy
    let groups = split_bit_stream(bit_stream); // Step 4: Split Entropy into 11-bit groups
    let mnemonic_phrase = generate_mnemonic(groups); // Step 5: Generate Mnemonic Phrase
    let unsecured_seed = bip39_mnemonic_to_seed(&mnemonic_phrase, ""); // Step 6: Convert Mnemonic Phrase to Seed
    eprintln!(
        "Seed: {}",
        unsecured_seed
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("")
    );
    let (xpriv, private_key) = private_key_from_seed(unsecured_seed); // Step 7: Derive Private Key
    let compressed_public_key =
        generate_public_key(&private_key).expect("Failed to generate public key");
    let address = generate_address(compressed_public_key.clone()).expect("Failed to generate address");

    eprintln!("Extended Private Key: {}", xpriv);
    eprintln!("Bitcoin Private Key: {}", private_key);
    eprintln!("Compressed Public Key: {}",hex::encode(compressed_public_key));
    eprintln!("Address: {}", address);

    let duration = start_time.elapsed(); // End timing
    eprintln!("-----------------------------------------------------------------");
    eprintln!("Total Generation Time: {:?}", duration);
    eprintln!("-----------------------------------------------------------------");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the generate128bitentropy function returns 16 bytes of entropy (128 bits).
    #[test]
    fn test_entropy_generation() {
        let entropy = generate_entropy(16);
        assert_eq!(entropy.len(), 16, "Entropy must be 128 bits (16 bytes)");
    }

    /// Tests that the checksum calculation function produces the correct checksum given a known example
    /// of entropy. If the checksum calculation logic changes, this test will fail.
    #[test]
    fn test_checksum_calculation() {
        // Example entropy and expected checksum
        let entropy: [u8; 32] = [
            0x25, 0xc9, 0x63, 0xfc, 0x2e, 0xcc, 0x64, 0x28, 0x41, 0xf2, 0x53, 0xd7, 0xca, 0x08,
            0xae, 0x7b, 0x10, 0x91, 0x4f, 0x46, 0x95, 0xc6, 0x0a, 0x0c, 0xa8, 0x9b, 0xa6, 0x22,
            0x2f, 0xf7, 0x72, 0xaf,
        ];
        let (checksum_binary, _) = calculate_checksum(&entropy);
        assert_eq!(
            checksum_binary,
            format!("{:b}", 0b10011011),
            "Checksum must match the expected value"
        );
    }

    /// Test that the mnemonic generated from entropy and checksum matches the expected mnemonic.
    /// This test ensures that the mnemonic generation function produces the correct sequence of words
    /// by using known entropy, checksum, and expected mnemonic output. If the generation logic changes,
    /// this test will fail.

    #[test]
    fn test_mnemonic_generation() {
        let entropy: [u8; 32] = [
            0x25, 0xc9, 0x63, 0xfc, 0x2e, 0xcc, 0x64, 0x28, 0x41, 0xf2, 0x53, 0xd7, 0xca, 0x08,
            0xae, 0x7b, 0x10, 0x91, 0x4f, 0x46, 0x95, 0xc6, 0x0a, 0x0c, 0xa8, 0x9b, 0xa6, 0x22,
            0x2f, 0xf7, 0x72, 0xaf,
        ];
        let (checksum_binary, checksum_bits) = calculate_checksum(&entropy);
        let bit_stream = bit_stream(&entropy, checksum_binary, checksum_bits);
        let groups = split_bit_stream(bit_stream);
        let mnemonic = generate_mnemonic(groups);

        assert_eq!(
            mnemonic,
            vec![
                "chair", "enrich", "yellow", "frown", "shock", "before", "amazing", "engine",
                "style", "expect", "clog", "wage", "animal", "police", "bottom", "rhythm",
                "anxiety", "grab", "cheap", "era", "carry", "wing", "skirt", "vibrant"
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
        let mnemonic = "chair enrich yellow frown shock before amazing engine style expect clog wage animal police bottom rhythm anxiety grab cheap era carry wing skirt vibrant";
        let seed = bip39_mnemonic_to_seed(mnemonic, "mnemonic");
        let expected_seed = hex::decode("b9d54590734644b7374bcf6521ca8cf024803801ce44c21b467434d33008c41c07d40fa43495dfbfd0808054220eae00ea5d65ad12ea1506aeb50f2a3c7bce67").unwrap();
        assert_eq!(
            seed.to_vec(),
            expected_seed,
            "Derived seed must match expected value"
        );
    }
}
