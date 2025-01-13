use bitcoin::secp256k1;
use bitcoin::Network;
use bitcoin::bip32::Xpriv;
use bitcoin::bip32::Xpub;
use bitcoin::key::PrivateKey;
use bitcoin::bip32::DerivationPath;
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::{Sha256, Sha512, Digest};
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use secp256k1::Secp256k1;
use pbkdf2::pbkdf2;
pub type ExtendedPrivKey = Xpriv;
pub type ExtendedPubKey = Xpub;
pub type HmacSha512 = Hmac<Sha512>;

fn main() {
    let start_time = Instant::now(); // Start timing

    // Step 1: Generate 128 bits of entropy
    let mut entropy = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut entropy);
    println!("Entropy: {}", entropy.iter().map(|b| format!("{:02x}", b)).collect::<String>());

    // Step 2: Create Checksum
    let hash = Sha256::digest(&entropy);
    let checksum_bits = entropy.len() * 8 / 32; // Calculate number of checksum bits
    let checksum = (hash[0] >> (8 - checksum_bits)) & ((1 << checksum_bits) - 1); // Extract only the required checksum bits

    let checksum_binary = format!("{:0width$b}", checksum, width = checksum_bits);

    // Step 3: Append Checksum to end of entropy (as bits)
    let mut bit_stream = Vec::with_capacity((entropy.len() * 8) + checksum_bits);
    for byte in &entropy {
        for i in (0..8).rev() {
            bit_stream.push((byte >> i) & 1);
        }
    }
    for bit in checksum_binary.chars() {
        bit_stream.push(bit.to_digit(10).unwrap() as u8);
    }

    // Step 4: Split into groups of 11 bits
    let groups: Vec<u16> = bit_stream.chunks(11).map(|chunk| {
        chunk.iter().fold(0, |acc, &bit| (acc << 1) | bit as u16)
    }).collect();

    // Step 6: Select word from english.txt wordlist
    let wordlist = fs::read_to_string("english.txt").expect("Failed to read wordlist");
    let words: Vec<&str> = wordlist.lines().collect();

    let mnemonic: Vec<&str> = groups.iter().map(|&index| {
        words.get(index as usize).unwrap_or_else(|| {
            eprintln!("Index {} out of bounds for wordlist", index);
            std::process::exit(1);
        })
    }).map(|s| *s).collect();

    // Step 7: Display Mnemonic
    let mnemonic_phrase = mnemonic.join(" ");
    println!("Mnemonic: {}", mnemonic_phrase);

    // Step 8: Convert Mnemonic to Seed (BIP-39 style, assuming a passphrase of an empty string)
    let passphrase = "mnemonic"; // BIP-39 standard prefix for passphrase

    let mut hmac = HmacSha512::new_from_slice(passphrase.as_bytes()).expect("HMAC can take key of any size");
    hmac.update(mnemonic_phrase.as_bytes());

    let mut seed = [0u8; 64];
    let _ = pbkdf2::<HmacSha512>(mnemonic_phrase.as_bytes(), passphrase.as_bytes(), 2048, &mut seed);

    println!("Seed: {}", seed.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(""));

    // Step 9: Convert Seed to Bitcoin Private Key
    let secp = Secp256k1::new();
    let network = Network::Bitcoin;
    let xpriv = ExtendedPrivKey::new_master(network, &seed).expect("Failed to create master private key");

    println!("Extended Private Key: {}", xpriv);

    let derivation_path = "m/44'/0'/0'/0/0"; // Example derivation path for Bitcoin
    let path = DerivationPath::from_str(derivation_path).expect("Invalid derivation path");
    let child_xpriv = xpriv.derive_priv(&secp, &path).expect("Failed to derive child private key");

    let private_key = PrivateKey::new(child_xpriv.private_key, network);
    println!("Bitcoin Private Key: {}", private_key);

    let duration = start_time.elapsed(); // End timing
    println!("Total Generation Time: {:?}", duration);
}
