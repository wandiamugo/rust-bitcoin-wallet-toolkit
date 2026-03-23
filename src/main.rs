// ============================================================
// Rust Bitcoin Wallet — Testnet Key & Address Generation
// ============================================================
// Crates used:
//   bitcoin = "0.31"
//   rand    = "0.8"
// ============================================================

// `use` is Rust's import system — like Python's `import` or JS's `import { } from`
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Address, Network, PublicKey};

// rand is for cryptographically-seeded random number generation
use rand::rngs::OsRng;
use rand::RngCore;

// ============================================================
// ENTRY POINT
// ============================================================
// `fn main()` is the entry point — like `if __name__ == "__main__"` in Python
// or the top-level code in a Node.js script.
fn main() {
    println!("=== Bitcoin Testnet Wallet Generator ===\n");

    // --- Step 1: Generate a random private key ---
    let secret_key = generate_private_key();

    // --- Step 2: Derive the public key ---
    // `&secret_key` passes a borrow — we lend it to the function without
    // giving up ownership. The function reads it; we keep it.
    let public_key = derive_public_key(&secret_key);

    // --- Step 3: Generate a testnet Bitcoin address ---
    let address = derive_address(&public_key);

    // --- Display results ---
    // `secret_key.secret_bytes()` returns a [u8; 32] — a fixed array of 32 bytes.
    // We iterate over them and format each as 2-digit hex, then collect into a String.
    // `format!("{:02x}", b)` is like Python's f"{b:02x}"
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

// ============================================================
// STEP 1: GENERATE A PRIVATE KEY
// ============================================================
// `fn` declares a function.
// `-> SecretKey` is the return type — Python has no equivalent syntax,
//   TypeScript would write `: SecretKey` after the parens.
//
// No parameters — this function only produces a value.
fn generate_private_key() -> SecretKey {
    println!("[1/3] Generating private key...");

    // `OsRng` is the OS-level cryptographically secure RNG.
    // Always use this for key material — never use `thread_rng()`.
    // Python equivalent: `secrets.token_bytes(32)`
    // JS equivalent:     `crypto.getRandomValues(new Uint8Array(32))`
    let mut rng = OsRng;

    // `let mut` — `mut` opts into mutability. Rust is immutable by default.
    // `[0u8; 32]` = array of 32 `u8` bytes, all zero.
    // `u8` = unsigned 8-bit integer (0-255). Python/JS just call these "bytes".
    let mut key_bytes = [0u8; 32];

    // `fill_bytes` writes random data into our array.
    // `&mut key_bytes` is a mutable borrow — we let the function write into
    // our array, then we get it back when the call returns.
    rng.fill_bytes(&mut key_bytes);

    // `SecretKey::from_slice(...)` — `::` calls an associated function
    // (like a static/class method in Python) on the `SecretKey` type.
    //
    // Returns `Result<SecretKey, Error>`.
    // `.expect("msg")` unwraps Ok(value) → value, or panics with "msg" on Err.
    // Safe here because the chance of invalid random bytes is ~1 in 2^128.
    //
    // KEY RUST SYNTAX: No `return` keyword — the last expression WITHOUT a
    // semicolon IS the return value. Add a semicolon and it returns nothing (unit).
    SecretKey::from_slice(&key_bytes).expect("Failed to create secret key — astronomically unlikely")
}

// ============================================================
// STEP 2: DERIVE THE PUBLIC KEY
// ============================================================
// `&SecretKey` — we borrow, not own. The caller keeps the secret key after this call.
// `-> PublicKey` — we return an owned public key to the caller.
fn derive_public_key(secret_key: &SecretKey) -> PublicKey {
    println!("[2/3] Deriving public key...");

    // `Secp256k1::new()` creates the elliptic curve context.
    // Bitcoin uses the secp256k1 curve for all key operations.
    // This is the same curve Ethereum uses.
    let secp = Secp256k1::new();

    // Derive the public key: mathematically, public_key = secret_key * G
    // where G is the curve's generator point. This is a one-way operation —
    // you cannot reverse it to recover the private key.
    let inner = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, secret_key);

    // Struct literal syntax: `TypeName { field: value, field: value }`
    // Python equivalent: `PublicKey(compressed=True, inner=inner)`
    // `compressed: true` tells Bitcoin to use the 33-byte compressed format
    // (vs the older 65-byte uncompressed format). Always use compressed.
    PublicKey {
        compressed: true,
        inner,       // shorthand for `inner: inner` when name matches variable
    }
}

// ============================================================
// STEP 3: DERIVE THE BITCOIN ADDRESS
// ============================================================
// Takes a reference to a PublicKey, returns an owned Address.
fn derive_address(public_key: &PublicKey) -> Address {
    println!("[3/3] Generating testnet address...");

    // `Network::Testnet` is an enum variant.
    // Rust enums are more powerful than Python's — they can carry data.
    // Here we use a simple variant with no attached data.
    //
    // `Address::p2pkh` generates a Pay-to-Public-Key-Hash address —
    // the classic Bitcoin address format.
    // On testnet: starts with 'm' or 'n'
    // On mainnet: starts with '1'
    //
    // Internally this computes: SHA256(RIPEMD160(public_key_bytes))
    // then encodes with Base58Check. The crate handles all of this.
    Address::p2pkh(public_key, Network::Testnet)
}