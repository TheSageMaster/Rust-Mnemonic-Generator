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
- If rust is previously installed, please run a 'rustup update' command
- The `english.txt` file containing the official BIP-39 wordlist ([BIP-39 Wordlist](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt))

### Installation

1. Clone this repository:
   ```
   git clone https://github.com/yourusername/Rust-Mnemonic-Generator.git
   cd Rust-Mnemonic-Generator
   ```

2. Build and create a symlink:
   ```
   cargo build --release
   ln -s ./target/release/Rust-Mnemonic-Generator Rust-Mnemonic-Generator
   ```

3. Run:
   ```
   ./Rust-Mnemonic-Generator
   ```
   or to specify the number of words to generate:
   ```
   ./Rust-Mnemonic-Generator -w 21
   ```
   help can be displayed:
   ```
   ./Rust-Menmonic-Generator -h
   ```

## How It Works

1. Entropy Generation

   - The program generates 128 bits of entropy using Rust's rand crate. This entropy is the starting point for generating a BIP-39 mnemonic.

2. Checksum Calculation

   - A checksum is computed by taking the first (entropy bits / 32) bits of the SHA-256 hash of the entropy. This ensures integrity and adds an extra layer of security to the mnemonic.

3. Bit Stream Construction

   - The entropy and checksum are combined into a single bit stream, which is then split into 11-bit chunks.

4. Mnemonic Phrase Generation

   - Each 11-bit chunk is mapped to a word in the english.txt wordlist. The resulting words form the mnemonic phrase.

5. Seed Derivation

   - The mnemonic phrase is converted into a seed using PBKDF2-HMAC-SHA512 with 2048 iterations. A passphrase can also be added (default is an empty string).

6. Bitcoin Key Derivation

   - The seed is used to generate a master private key (xpriv). Using BIP-32, the program can derive child private keys for specific paths, e.g., m/44'/0'/0'/0/0.
   - Derivation path will be added a command line argument in the future

7. Bitcoin Address generation

   - Step 1: Apply SHA-256 hashing on the compressed public key.
   - Step 2: Apply RIPEMD-160 hashing on the result of the SHA-256.
   - Step 3: Add the version byte (0x00 for Bitcoin mainnet).
   - Step 4: Perform double SHA-256 on the extended result (version byte + RIPEMD-160 hash).
   - Step 5: Take the first 4 bytes of the double SHA-256 result as the checksum.
   - Step 6: Append the checksum to the result from Step 3.
   - Step 7: Base58 encode the result of Step 6.

### Example Output

```
-----------------------------------------------------------------
--------------- Rust Mnemonic Generator [v1.0.4] ----------------
-----------------------------------------------------------------
Number of words: 24
-----------------------------------------------------------------
Entropy (hex): 10228fd78f13af10a487f0f2b2d7e763d27d3e60fc4ec4962f2d23a616feb2b9
-----------------------------------------------------------------
Mnemonic: awake behave vote bullet depend marriage must wrap verify note what sibling chief palace loop mean setup ramp slender elder gesture leg slab cattle
-----------------------------------------------------------------
Seed: 27552d9cae9d645b1b0a0cf45cc3325e58edc1f8a7f8044f934ef8aeb8249cd46391f71baf04fdd30c3c43c1a5ca93305147d05679eb28fe5be543717203c42b
Extended Private Key: xprv9s21ZrQH143K2mbfwKRdBbbUzRQkeoc1qg3K97tUZDGSu3jbjoaG4Jjm54oZHhhJe4W8MApTm9UQokyXbNX9vf2CEgy38fFYE4G98bNATxN
Bitcoin Private Key: KzkAHnM7mtoWsiDznSJ8BkDga7CYU8s5hyA6wzfVF2x9B55RjEaP
Compressed Public Key: 0260ecf665907662f548cc26e3b0dbc4493a10e35d2b49a0af93b96e0a363f8729
Address: 134LiGXL2jUZ9Vy3rkUobgotnmjDGrWng4
-----------------------------------------------------------------
Total Generation Time: 1.901923ms
-----------------------------------------------------------------
```

### Dependencies

1. rand: For entropy generation
2. sha2: For SHA-256 and HMAC-SHA512
3. hmac: For HMAC implementation
4. pbkdf2: For deriving the seed
5. bitcoin: For private key and extended key generation

### Adding Dependencies

The following dependencies are required in your Cargo.toml file:

```
[dependencies]
bs58 = '0.5.1'
num-bigint = "0.4.6"
num-traits = "0.2.15"
num-integer = "0.1.46"
rand = '0.9.0'
sha2 = '0.10'
sha3 = '0.10'
ripemd = '0.1.3'
hmac ={ version = '0.12.1', features = ['std'] }
bitcoin = '0.32.5'
secp256k1 = '0.30.0'
pbkdf2 = '0.12.2'
hex = '0.4.3'
clap = { version = '4.0.29', features = ['derive'] }
```

### Customization

Modify the derivation path (m/44'/0'/0'/0/0) to suit different use cases or standards.

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
