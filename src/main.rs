use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Address, Network, PublicKey};
use rand::rngs::OsRng;
use rand::RngCore;

fn main() {
    println!("=== Bitcoin Testnet Wallet Generator ===\n");

    // Created once — passed to any function that needs it
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    let secret_key = generate_private_key(&mut rng);
    let public_key = derive_public_key(&secp, &secret_key);
    let address    = derive_address(&public_key);

    let private_key_hex: String = secret_key
        .secret_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();

    println!("Private Key (hex) : {}", private_key_hex);
    println!("Public Key        : {}", public_key);
    println!("Testnet Address   : {}", address);
    println!("\n⚠️  Never share your private key. This is testnet — no real funds.");
}

fn generate_private_key(rng: &mut OsRng) -> SecretKey {
    let mut key_bytes = [0u8; 32];
    rng.fill_bytes(&mut key_bytes);
    SecretKey::from_slice(&key_bytes)
        .expect("Failed to create secret key")
}

fn derive_public_key(
    secp: &Secp256k1<bitcoin::secp256k1::All>,
    secret_key: &SecretKey,
) -> PublicKey {
    let inner = bitcoin::secp256k1::PublicKey::from_secret_key(secp, secret_key);
    PublicKey { compressed: true, inner }
}

fn derive_address(public_key: &PublicKey) -> Address {
    Address::p2pkh(public_key, Network::Testnet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;

    #[test]
    fn test_private_key_generation() {
        let mut rng = OsRng;
        let key = generate_private_key(&mut rng);
        //a valid private key should be exactly 32 bytes
        assert_eq!(key.secret_bytes().len(), 32);
    }
}

#[test]
    fn test_two_keys_are_different() {
        let mut rng = OsRng;
        let key1 = generate_private_key(&mut rng);
        let key2 = generate_private_key(&mut rng);
        // two randomly generated keys should never be the same
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
        // testnet P2PKH addresses always start with m or n
        assert!(
            addr_str.starts_with('m') || addr_str.starts_with('n'),
            "Expected testnet address starting with m or n, got: {}",
            addr_str
        );
    }

    #[test]
    fn test_public_key_derived_from_known_private_key() {
        let secp = Secp256k1::new();
        // use a fixed known private key so the test is deterministic
        let key_bytes = [1u8; 32];
        let secret_key = SecretKey::from_slice(&key_bytes)
            .expect("Failed to create key");
        let public_key = derive_public_key(&secp, &secret_key);
        // same private key should always produce same public key
        let public_key2 = derive_public_key(&secp, &secret_key);
        assert_eq!(public_key.to_string(), public_key2.to_string());
    }