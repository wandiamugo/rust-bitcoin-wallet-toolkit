# 🦀 Rust Bitcoin Wallet CLI — Beginner's Toolkit

**Moringa School AI Capstone Project**  
*Prompt-Powered Kickstart: Building a Beginner's Toolkit for Rust (Bitcoin Development)*

---

## 🎯 Project Description

A beginner-friendly toolkit for learning Rust through Bitcoin development. This project builds a working Bitcoin testnet wallet CLI — generating private keys, public keys, WIF-encoded keys, and P2PKH addresses — entirely in Rust, guided by AI-assisted learning techniques from the Moringa AI curriculum.

**Why Rust for Bitcoin?** ~70% of critical security vulnerabilities in large codebases come from memory bugs that Rust eliminates at compile time. For Bitcoin, bugs aren't just crashes — they're financial losses.

---

## ✨ Key Features

- 🔑 Cryptographically secure private key generation using OS-level RNG
- 📐 secp256k1 elliptic curve public key derivation
- 🏠 P2PKH Bitcoin testnet address generation
- 🔐 WIF (Wallet Import Format) private key encoding
- 🎨 Themed ASCII banner CLI with colour-coded output
- 🧪 4-test suite covering correctness, randomness, format, and determinism
- 📓 Full AI Prompt Journal documenting the learning process

---

## 🖥️ System Requirements

- **OS:** macOS / Linux / Windows
- **Editor:** VS Code (recommended)
- **Rust:** v1.70 or later (via rustup)
- **Git:** For cloning the repository

---

## ⚙️ Installation & Setup

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Make PATH permanent (macOS/Linux)
```bash
echo 'source $HOME/.cargo/env' >> ~/.zshrc
source ~/.zshrc
```

### 3. Verify installation
```bash
rustc --version
cargo --version
```

### 4. Clone this repository
```bash
git clone https://github.com/YOUR_USERNAME/rust-bitcoin-wallet-toolkit
cd rust-bitcoin-wallet-toolkit
```

### 5. Install dependencies
```bash
cargo add bitcoin@0.31
cargo add rand@0.8
cargo build
```

> The first build takes 2-3 minutes — Rust is downloading and compiling the Bitcoin and cryptography libraries.

---

## 🚀 Basic Usage

### Run the wallet generator
```bash
cargo run
```

### Run the test suite
```bash
cargo test
```

---

## 💻 Expected Output

```
  ██████╗ ██╗████████╗ ██████╗ ██████╗ ██╗███╗   ██╗
  ██╔══██╗██║╚══██╔══╝██╔════╝██╔═══██╗██║████╗  ██║
  ██████╔╝██║   ██║   ██║     ██║   ██║██║██╔██╗ ██║
  ██╔══██╗██║   ██║   ██║     ██║   ██║██║██║╚██╗██║
  ██████╔╝██║   ██║   ╚██████╗╚██████╔╝██║██║ ╚████║
  ╚═════╝ ╚═╝   ╚═╝    ╚═════╝ ╚═════╝ ╚═╝╚═╝  ╚═══╝

  🦀 Rust Bitcoin Testnet Wallet Generator
  Built with rust-bitcoin v0.31 + secp256k1
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  [1/3] Generating cryptographically secure private key...
  [2/3] Deriving public key via secp256k1...
  [3/3] Computing P2PKH testnet address...

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    WALLET GENERATED SUCCESSFULLY
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Private Key (hex) : 331f449f8514957e8ea16942560d490b...
  Private Key (WIF) : cPJ5PGaiXSA3imcD1QKFuKWAaJWmzxKq...
  Public Key        : 03b79e7d7e57b88d493349ba04b4a73c...
  Testnet Address   : mudoXPzgDoXXhUoUWhYt9mhGRz5hEoYAeh

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  ⚠️  SECURITY NOTICE
  Never share your private key or WIF with anyone.
  This is TESTNET — no real funds are at risk.
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

> Keys and address are randomly generated every run. Format will always match the above.

---

## 🧪 Test Results

```
running 4 tests
test test_two_keys_are_different                    ... ok
test tests::test_private_key_generation             ... ok
test test_testnet_address_format                    ... ok
test test_public_key_derived_from_known_private_key ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured
```

| Test | What it verifies |
|---|---|
| `test_private_key_generation` | Private key is exactly 32 bytes |
| `test_two_keys_are_different` | Random generation never produces duplicates |
| `test_testnet_address_format` | Address always starts with m or n (testnet) |
| `test_public_key_derived_from_known_private_key` | Same private key always yields same public key |

---

## 📁 Code Structure

```
rust-bitcoin-wallet-toolkit/
├── src/
│   └── main.rs          # All wallet logic + tests
├── Cargo.toml           # Dependencies (bitcoin 0.31, rand 0.8)
├── Cargo.lock           # Locked dependency versions
├── README.md            # This file
└── AI_PROMPT_JOURNAL.md # Full record of AI prompts used
```

---

## 🔑 Key Rust Concepts

| Concept | What it means | Python/JS equivalent |
|---|---|---|
| Ownership | One owner per value; freed on scope exit | Garbage collector |
| `&` borrow | Read-only temporary access | Passing a reference |
| `&mut` borrow | Write access, exclusive | Mutable reference |
| `Option<T>` | `Some(v)` or `None` | `null` / `undefined` |
| `Result<T,E>` | `Ok(v)` or `Err(e)` | try/catch |
| Struct + impl | Data + behaviour separated | Class |
| Trait | Shared behaviour contract | Interface / ABC |
| `let` vs `let mut` | Immutable by default | `const` vs `let` |

---

## 🔧 Configuration

The wallet targets **Bitcoin Testnet** by default. To switch to Mainnet, change `Network::Testnet` to `Network::Bitcoin` in two places in `src/main.rs`:

```rust
// In derive_address():
Address::p2pkh(public_key, Network::Bitcoin)  // was Network::Testnet

// In main() WIF encoding:
let wif = PrivateKey::new(secret_key, Network::Bitcoin).to_wif();
```

> ⚠️ Never use Mainnet keys generated by this tool with real funds — this is a learning project.

---

## 🐛 Troubleshooting

| Error | Cause | Fix |
|---|---|---|
| `command not found: cargo` | PATH not loaded | Run `source $HOME/.cargo/env` |
| `cargo build` finishes instantly | Dependencies not saved | Run `cargo add bitcoin@0.31 && cargo add rand@0.8` |
| `cargo test` shows 0 tests | File not saved by VS Code | Write directly via terminal: `cat > src/main.rs` |
| `Secp256k1::new()` in every function | Expensive recreation | Create once in `main()`, pass as `&secp` |
| `.expect()` crashes on error | No error recovery | Replace with `?` operator and `Result` propagation |

---

## 🧠 Learning Reflections

Coming from Python and JavaScript, these were the biggest mental shifts:

**Ownership** — when you hand your car keys to someone, you no longer have them. That's Rust's move semantics. Python passes references everywhere without thinking about it. Rust makes every transfer explicit.

**The borrow checker is not the enemy** — early compiler errors felt like obstacles. By the end, they were surfacing real design problems. Every time it rejected code, it was right.

**Immutability by default** — writing `let` instead of `let mut` by default made it obvious which values actually need to change.

**Explicit errors are self-documenting** — Python exceptions are invisible in function signatures. Rust's `Result<T, E>` means every fallible function says so upfront. For Bitcoin code, this is not optional.

**Traits are not inheritance** — traits replace the behaviour-sharing part of inheritance only. Data sharing uses composition — contain a struct, don't inherit from it.

**AI accelerated learning dramatically** — six structured prompts covering conceptual understanding, step-by-step breakdown, guided implementation, understanding verification, learning through teaching, and test improvement took me from zero Rust to a working Bitcoin wallet CLI in under a week.

---

## 📚 References

### Official Documentation
- [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
- [Rust Standard Library Docs](https://doc.rust-lang.org/std/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

### Bitcoin & Cryptography Crates
- [rust-bitcoin docs (v0.31)](https://docs.rs/bitcoin/0.31.2/bitcoin/)
- [secp256k1 crate docs](https://docs.rs/secp256k1/latest/secp256k1/)
- [rand crate docs](https://docs.rs/rand/0.8.5/rand/)

### Learning Resources
- [Rustlings](https://github.com/rust-lang/rustlings) — hands-on exercises
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Jon Gjengset — Rust for Rustaceans](https://nostarch.com/rust-rustaceans)

### Bitcoin Development
- [Bitcoin Dev Kit (BDK)](https://bitcoindevkit.org) — next step after this project
- [Learn Me a Bitcoin](https://learnmeabitcoin.com) — plain-English Bitcoin internals
- [Bitcoin Developer Guide](https://developer.bitcoin.org/devguide/)
- [Programming Bitcoin — Jimmy Song](https://programmingbitcoin.com)

### Community
- [Rust Users Forum](https://users.rust-lang.org)
- [Bitcoin Stack Exchange](https://bitcoin.stackexchange.com)
- [Rust Discord](https://discord.gg/rust-lang)

---

## 🤝 Contributing

This is a learning project. If you find an error in the documentation or a bug in the code:
1. Fork the repository
2. Create a branch: `git checkout -b fix/your-fix-name`
3. Commit your changes: `git commit -m 'fix: describe your fix'`
4. Push and open a Pull Request

---

## 📄 License

MIT License — free to use, modify, and distribute with attribution.

---

*Built with 🦀 Rust + ₿ Bitcoin + 🤖 AI-assisted learning at Moringa School*
