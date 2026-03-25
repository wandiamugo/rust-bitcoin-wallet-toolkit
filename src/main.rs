use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Address, Network, PrivateKey, PublicKey};
use rand::rngs::OsRng;
use rand::RngCore;

// =============================================================================
// Bitcoin Key Generation Pipeline
// =============================================================================
//
//   OsRng  ──►  generate_private_key  ──►  derive_public_key  ──►  derive_address
//   (entropy)      (SecretKey)               (PublicKey)            (Address)
//
// Each arrow is a one-way cryptographic transformation. Knowing the address
// reveals nothing about the public key; knowing the public key reveals nothing
// about the private key. This asymmetry is the foundation of Bitcoin's security.
// =============================================================================

const ORANGE: &str = "\x1b[38;5;214m";
const GREEN:  &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN:   &str = "\x1b[36m";
const BOLD:   &str = "\x1b[1m";
const RESET:  &str = "\x1b[0m";

fn print_banner() {
    println!("{}{}", ORANGE, BOLD);
    println!("  ██████╗ ██╗████████╗ ██████╗ ██████╗ ██╗███╗   ██╗");
    println!("  ██╔══██╗██║╚══██╔══╝██╔════╝██╔═══██╗██║████╗  ██║");
    println!("  ██████╔╝██║   ██║   ██║     ██║   ██║██║██╔██╗ ██║");
    println!("  ██╔══██╗██║   ██║   ██║     ██║   ██║██║██║╚██╗██║");
    println!("  ██████╔╝██║   ██║   ╚██████╗╚██████╔╝██║██║ ╚████║");
    println!("  ╚═════╝ ╚═╝   ╚═╝    ╚═════╝ ╚═════╝ ╚═╝╚═╝  ╚═══╝");
    println!();
    println!("  🦀 Rust Bitcoin Testnet Wallet Generator");
    println!("  Built with rust-bitcoin v0.31 + secp256k1");
    println!("{}", RESET);
    println!("  {}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}", CYAN, RESET);
    println!();
}

fn main() {
    print_banner();

    // Created once — Secp256k1::new() precomputes lookup tables and is
    // expensive. Pass it to functions rather than recreating per call.
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    println!("  {}[1/3]{} Generating cryptographically secure private key...", CYAN, RESET);
    let secret_key = generate_private_key(&mut rng);

    println!("  {}[2/3]{} Deriving public key via secp256k1...", CYAN, RESET);
    let public_key = derive_public_key(&secp, &secret_key);

    println!("  {}[3/3]{} Computing P2PKH testnet address...", CYAN, RESET);
    let address = derive_address(&public_key);

    let wif = PrivateKey::new(secret_key, Network::Testnet).to_wif();

    let private_key_hex: String = secret_key
        .secret_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();

    println!();
    println!("  {}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}", CYAN, RESET);
    println!("  {}{}  WALLET GENERATED SUCCESSFULLY{}", BOLD, GREEN, RESET);
    println!("  {}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}", CYAN, RESET);
    println!();
    println!("  {}Private Key (hex){} : {}", BOLD, RESET, private_key_hex);
    println!("  {}Private Key (WIF){} : {}", BOLD, RESET, wif);
    println!("  {}Public Key       {} : {}", BOLD, RESET, public_key);
    println!("  {}{}Testnet Address  {} : {}{}{}", BOLD, GREEN, RESET, GREEN, address, RESET);
    println!();
    println!("  {}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}", CYAN, RESET);
    println!("  {}⚠️  SECURITY NOTICE{}", YELLOW, RESET);
    println!("  {}Never share your private key or WIF with anyone.{}", YELLOW, RESET);
    println!("  {}This is TESTNET — no real funds are at risk.{}", YELLOW, RESET);
    println!("  {}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}", CYAN, RESET);
    println!();
}

/// Generates a cryptographically secure random private key for a Bitcoin wallet.
///
/// Uses the OS entropy source ([`OsRng`]) to fill a 32-byte buffer, then
/// validates the result against the secp256k1 curve order. The entire security
/// of the wallet depends on this value being unpredictable and secret.
///
/// # Panics
/// Panics if the random bytes fall outside the valid secp256k1 scalar range
/// (`0 < key < n`). Probability ~1 in 2^128 — exists to satisfy the type
/// system, not to handle a realistic failure.
///
/// # Security
/// - Always pass [`OsRng`] — never `thread_rng()` for key material
/// - Never log or serialise the returned [`SecretKey`] in production
fn generate_private_key(rng: &mut OsRng) -> SecretKey {
    let mut key_bytes = [0u8; 32];
    rng.fill_bytes(&mut key_bytes);
    SecretKey::from_slice(&key_bytes)
        .expect("Failed to create secret key — not a valid secp256k1 scalar")
}

/// Derives a compressed secp256k1 public key from a private key.
///
/// Computes `P = k × G` (elliptic curve scalar multiplication) where `G` is
/// the secp256k1 generator point. One-way: recovering `k` from `P` requires
/// solving ECDLP, which is computationally infeasible.
///
/// # Parameters
/// - `secp`: Shared context with precomputed tables. Never reconstruct per call.
/// - `secret_key`: Borrowed — caller retains ownership after derivation.
///
/// # Returns
/// [`PublicKey`] in compressed format (33 bytes). Safe to display and share.
fn derive_public_key(
    secp: &Secp256k1<bitcoin::secp256k1::All>,
    secret_key: &SecretKey,
) -> PublicKey {
    let inner = bitcoin::secp256k1::PublicKey::from_secret_key(secp, secret_key);
    // compressed: true = 33-byte format (parity prefix + x-coordinate only)
    // Compressed and uncompressed keys produce DIFFERENT addresses — always
    // use compressed for post-2012 Bitcoin.
    PublicKey { compressed: true, inner }
}

/// Derives a P2PKH testnet address from a compressed public key.
///
/// Internally computes SHA-256 → RIPEMD-160 → version byte → checksum →
/// Base58Check encoding. Testnet P2PKH addresses always begin with `m` or `n`.
///
/// # Note
/// Hardcoded to [`Network::Testnet`]. For production use, accept `network`
/// as a parameter. **Never send mainnet funds to a testnet address.**
fn derive_address(public_key: &PublicKey) -> Address {
    Address::p2pkh(public_key, Network::Testnet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::Secp256k1;
    use rand::rngs::OsRng;

    #[test]
    fn test_private_key_generation() {
        let mut rng = OsRng;
        let key = generate_private_key(&mut rng);
        assert_eq!(key.secret_bytes().len(), 32);
    }

    #[test]
    fn test_two_keys_are_different() {
        let mut rng = OsRng;
        let key1 = generate_private_key(&mut rng);
        let key2 = generate_private_key(&mut rng);
        assert_ne!(key1.secret_bytes(), key2.secret_bytes());
    }

    #[test]
    fn test_testnet_address_format() {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let secret_key = generate_private_key(&mut rng);
        let public_key = derive_public_key(&secp, &secret_key);
        let address = derive_address(&public_key);
        let addr_str = address.to_string();
        assert!(
            addr_str.starts_with('m') || addr_str.starts_with('n'),
            "Expected testnet address starting with m or n, got: {}",
            addr_str
        );
    }

    #[test]
    fn test_public_key_derived_from_known_private_key() {
        let secp = Secp256k1::new();
        let key_bytes = [1u8; 32];
        let secret_key = SecretKey::from_slice(&key_bytes)
            .expect("Failed to create key");
        let public_key  = derive_public_key(&secp, &secret_key);
        let public_key2 = derive_public_key(&secp, &secret_key);
        assert_eq!(public_key.to_string(), public_key2.to_string());
    }
}