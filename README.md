# Rust-Mnemonic-Generator

A Rust-based implementation for generating BIP-39 mnemonics, converting them into seeds, and deriving Bitcoin private keys. This project demonstrates how to work with entropy, checksum calculation, and mnemonic wordlists to conform to the BIP-39 standard.

## Features

- Generates 128 bits of entropy
- Computes the BIP-39 checksum
- Constructs mnemonic phrases using the English wordlist
- Derives a seed from the mnemonic using PBKDF2-HMAC-SHA512
- Supports Bitcoin key generation, including master and child private keys
- Follows BIP-39 and BIP-32 standards

## Getting Started

### Prerequisites

To run this project, you need:

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))
- The `english.txt` file containing the official BIP-39 wordlist ([BIP-39 Wordlist](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt))

### Installation

1. Clone this repository:
   ``bash
   git clone https://github.com/yourusername/Rust-Mnemonic-Generator.git
   cd Rust-Mnemonic-Generator``

2. Build:
   ``bash
   cargo build --release
   ln ./target/release/Rust-Mnemonic-Generator Rust-Mnemonic-Generator``

3. Run:
   ``bash
   ./Rust-Mnemonic-Generator``

## How It Works

1. Entropy Generation

The program generates 128 bits of entropy using Rust's rand crate. This entropy is the starting point for generating a BIP-39 mnemonic.

2. Checksum Calculation

A checksum is computed by taking the first (entropy bits / 32) bits of the SHA-256 hash of the entropy. This ensures integrity and adds an extra layer of security to the mnemonic.

3. Bit Stream Construction

The entropy and checksum are combined into a single bit stream, which is then split into 11-bit chunks.

4. Mnemonic Phrase Generation

Each 11-bit chunk is mapped to a word in the english.txt wordlist. The resulting words form the mnemonic phrase.

5. Seed Derivation

The mnemonic phrase is converted into a seed using PBKDF2-HMAC-SHA512 with 2048 iterations. A passphrase can also be added (default is an empty string).

6. Bitcoin Key Derivation

The seed is used to generate a master private key (xpriv). Using BIP-32, the program can derive child private keys for specific paths, e.g., m/44'/0'/0'/0/0.

### Example Output

``bash
Entropy: 8bbf9deade037ba27ac2d8e16f7cfa7d
Mnemonic: menu bicycle sort wool theory prefer measure february install disease dirt good ability
Seed: 2d54258d147fb594dd7ef5a416f432ae219da8f0da8ee7d0ce451e10fda0c635ceda594b6213da794ec6787c8bd4cf9437ab3246ac698cc9ced4a36ae6042f66
Extended Private Key: xprv9s21ZrQH143K4TC62fybTm7NPrVXP69z3MykR8aGP5nvgz7z5L266EnxqtkqLnQr3DS8a4XfP3Xs1bguSnRNgUdEUa6Kvr7Jm274EcSqHi7
Bitcoin Private Key: L1qYVuTvaMSpnXfEm271HTvBdPhcxQWUmAp1Ep4nTKLJWQYgXQyg
Total Generation Time: 2.8ms``

### Dependencies

1. rand: For entropy generation
2. sha2: For SHA-256 and HMAC-SHA512
3. hmac: For HMAC implementation
4. pbkdf2: For deriving the seed
5. bitcoin: For private key and extended key generation

### Adding Dependencies

The following dependencies are required in your Cargo.toml file:

``[dependencies]
rand = "0.8"
sha2 = "0.10"
sha3 = "0.10"
hmac ={ version = "0.12.1", features = ["std"] }
bitcoin = "0.32.5"
secp256k1 = "0.30.0"
pbkdf2 = "0.12.2"``

### Customization

Modify the derivation path (m/44'/0'/0'/0/0) to suit different use cases or standards.

Add a passphrase for mnemonic-to-seed conversion for enhanced security.

### Contributing

Contributions are welcome! Please fork this repository and submit a pull request for any improvements.

### License

This project is licensed under the MIT License. See the LICENSE file for details.

### Release of Liability

This software is provided "as is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the software or the use or other dealings in the software.

The use of this software is entirely at your own risk. The authors are not responsible for any loss of funds, data, or any other damages resulting from the use of this software. Use responsibly and always verify your keys and data independently.

### Acknowledgments

1. [BIP-39 Specification](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
2. [BIP-32 Specification](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
3. [Ian Coleman's Mnemonic Tool](https://github.com/iancoleman/bip39)


### Show some support!

If you find this project useful, consider supporting its development by donating:

Bitcoin (BTC): bc1pk8k0z99675uvlzg3v7tm3afllhlm7zrsufzpa9f4xtk80e3mknwq5yl2mu

Ethereum (ETH): 0x0df8F37A6a3d02C8438229C591F0642af2D881E9

Other Tokens: Reach out to the project maintainer.

Thank you for your support!