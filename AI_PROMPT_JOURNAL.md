# 📓 AI Prompt Journal — Rust Bitcoin Wallet Toolkit
**Student:** WANDIA MUGO  
**Capstone Project:** Building a Beginner's Toolkit for Rust (Bitcoin Development)  
**AI Tool Used:** ai.moringaschool.com, claude.ai  

---

## How to Use This Journal
For every prompt I use during this project, I record:
- The prompt technique used (from Moringa AI Module)
- The exact prompt and the response
- A summary of the AI's response
- My evaluation of how helpful it was

---

## Prompt #1 — Conceptual Understanding
**Date:** 21st March 2025  
**Technique:** Section 1.1 - Conceptual Understanding  

**Prompt Used:**
> "I'm currently proficient in Python and JavaScript and want to learn Rust
> for Bitcoin development. Before diving into code:
> 1. What are the key philosophical differences between Rust and Python/JS?
> 2. What problems was Rust designed to solve?
> 3. What mental models should I adjust coming from Python/JS?
> 4. What are common misconceptions Python/JS developers have about Rust?"

**AI Response**
Rust Through Python/JS Eyes: A Conceptual Guide
1. Key Philosophical Differences
Python's philosophy: "We're all consenting adults here." Trust the developer, maximize readability, runtime handles the messy bits.
JavaScript's philosophy: "Just make it work." Flexibility above all, coerce types, patch problems at runtime.
Rust's philosophy: "If it compiles, it's probably correct." The compiler is your co-author, not your adversary. Rust front-loads all the hard thinking — memory, concurrency, nullability — into compile time, so runtime surprises are nearly eliminated.
The core difference is where responsibility lives:
Concern                     Python/JS                      Rust
Memory management           Runtime GC                     You (guided by compiler)
Null safety                 Runtime crash                  Compile-time Option<T>
Concurrency bugs            Runtime race                   Compile-time rejection
Type errors                 Runtime (JS)/gradual (Python)  Compile-time, always
Error handling              Exceptions (implicit)          Result<T, E> (explicit)

2. What Problems Was Rust Designed to Solve?
Rust was born at Mozilla in ~2010 to solve a very specific problem: systems code that is both fast and safe — something C/C++ couldn't guarantee.

The core problem trio:

Memory bugs — C/C++ let you use memory after freeing it (use-after-free), write past buffers, and leak memory. These cause ~70% of critical CVEs in large codebases (including in Bitcoin Core historically).
Data races — two threads touching the same memory simultaneously, silently corrupting state. GC languages avoid this by being slow; C avoids it by pretending it's your problem.
The "pick two" trap — before Rust, you could have fast, safe, or concurrent — but not all three without heroic effort.

Rust's answer was the ownership system: a set of compile-time rules that make the above bugs inexpressible, not just unlikely. For Bitcoin specifically, this matters enormously — bugs in consensus or wallet code aren't just crashes, they're financial losses.

3. Mental Models to Adjust
From Python/JS → Rust, you need to unlearn these reflexes:
① "Variables are boxes I can freely share"
In Python/JS, you pass references everywhere and never think about it. In Rust, every value has exactly one owner. When you assign or pass a value, you may be moving it — the original binding becomes invalid.
Python brain:  a = [1,2,3]; b = a; print(a)  # works fine
Rust brain:    let a = vec![1,2,3]; let b = a; // a is GONE. b owns it now.
② "I'll handle errors later / just throw"
Python/JS exceptions are invisible in function signatures — callers don't know what might blow up. Rust forces errors into the return type: Result<Value, Error>. Every caller must decide what to do. This is not verbose bureaucracy — it's self-documenting contracts.
③ "The type system is for catching typos"
In Python (even with mypy) and JS (even with TypeScript), types are largely advisory. In Rust, types encode business logic. An unvalidated Bitcoin address and a validated one can be different types, making it impossible to confuse them.
④ "Mutability is the default"
In Python/JS, everything is mutable unless you work to prevent it. In Rust, everything is immutable by default — you opt into mutation with mut. This inverts your instincts.
⑤ "Lifetimes are the compiler's problem"
You've never thought about how long a value lives — the GC handles it. In Rust, when you borrow a reference, the compiler tracks how long it's valid. You'll eventually need to annotate this explicitly. It's the steepest learning curve, but it's also why Rust is safe.

4. Common Misconceptions from Python/JS Developers
❌ "Rust is just C with better syntax"
Rust is a fundamentally different language from C. It has traits (like interfaces), generics, pattern matching, algebraic types, closures, and iterators. It feels closer to a functional language than C does.
❌ "The borrow checker is fighting me"
Early on it feels adversarial. It's not. The borrow checker is surfacing real bugs in your design — usually shared mutable state that would silently corrupt data in Python or crash with a race condition in JS. When it rejects your code, the right question is "what is my data's actual ownership structure?" not "how do I get past this?"
❌ "Clone fixes everything" (the opposite mistake)
When beginners lose a fight with the borrow checker, the instinct is to .clone() everything. That works — but it defeats Rust's performance guarantees and signals you don't understand ownership yet. Clone is a valid tool; using it as a universal escape hatch is a code smell.
❌ "Rust is too verbose for high-level work"
Rust's iterator chains, closures, and type inference are genuinely expressive. data.iter().filter(|x| x > 0).map(|x| x * 2).collect() reads cleanly. You're not writing C-style loops for everything.
❌ "Unsafe Rust means Rust isn't safe"
unsafe blocks exist for FFI (calling C libraries) and very low-level operations. They're rare, localized, and explicitly marked — unlike C where unsafety is everywhere and invisible. The bitcoin/rust-bitcoin library is almost entirely safe Rust.
❌ "I should learn Rust the same way I learned Python"
Python rewards experimentation: try it in the REPL, see what happens. Rust rewards thinking before writing. Design your data structures and ownership topology first. Fighting the compiler reactively is painful; understanding it proactively is satisfying.

Your Bitcoin-Specific Onramp
Given your goal, the ownership model matters most because Bitcoin development involves:

Parsing untrusted bytes → Rust's exhaustive pattern matching shines
Cryptographic keys → Type safety prevents key/address confusion
Transaction construction → Result propagation makes error handling explicit and auditable
Consensus-critical code → Compile-time correctness guarantees are not optional here

The crates to know: rust-bitcoin, bdk (Bitcoin Dev Kit), and lightning (LDK). All three are idiomatic, well-documented, and will teach you Rust patterns in a Bitcoin context simultaneously.

**AI Response Summary:**  
Rust's core philosophy is "If it compiles, it's probably correct." 
Key mental shifts from Python/JS: ownership over shared references, explicit error handling with `Result<T,E>` over exceptions, immutability by default, and designing
data structures before writing code. For Bitcoin specifically, Rust's type
safety prevents key/address confusion and exhaustive pattern matching is ideal
for parsing untrusted bytes.

**Helpfulness Evaluation:**  
Very helpful — the comparison table and Bitcoin-specific onramp section made
abstract Rust concepts immediately relevant to this project's goals.

## Prompt #1b — Follow-up Clarification on Prompt #1
**Date:** 21st March 2025  
**Technique:** Section 2.2 - Promoting Deep Understanding  

**Prompt Used:**
> "What is the difference between runtime and compile time? In simple terms?"


**AI Response**
Runtime vs Compile Time — Simply Put
Think of it like writing a recipe vs actually cooking.

Compile Time = Before the program runs
The compiler reads your code and translates it into something the computer can execute. This happens once, before anyone uses your program.
If the compiler finds a problem, it refuses to build — nothing runs until you fix it.

Like a recipe editor reading your recipe before it goes to print and saying: "You wrote 'add 2 cups of fridge' — that makes no sense, fix it."


Runtime = While the program is actually running
This is when real users, real data, and real conditions are in play. Errors here happen in the moment, often unexpectedly.

Like a chef following the recipe mid-service and discovering "we don't have any eggs" — the problem only surfaces when it's already a crisis.


The Python/JS vs Rust difference
python# Python — this is fine at compile time (there is no compile step)
# The crash only happens when this line actually executes
x = "hello" + 5   # TypeError at RUNTIME
rust// Rust — the compiler catches this before the program ever runs
let x = "hello" + 5;  // ERROR at COMPILE TIME — won't even build
In Python and JS you find out about many bugs when they happen to a user. In Rust you find out when you're building the program.

Why this matters for Bitcoin
Imagine a bug in wallet software that sends funds to the wrong address.

In a runtime language, that bug might only trigger on a rare combination of inputs — you find out when someone loses money.
In Rust, an entire class of those bugs can't be expressed in the first place — the compiler rejects them before the program exists.

Simple rule of thumb:

Compile time = problems caught by the compiler, on your machine, before anyone is affected
Runtime = problems that surface in the wild, while the program is running, potentially affecting real users

Rust pushes as many problems as possible from the right column to the left.

**AI Response Summary:**  
The AI used a recipe analogy — compile time is a recipe editor catching errors
before the recipe goes to print (before the program runs), runtime is a chef
discovering missing ingredients mid-service (while the program is running).
In Python/JS, `"hello" + 5` only crashes when that line executes — affecting
real users. In Rust, the same mistake is rejected by the compiler before the
program ever exists. For Bitcoin, this means an entire class of wallet bugs
simply cannot be expressed in Rust code.

**Helpfulness Evaluation:**  
Very helpful — the recipe analogy made an abstract CS concept immediately
clear. The Bitcoin-specific example (wrong address bug) connected it directly
to why this project's tech choice matters.

**Key Takeaway:**  
Rust pushes problems from runtime (user-facing) to compile time (developer-facing).
---

## Prompt #2 — Step-by-Step Breakdown
**Date:** 21st March 2026  
**Technique:** Section 1.2 - Step-by-Step Breakdown (Moringa AI Module)

**Prompt Used:**
> "I want to understand Rust's ownership and memory model before writing Bitcoin
> wallet code. Could you break down:
> 1. What ownership means and the rules around it
> 2. What borrowing is and when to use & vs &mut
> 3. What the Option and Result types are and why Bitcoin code uses them heavily
> 4. How Rust structs compare to Python classes
> Let's not write complex code yet — just focus on concepts."

**AI Full Response:**

Rust's Memory Model: The Foundation

1. Ownership

Ownership is Rust's answer to the question every language has to answer:
"Who is responsible for cleaning up this memory?"

- Python/JS answer: "The garbage collector will figure it out."
- C answer: "You, the programmer. Good luck."
- Rust answer: "Whoever owns the value. And there can only ever be one owner."

The three ownership rules — memorise these:
1. Every value has exactly one owner
2. When the owner goes out of scope, the value is dropped (memory freed)
3. Ownership can be transferred (moved), but only to one place at a time

Think of ownership like a physical object, not a pointer. If you hand your car
keys to someone, you no longer have them. You didn't copy the keys — you
transferred them.

Python brain:
  a = [1, 2, 3]
  b = a          # both a and b point to the same list
  print(a)       # totally fine

Rust brain:
  let a = vec![1, 2, 3];
  let b = a;     // ownership MOVES to b
  print(a)       // COMPILER ERROR — a no longer owns anything

Scope = lifetime of ownership:
```rust
{
    let x = 5;   // x comes into scope, owns the value 5
}        do stuff with x  // x goes out of scope — value is dropped here, automatically
```

2. Borrowing: & vs &mut

Moving ownership everywhere would be exhausting. Borrowing solves this — you let
something use a value temporarily, without giving up ownership. Think of it like
lending a book. You still own it. You just let someone read it.

& — Shared/immutable borrow (read-only):
"You can look at this, but you cannot change it. Multiple people can look
simultaneously."
- Many readers at the same time = fine
- No one can write while others are reading
- The owner keeps ownership throughout

&mut — Exclusive/mutable borrow (read + write):
"You can look at this AND change it. But you are the ONLY one with access right now."
- Only one writer at a time
- No other readers allowed while the mutable borrow exists
- Enforced by the compiler — not a convention, a rule

The borrowing rules — at any given time, you can have EITHER:
- Any number of & (immutable) borrows, OR
- Exactly one &mut (mutable) borrow
Never both simultaneously.

Bitcoin relevance: When constructing a transaction, you might pass a UTXO set to
multiple functions for reading, but only one function at a time can modify the
wallet state. The compiler enforces this automatically.

3. Option<T> and Result<T, E>

Option<T> — "This value might not exist"
Rust bans null entirely. Instead, if a value might not exist, the type itself
tells you:

```
Option<T> is either:
  Some(value)   — the value exists, here it is
  None          — the value does not exist
```

You cannot use an Option<T> as if it's definitely a T. The compiler forces you to handle both cases. No more null pointer exceptions — ever.

```python
# Python
address = get_address()   # might return None
print(address.upper())    # crashes if None — found out at runtime
```

```rust
// Rust
match get_address() {
    Some(address) => { /* use the address */ },
    None          => { /* handle the missing case */ }
}
// compiler won't let you skip the None case
```
Result<T, E> — "This might fail, and here's why"
Python/JS use exceptions. The problem with exceptions is they're invisible — a function's signature gives no hint that it might throw. You discover it by reading docs, or by crashing.
Rust puts the error in the return type:

Result<T, E> is either:
  Ok(value)    — it worked, here's the value
  Err(error)   — it failed, here's why
```
Every function that can fail says so explicitly in its signature. Every caller must consciously decide: handle the error, propagate it, or explicitly choose to panic.

```rust
match parse_bitcoin_address(input) {
    Ok(address) => { /* proceed with address */ },
    Err(e)      => { /* tell the user what went wrong */ }
}
```

Why Bitcoin code uses both heavily:
- Is this address valid? (Option — it might not parse)
- Did the transaction broadcast? (Result — the network might reject it)
- Does this UTXO exist? (Option — it might be spent)
- Did signing succeed? (Result — the key might be wrong)

In Python, all of these would be silent None returns or uncaught exceptions
waiting to surprise you. In Rust, every single one is explicit, visible, and
enforced. For code that handles people's money, that's not optional — it's the
minimum bar.

4. Structs vs Python Classes

Python classes mix data and behaviour in one blob, with shared mutable state
as the default:

```python
class Wallet:
    def __init__(self):
        self.balance = 0        # data
    def deposit(self, amount):  # behaviour
        self.balance += amount  # mutates freely
```

Anything can access and mutate self.balance at any time. There's no concept of "who owns this" or "is anyone else using this right now."

Rust structs separate data from behaviour, with immutability as the default:
```rust
// The DATA — just a shape
struct Wallet {
    balance: u64,
    address: String,
}

// The BEHAVIOUR — attached separately via impl
impl Wallet {
    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }
    fn balance(&self) -> u64 {   // & not &mut — read only
        self.balance
    }
}
```
Key differences from Python classes:

| Feature            | Python Class               | Rust Struct                       |
|--------------------|----------------------------|-----------------------------------|
| Inheritance        | Yes, deeply                | No — use composition instead      |
| Default mutability | Mutable                    | Immutable — opt into mut          |
| Null fields        | Any field can be None      | Must use Option<T> explicitly     |
| Shared behaviour   | Interfaces/ABCs (optional) | Traits (enforced)                 |
| "Self" access      | self freely                | &self (read) or &mut self (write) |


Notice that deposit takes &mut self — it's telling the compiler "this method needs to modify the struct." And balance takes &self — "this method only reads." This is declared in the function signature, not buried in the implementation.

Traits are Rust's alternative to inheritance. Rather than "Wallet extends
Account," you say "Wallet implements the Signable trait." It's composition and
interfaces — closer to Go or TypeScript interfaces than Python inheritance.

How It All Connects — These four concepts are one unified system:
- Ownership  → who is responsible for this data?
- Borrowing  → how can others temporarily access it?
- Option     → what if this data doesn't exist?
- Result     → what if producing this data fails?
- Structs    → how do I organise related data?

For Bitcoin wallet code specifically, you'll use all five constantly — parsing
addresses (Option), broadcasting transactions (Result), managing UTXOs (ownership
transfers), reading wallet state (& borrows), and updating balances (&mut borrows).
These aren't language trivia — they're the tools you'll reach for every single day.

**AI Response Summary:**
The AI broke down Rust's four foundational concepts as one unified system.
Ownership means every value has one owner and is freed when that owner goes out
of scope — like handing car keys, not copying them. 
Borrowing via & allows multiple read-only access simultaneously while &mut allows exactly one writer with no other readers — making data races impossible. 
Option<T> replaces null by forcing both Some and None to be handled at compile time. Result<T, E> replaces exceptions by putting errors in function signatures. 
Structs separate data from behaviour, with immutability as default and traits replacing inheritance.

**Helpfulness Evaluation:**
Extremely helpful — the car keys analogy for ownership and the book lending
analogy for borrowing made abstract concepts concrete. The Bitcoin-specific
examples at the end of each section showed exactly where each concept appears
in wallet code.

**Key Takeaway:**
Ownership + Borrowing + Option + Result + Structs = one unified system. For code
that handles people's money, explicit error handling and compile-time safety are
not optional — they are the minimum bar.


## Prompt #3 — Guided Implementation
**Date:** 21st March 2026  
**Technique:** Section 1.3 - Guided Implementation (Moringa AI Module)

**Prompt Used:**
> "I'm ready to implement a basic Bitcoin wallet CLI in Rust. I know Python and JS.
> Could you guide me through creating a Rust program that:
> 1. Generates a random private key
> 2. Derives a public key from it
> 3. Generates a Bitcoin address (testnet)
>
> Please explain each part of the syntax as you go, especially parts that differ
> from Python. Use the bitcoin and rand crates which I already have installed at
> versions bitcoin = "0.31" and rand = "0.8"."

**AI Full Response:**
**AI Full Response:**

// ============================================================
// Rust Bitcoin Wallet — Testnet Key & Address Generation
// ============================================================
// Crates used:
//   bitcoin = "0.31"
//   rand    = "0.8"
// ============================================================

The program is broken into three clean functions, each handling one step.
Here is the full code with every line explained:

---

IMPORTS — Rust's `use` is like Python's `import` or JS's `import { } from`

`use bitcoin::secp256k1::{Secp256k1, SecretKey};`
→ Brings in the elliptic curve context and secret key type from the bitcoin crate.

`use bitcoin::{Address, Network, PublicKey};`
→ Address = the final Bitcoin address type
→ Network = an enum to select Mainnet, Testnet, etc.
→ PublicKey = the public key type (wraps the secp256k1 key)

`use rand::rngs::OsRng;`
→ OS-level cryptographically secure random number generator.
→ Python equivalent: secrets.token_bytes(32)
→ JS equivalent: crypto.getRandomValues(new Uint8Array(32))

`use rand::RngCore;`
→ The trait that gives OsRng its fill_bytes() method.
→ Traits in Rust are like interfaces — you must import them to use their methods.


ENTRY POINT

`fn main()` is the entry point — like `if __name__ == "__main__"` in Python
or top-level code in a Node.js script.
```rust
fn main() {
    println!("=== Bitcoin Testnet Wallet Generator ===\n");

    let secret_key = generate_private_key();
    let public_key = derive_public_key(&secret_key);
    let address = derive_address(&public_key);

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
```

Key syntax notes:
- `let secret_key = ...` — immutable by default. No `const` or `var` needed.
- `&secret_key` — a borrow. We lend the key to the function without giving up
  ownership. After the call, secret_key is still usable in main.
- `.iter().map().collect()` — Rust iterator chains. Same idea as Python's
  list comprehensions or JS's array .map(). Converts bytes to hex strings.
- `format!("{:02x}", b)` — like Python's f"{b:02x}". Formats a byte as
  2-digit lowercase hex.

---

STEP 1: GENERATE A PRIVATE KEY
```rust
fn generate_private_key() -> SecretKey {
    println!("[1/3] Generating private key...");

    let mut rng = OsRng;
    let mut key_bytes = [0u8; 32];
    rng.fill_bytes(&mut key_bytes);

    SecretKey::from_slice(&key_bytes)
        .expect("Failed to create secret key — astronomically unlikely")
}
```

Key syntax notes:
- `-> SecretKey` — the return type. TypeScript would write `: SecretKey` after
  the parens. Python has no equivalent syntax.
- `let mut` — opts into mutability. Rust is immutable by default.
- `[0u8; 32]` — array of 32 unsigned 8-bit integers, all zero.
  u8 = 0 to 255. Python/JS just call these "bytes".
- `rng.fill_bytes(&mut key_bytes)` — fills the array with random bytes.
  &mut = mutable borrow. We let the function write into our array.
- `SecretKey::from_slice(...)` — `::` calls an associated function on the type.
  Like a Python @classmethod or static method. No instance needed.
- `.expect("msg")` — unwraps Result<SecretKey, Error>. If Ok → returns the value.
  If Err → crashes with the message. Safe here because invalid random bytes
  are astronomically unlikely.
- NO `return` keyword — the last expression without a semicolon IS the return
  value. This is the most common Rust syntax surprise for Python/JS developers.

---

STEP 2: DERIVE THE PUBLIC KEY
```rust
fn derive_public_key(secret_key: &SecretKey) -> PublicKey {
    println!("[2/3] Deriving public key...");

    let secp = Secp256k1::new();
    let inner = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, secret_key);

    PublicKey {
        compressed: true,
        inner,
    }
}
```

Key syntax notes:
- `secret_key: &SecretKey` — borrowed parameter. We read it; the caller keeps it.
- `Secp256k1::new()` — creates the elliptic curve context.
  Bitcoin uses secp256k1 for all key operations. Same curve Ethereum uses.
- `PublicKey::from_secret_key(&secp, secret_key)` — derives the public key.
  Mathematically: public_key = secret_key × G (generator point). One-way only.
- `PublicKey { compressed: true, inner }` — struct literal syntax.
  Like Python's dataclass(compressed=True, inner=inner).
  `inner` alone is shorthand for `inner: inner` when the name matches.
- `compressed: true` — use the 33-byte compressed format. Always use this.
  The older uncompressed format is 65 bytes and considered legacy.

---

STEP 3: DERIVE THE BITCOIN ADDRESS
```rust
fn derive_address(public_key: &PublicKey) -> Address {
    println!("[3/3] Generating testnet address...");
    Address::p2pkh(public_key, Network::Testnet)
}
```

Key syntax notes:
- `Network::Testnet` — an enum variant. Rust enums are more powerful than
  Python's — they can carry data. Here it's a simple tag with no attached data.
- `Address::p2pkh` — generates a Pay-to-Public-Key-Hash address.
  The classic Bitcoin address format.
  On testnet: starts with 'm' or 'n'
  On mainnet: starts with '1'
- Internally computes: SHA256(RIPEMD160(public_key_bytes)) then encodes
  with Base58Check. The crate handles all of this automatically.
- Again — no `return`, no semicolon. The address is returned directly.

---

ACTUAL OUTPUT (from running `cargo run`):

=== Bitcoin Testnet Wallet Generator ===

[1/3] Generating private key...
[2/3] Deriving public key...
[3/3] Generating testnet address...
Private Key (hex) : 75a67f10f65c0c02eb65fd8c3bbaacae9d1ad8d1bf35c2b5da94c8d9e2ea7ad0
Public Key        : 03e65cdce32f198b9765ba6387ddbfe85563e4f0f7cfbf531edbe70fc59131f320
Testnet Address   : mgG5DmNJQbw8uhjaWmswDi4Xp2twxE3AVe

⚠️  Never share your private key. This is testnet — no real funds.

Note: Keys and address are randomly generated each run. Format will always match.

---

What to learn next:
1. WIF encoding — encode the private key in Wallet Import Format
2. BIP-39 mnemonics — generate a 12/24-word seed phrase
3. BIP-32/44 HD derivation — derive multiple addresses from one seed
4. Replace .expect() with ? — proper error propagation using Result throughout

**AI Response Summary:**
The AI produced a fully annotated Bitcoin wallet generator in three clean
functions — generate_private_key(), derive_public_key(), and derive_address().
Key Rust syntax lessons highlighted:
- `let` vs `let mut` — immutability is the default, mutation is opt-in
- `&` borrows — passing &secret_key lends without giving up ownership
- Return without `return` — last expression without semicolon is the return value
- `::` vs `.` — type-level functions vs instance methods
- `.expect("msg")` — explicit Result unwrapping with a crash message
The program uses OsRng for cryptographically secure key generation,
secp256k1 elliptic curve math for public key derivation, and p2pkh
address format for the testnet Bitcoin address.

**Helpfulness Evaluation:**
Extremely helpful — every line had a comment explaining the Rust syntax AND
its Python/JS equivalent. The three-function structure mapped directly onto
the three learning steps, making it easy to understand what each part does
independently.

**Key Takeaway:**
Rust functions are clean units of ownership — each one declares exactly what
it borrows (&) and what it returns (->). Reading a function signature tells
you the entire ownership story without reading the body.


## Prompt #4 — Understanding Verification
**Date:** 21st March 2026  
**Technique:** Section 1.4 - Understanding Verification (Moringa AI Module)

**Prompt Used:**
> "I've written this Rust Bitcoin wallet program: [pasted full main.rs code]
> Could you:
> 1. Verify if I've followed Rust best practices?
> 2. Explain any improvements I should make?
> 3. Point out any Python/JS habits that might be showing in my Rust code?
> 4. Suggest what I should learn next for Bitcoin development in Rust?"

**AI Full Response:**

Overall Verdict: Solid First Rust

Your code compiles clean, runs correctly, uses the right crate APIs, and shows
you've genuinely absorbed the ownership concepts — borrowing with & in all the
right places, immutability by default, no unnecessary clones. For a first Rust
program this is good work. There are no bugs. The issues below are about
graduating from "correct" to "idiomatic."


Issue 1: Secp256k1 is recreated on every call — the main real problem

Your code:
```rust
fn derive_public_key(secret_key: &SecretKey) -> PublicKey {
    let secp = Secp256k1::new();  // ← created inside the function
    ...
}
```

Secp256k1::new() isn't a cheap operation — it allocates and precomputes lookup
tables used by the elliptic curve math. Creating it once per key derivation is
wasteful. In a wallet generating hundreds of addresses (HD wallets do this),
it adds up fast.

The fix — create it once in main and pass it in:
```rust
fn main() {
    let secp = Secp256k1::new();  // created once
    let mut rng = OsRng;

    let secret_key = generate_private_key(&mut rng);
    let public_key = derive_public_key(&secp, &secret_key);  // passed in
    ...
}

fn derive_public_key(
    secp: &Secp256k1<bitcoin::secp256k1::All>,
    secret_key: &SecretKey,
) -> PublicKey {
    let inner = bitcoin::secp256k1::PublicKey::from_secret_key(secp, secret_key);
    PublicKey { compressed: true, inner }
}
```

This is a Python/JS habit showing through — in those languages you don't think
about construction cost because the runtime handles it. Rust makes you think
about when things are built and how often.

---

Issue 2: .expect() everywhere — a Python/JS exception habit

Your code:
```rust
SecretKey::from_slice(&key_bytes)
    .expect("Failed to create secret key")
```

.expect() is a panic — the entire program crashes with no recovery. It's fine
for learning and for truly unrecoverable situations, but it's the Rust equivalent
of this Python:
```python
try:
    key = SecretKey.from_slice(key_bytes)
except Exception:
    raise SystemExit("Failed")  # just crash, no recovery possible
```

In production wallet code you'd use Result propagation with the ? operator:
```rust
// Functions that can fail return Result
fn generate_private_key(rng: &mut OsRng) -> Result<SecretKey, bitcoin::secp256k1::Error> {
    let mut key_bytes = [0u8; 32];
    rng.fill_bytes(&mut key_bytes);
    SecretKey::from_slice(&key_bytes)  // ? would propagate the error up
}

// main() handles it at the top level
fn main() {
    match generate_private_key(&mut rng) {
        Ok(key)  => { /* use it */ }
        Err(e)   => { eprintln!("Key generation failed: {}", e); }
    }
}
```

The ? operator is the idiomatic way to propagate errors — it's the next
significant concept to learn after ownership.

---

Issue 3: rng is created inside the function that doesn't need to own it

Your code:
```rust
fn generate_private_key() -> SecretKey {
    let mut rng = OsRng;  // ← created here, thrown away when function ends
    ...
}
```

OsRng is a handle to the OS random number generator. Creating and discarding it
per call isn't catastrophic (it's lightweight), but it signals the same habit as
Issue 1 — creating resources inside functions that could be shared. The idiomatic
approach passes it in:
```rust
fn generate_private_key(rng: &mut OsRng) -> SecretKey {
    ...
}
```

This also makes the function easier to test — in tests you can swap in a
deterministic RNG instead of the real OS one.

---

Issue 4: The hex conversion — correct but not the most idiomatic

Your code:
```rust
let private_key_hex: String = secret_key
    .secret_bytes()
    .iter()
    .map(|b| format!("{:02x}", b))
    .collect();
```

This works and is readable. The more idiomatic Rust would use the hex crate or
the bitcoin crate's built-in hex display — but since you don't have those as
dependencies yet, your version is perfectly reasonable. It's not a habit to
break, just something to know exists.


What You Did Well — specifically

These are things beginners consistently get wrong, and you got right:

- Borrowing correctly in main: Passing &secret_key and &public_key rather than
  moving them. You clearly understood why — you needed to use them after the
  function calls.
- No unnecessary .clone(): A common beginner reflex when fighting the borrow
  checker is to clone everything. You didn't. Clean.
- Immutability by default: You only used let mut where actually needed (rng,
  key_bytes). Every other binding is correctly immutable.
- Correct compressed key format: PublicKey { compressed: true, ... } — you knew
  to set this. Uncompressed keys are a legacy footgun.
- OsRng not thread_rng: Using the cryptographically secure RNG for key material
  rather than the faster-but-not-crypto-safe alternative. This matters enormously
  for Bitcoin.

---

The Improved Version:
```rust
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
```

The changes are minimal: secp and rng created in main and passed in.
Everything else stays the same.

---

What to Learn Next, in Order:

1. The ? operator and Result propagation — your immediate next step. Replace
   .expect() calls with proper error handling. This is the single biggest gap
   between your current code and production-quality Rust.

2. BIP-39 mnemonics — add the bip39 crate. Instead of a raw 32-byte private key,
   generate a 12-word seed phrase. This is how every real wallet works and it'll
   teach you more about Rust's type system through a concrete Bitcoin concept.

3. BIP-32 HD key derivation — use Xpriv and DerivationPath (already in the
   bitcoin crate). Derive multiple addresses from one seed using paths like
   m/44'/0'/0'/0/0. This is what makes a wallet a wallet rather than just a
   key generator.

4. Structs with impl — wrap your current functions into a Wallet struct with
   methods. This is where Rust's struct/trait system becomes real and you'll
   see how it differs from Python classes.

5. Writing tests — Rust has first-class test support built into the language
   with #[test]. For Bitcoin code, tests are essential. You'll learn how to
   pass a deterministic RNG in tests to get reproducible keys.

The path from here to a real HD wallet is shorter than it looks — you've already
understood the hardest conceptual parts.

**AI Response Summary:**
The AI gave the code an overall verdict of "Solid First Rust" — compiles clean,
correct ownership and borrowing throughout, no unnecessary clones, right RNG
choice. Three improvements were identified:

1. Secp256k1::new() was being recreated inside derive_public_key() on every
   call — wasteful because it precomputes elliptic curve lookup tables. Fix:
   create it once in main() and pass it in as &secp. This was identified as
   a Python/JS habit — those languages hide construction costs behind the runtime.

2. .expect() everywhere is the Rust equivalent of crashing with no recovery —
   fine for learning but not for production. The next step is replacing it with
   the ? operator and proper Result propagation throughout the call chain.

3. OsRng was being created inside generate_private_key() and thrown away —
   same pattern as Issue 1. Fix: create it in main() and pass as &mut rng.
   This also makes the function easier to test with a deterministic RNG.

What the AI confirmed was done correctly:
- Borrowing with & in all the right places
- No unnecessary .clone() calls
- let mut only where actually needed
- compressed: true on the public key
- OsRng instead of thread_rng for cryptographic key material

**Helpfulness Evaluation:**
Extremely helpful — the AI didn't just say "good job." It identified real
performance and idiomatic issues, explained WHY they are issues, connected
them back to Python/JS habits, and gave a concrete learning roadmap. The
distinction between "correct" and "idiomatic" Rust is exactly the gap a
beginner needs to understand.

**Key Takeaway:**
Correct Rust and idiomatic Rust are different things. The three issues found
— Secp256k1 recreation, .expect() overuse, and rng scoping — all trace back
to the same Python/JS habit: not thinking about when and how often resources
are created. Rust makes you think about this explicitly.

---

## Prompt #5 — Learning Through Teaching
**Date:** 21st March 2026  
**Technique:** Section 2.3 - Learning Through Teaching (Moringa AI Module)

**Prompt Used:**
> "I've just built a Bitcoin testnet wallet CLI in Rust as a beginner coming
> from Python and JavaScript. Let me explain what I learned and you tell me
> what I'm getting right, what I'm missing, and what I'm misunderstanding:
>
> Here's how I would explain Rust to another Python/JS developer:
>
> Rust is a systems programming language that catches bugs at compile time
> instead of runtime. The most important concept is ownership — every value
> has exactly one owner, and when that owner goes out of scope the memory is
> freed automatically, no garbage collector needed. You can temporarily lend
> values using borrowing — & for read-only access and &mut for write access,
> but never both at the same time. Instead of null, Rust uses Option<T> with
> Some and None, and instead of exceptions it uses Result<T, E> with Ok and
> Err — both force you to handle every case explicitly. Structs replace
> classes, and traits replace inheritance. For Bitcoin specifically, these
> features matter because wallet bugs aren't just crashes — they're financial
> losses, and Rust makes an entire class of those bugs impossible to write.
>I've also learnt that writing idiomatic code is important ,for example, I could have used Secp256k1 in my main function so it can be shared in the whole code.
> What am I getting right? What am I missing or misunderstanding? What would
> you add or correct?"

**AI Full Response:**

This is a strong summary. Let me go through it piece by piece — what's solid,
what needs a small correction, and what's missing.

What You're Getting Right:

- The ownership explanation is accurate and well-framed. "Every value has
  exactly one owner, and when that owner goes out of scope the memory is freed
  automatically" is the correct mental model, stated cleanly.
- The borrowing rules are correct. & for read-only, &mut for write access,
  never both at the same time. That's the rule. You've absorbed it properly.
- Option and Result framing is exactly right. Calling them things that "force
  you to handle every case explicitly" is the key insight — it's not just that
  they exist, it's that the compiler won't let you ignore them.
- The Bitcoin motivation is the right reason. "Wallet bugs aren't just crashes
  — they're financial losses" is precisely why rust-bitcoin, BDK, and LDK are
  all written in Rust.
- The Secp256k1 insight is real idiomatic thinking. Recognising that creating
  it once and passing it around is better than recreating it in every function
  — that's not a beginner observation. That's thinking about resource cost and
  API design, which is exactly the Rust mindset.

One Thing to Correct:

"Traits replace inheritance" is about 70% right — but the part that's wrong
will trip you up later. Traits don't replace inheritance entirely. They replace
a specific use of inheritance — the part where you say "this type can do X."
But Python inheritance does two things at once that Rust deliberately separates:

- Sharing behaviour → traits handle this. If multiple types can all sign() a
  transaction, you define a Signer trait and implement it for each type.
- Sharing data/implementation → Rust has no mechanism for this. You instead
  use composition — put the shared data inside your struct as a field.
```python
# Python — inherit data AND behaviour
class HDWallet(BaseWallet):
    pass  # gets all of BaseWallet's fields and methods for free
```
```rust
// Rust — compose data, share behaviour through traits
struct HDWallet {
    base: BaseWallet,  // composition: contain it, don't inherit it
}

impl Signer for HDWallet {  // trait: declare what it can do
    fn sign(&self, tx: &Transaction) -> Signature { ... }
}
```

The practical consequence: when building a Wallet struct, don't reach for
"what would I inherit here?" Reach for "what would I contain here, and what
traits should this implement?"

What's Missing:

1. The ? operator — you understand Result exists, but the mechanism for working
   with it in practice is the ? operator. It propagates errors up the call stack
   without writing match on every single line. This is your most immediate next
   concept.

2. Lifetimes — you've learned that borrowing is temporary, but you haven't yet
   encountered situations where the compiler needs you to explain how long a
   borrow lasts. When a function takes two references as input and returns a
   reference, the compiler asks: which input does the output borrow from? That's
   where lifetime annotations like 'a appear.

3. The compiler is the tool, not the enemy — in Python/JS, errors are things
   to get past. In Rust, when the borrow checker rejects your code, the correct
   response is "what is it seeing that I'm not?" The checker has never been
   wrong about a genuine issue. When it fights you, your design has a problem,
   not your syntax.

4. Zero-cost abstractions — Rust is not just safe, it's also fast. Iterators,
   closures, generics, traits — they all compile down to the same machine code
   as hand-written C. You can write expressive high-level code and pay nothing
   for it at runtime.

The Corrected Full Explanation:

Rust is a systems programming language that catches bugs at compile time instead
of runtime. The most important concept is ownership — every value has exactly one
owner, and when that owner goes out of scope, memory is freed automatically with
no garbage collector needed. You can temporarily lend values using borrowing — &
for read-only access and &mut for write access — but you can never have both at
the same time. This makes data races impossible to write, not just unlikely.

Instead of null, Rust uses Option<T> with Some and None. Instead of exceptions,
it uses Result<T, E> with Ok and Err. Both force you to handle every case
explicitly — the compiler won't let you ignore a potential failure. The ?
operator is how you propagate errors up the call stack cleanly without writing
a match on every line.

Structs replace classes. Traits let you define shared behaviour across types —
similar to interfaces in TypeScript. But unlike Python inheritance, traits don't
share data or implementation. When you want to share data, you use composition:
put one struct inside another as a field, rather than inheriting from it.

Idiomatic Rust also means thinking about when you create things. Expensive
resources like a Secp256k1 context should be created once and passed around by
reference, not recreated inside every function that needs them.

For Bitcoin specifically, these features matter because wallet bugs aren't just
crashes — they're financial losses. Rust makes an entire class of those bugs
impossible to write, and its performance means you pay no runtime cost for that
safety.

Where You Are Overall:

You've correctly understood the three hardest conceptual things about Rust —
ownership, borrowing, and why explicit error handling matters. Most people take
weeks to genuinely absorb those. The gaps you have are things you haven't needed
yet, not things you've misunderstood. The ? operator is your next concrete thing
to learn. After that, try wrapping your wallet functions in a Wallet struct with
impl methods.

**AI Response Summary:**
The AI confirmed the core explanation was accurate — ownership, borrowing rules,
Option/Result framing, and Bitcoin motivation were all correct. One correction:
"traits replace inheritance" is only 70% right. Traits handle shared behaviour
but not shared data — Rust uses composition for that instead. Missing concepts
identified: the ? operator for clean error propagation, lifetimes for explaining
borrow duration, treating the compiler as a tool not an enemy, and zero-cost
abstractions meaning safety has no runtime performance cost.

**Helpfulness Evaluation:**
The most valuable prompt of the project — having the AI verify my own explanation
revealed exactly where my understanding was solid vs where it was approximate.
The traits vs inheritance correction is a subtle but important distinction that
would have caused confusion when building more complex structs.

**Key Takeaway:**
I correctly understood the three hardest parts of Rust — ownership, borrowing,
and explicit error handling. The gaps are things I haven't needed yet, not things
I've misunderstood. Next step: the ? operator, then wrapping wallet functions
in a proper Wallet struct.

## Prompt #6 — Test Improvement Guidance
**Date:** 21st March 2026  
**Technique:** Section 2 - Improving a Single Test (Moringa AI Testing Module)

**Prompt Used:**
> "I wrote this test for my Rust Bitcoin wallet program:
>
> Function:
> ```rust
> fn generate_private_key(rng: &mut OsRng) -> SecretKey {
>     let mut key_bytes = [0u8; 32];
>     rng.fill_bytes(&mut key_bytes);
>     SecretKey::from_slice(&key_bytes)
>         .expect("Failed to create secret key")
> }
> ```
>
> My test:
> ```rust
> #[test]
> fn test_private_key_generation() {
>     let mut rng = OsRng;
>     let key = generate_private_key(&mut rng);
>     assert_eq!(key.secret_bytes().len(), 32);
> }
> ```
>
> Instead of rewriting it for me, please:
> 1. Ask me questions about what my test is trying to verify
> 2. Help me identify if my test is checking behaviour or implementation details
> 3. Suggest how I could make the test's purpose clearer
> 4. Ask me what edge cases my test might be missing
> 5. Guide me in improving my assertions to be more precise"

**AI Full Response:**
[paste AI response here when you run it]

**AI Response Summary:**
Started with one basic test checking private key byte length. Through guided
questions from the AI, identified three additional behaviours worth testing:
- Two generated keys should never be identical (randomness verification)
- Testnet addresses must always start with 'm' or 'n' (format verification)
- Same private key always produces same public key (determinism verification)

**Helpfulness Evaluation:**
The guided approach from the testing module worked well — rather than having
AI write the tests, the questions helped identify what was actually worth
testing. The distinction between testing behaviour vs implementation details
was the most valuable insight.

**Key Takeaway:**
A good test suite covers: correctness (does it produce valid output?),
randomness (are two outputs different?), format (is the output the right
shape?), and determinism (does the same input always give the same output?).

**Errors encountered during testing:**
- Initial test showed "0 tests" despite correct code — file was not being
  saved properly by VS Code. Fixed by writing directly to file via terminal
  using `cat > src/main.rs`.

  ## Prompt #7 — Comprehensive Function Documentation
**Date:** 25th March 2026
**Technique:** Code Documentation Exercise — Prompt 1

**Prompt Used:**
> "Please create comprehensive documentation for these Rust functions following
> Rust doc comment conventions (///):
> [pasted generate_private_key, derive_public_key, derive_address]
>
> The documentation should include:
> 1. A clear description of what each function does
> 2. All parameters with types and descriptions
> 3. Return value with type and description
> 4. Any panics or errors that might be thrown
> 5. Example usage
> 6. Any important notes or edge cases developers should be aware of"

**AI Full Response:**

```rust
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Address, Network, PublicKey};
use rand::rngs::OsRng;
use rand::RngCore;

/// Generates a cryptographically secure random private key suitable for use
/// in a Bitcoin wallet.
///
/// Fills a 32-byte buffer with random data from the OS entropy source, then
/// constructs a secp256k1 [`SecretKey`] from those bytes. The key is validated
/// against the secp256k1 curve order on construction.
///
/// # Parameters
///
/// - `rng`: A mutable reference to an [`OsRng`] instance. The caller owns the
///   RNG and passes it in so that a single entropy source can be reused across
///   multiple key generation calls without the overhead of re-initialising the
///   OS interface each time.
///
/// # Returns
///
/// A [`SecretKey`] containing 32 bytes of cryptographically random key material,
/// guaranteed valid on the secp256k1 curve.
///
/// # Panics
///
/// Panics if the randomly generated bytes do not form a valid secp256k1 private
/// key. A valid key must be a non-zero integer strictly less than the curve
/// order `n`:
///
/// ```text
/// n = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
/// ```
///
/// The probability of a random 32-byte value being invalid is approximately
/// 1 in 2^128 — effectively impossible in practice. The panic exists to satisfy
/// the type system, not to handle a realistic failure mode.
///
/// # Security
///
/// - Always use [`OsRng`] for key material. Never substitute `thread_rng()` or
///   any non-cryptographic RNG — doing so produces predictable keys.
/// - The raw key bytes exist briefly on the stack inside this function and are
///   not explicitly zeroed after use. For high-security applications, consider
///   using the `zeroize` crate to clear key material from memory after the
///   `SecretKey` is constructed.
/// - Never log, display, or serialise the returned `SecretKey` in production.
///
/// # Example
///
/// ```rust
/// use rand::rngs::OsRng;
///
/// let mut rng = OsRng;
/// let secret_key = generate_private_key(&mut rng);
/// // secret_key is now ready for public key derivation
/// ```
fn generate_private_key(rng: &mut OsRng) -> SecretKey {
    let mut key_bytes = [0u8; 32];
    rng.fill_bytes(&mut key_bytes);
    SecretKey::from_slice(&key_bytes)
        .expect("Failed to create secret key")
}

/// Derives a compressed secp256k1 public key from a private key.
///
/// Performs elliptic curve scalar multiplication: `public_key = secret_key × G`,
/// where `G` is the secp256k1 generator point. This operation is one-way — it
/// is computationally infeasible to reverse it to recover the private key.
///
/// The returned key is always in **compressed** format (33 bytes: a 1-byte
/// parity prefix followed by the 32-byte x-coordinate). Compressed keys are
/// the standard for all modern Bitcoin usage and produce shorter, cheaper
/// transactions than the legacy 65-byte uncompressed format.
///
/// # Parameters
///
/// - `secp`: A reference to an initialised [`Secp256k1`] context. This context
///   holds precomputed tables used by the elliptic curve operations and is
///   moderately expensive to create. Pass it in rather than constructing it
///   inside this function so that a single context can be shared across multiple
///   derivations.
///
/// - `secret_key`: A reference to the [`SecretKey`] to derive from. Borrowing
///   rather than taking ownership ensures the caller can continue using the
///   secret key after derivation — for example, to export it in WIF format.
///
/// # Returns
///
/// A [`PublicKey`] with `compressed` set to `true`. This wraps the raw
/// secp256k1 public key in the `bitcoin` crate's type, which carries the
/// compression flag used by address derivation and transaction signing.
///
/// # Example
///
/// ```rust
/// use bitcoin::secp256k1::Secp256k1;
/// use rand::rngs::OsRng;
///
/// let secp = Secp256k1::new();
/// let mut rng = OsRng;
///
/// let secret_key = generate_private_key(&mut rng);
/// let public_key = derive_public_key(&secp, &secret_key);
/// // secret_key is still accessible here — we only borrowed it
/// ```
///
/// # Notes
///
/// - The `Secp256k1<All>` type parameter means the context supports both
///   signing and verification operations. This is the standard choice when you
///   need general-purpose key operations.
/// - Public keys are safe to display, log, and share. They do not expose the
///   private key.
fn derive_public_key(
    secp: &Secp256k1<bitcoin::secp256k1::All>,
    secret_key: &SecretKey,
) -> PublicKey {
    let inner = bitcoin::secp256k1::PublicKey::from_secret_key(secp, secret_key);
    PublicKey { compressed: true, inner }
}

/// Derives a P2PKH (Pay-to-Public-Key-Hash) Bitcoin address for testnet from
/// a compressed public key.
///
/// P2PKH is the original Bitcoin address format, introduced in the first
/// version of Bitcoin. It encodes a hash of the public key rather than the
/// public key itself, providing a layer of indirection and a shorter on-chain
/// footprint.
///
/// # Address derivation
///
/// Internally, the `bitcoin` crate performs the following steps:
///
/// 1. SHA-256 hash the 33-byte compressed public key
/// 2. RIPEMD-160 hash the result → 20-byte public key hash
/// 3. Prepend the testnet version byte (`0x6F`)
/// 4. Compute a 4-byte checksum (double SHA-256, first 4 bytes)
/// 5. Base58Check encode the versioned hash and checksum
///
/// # Parameters
///
/// - `public_key`: A reference to the [`PublicKey`] to derive the address from.
///   Must have `compressed` set to `true`. Using an uncompressed key produces a
///   different, legacy address that modern wallets and tooling do not recognise.
///
/// # Returns
///
/// A [`bitcoin::Address`] for [`Network::Testnet`]. Testnet P2PKH addresses
/// always begin with `m` or `n` and are only valid on the Bitcoin testnet
/// network. Sending real (mainnet) funds to a testnet address will result in
/// permanent, unrecoverable loss.
///
/// # Network safety
///
/// This function is hardcoded to testnet. To generate a mainnet address,
/// change `Network::Testnet` to `Network::Bitcoin`. Consider making the network
/// a parameter if your application needs to support both:
///
/// ```rust
/// fn derive_address(public_key: &PublicKey, network: Network) -> Address {
///     Address::p2pkh(public_key, network)
/// }
/// ```
///
/// # Address formats
///
/// | Format  | Prefix      | Version byte | When to use                  |
/// |---------|-------------|--------------|------------------------------|
/// | P2PKH   | `m` / `n`   | `0x6F`       | Broad compatibility (legacy) |
/// | P2SH    | `2`         | `0xC4`       | Multisig, wrapped SegWit     |
/// | Bech32  | `tb1q`      | n/a          | Native SegWit (lower fees)   |
///
/// For new wallets, P2WPKH (native SegWit via `Address::p2wpkh`) is preferred
/// over P2PKH due to lower transaction fees and the ability to detect address
/// reuse errors at the protocol level.
///
/// # Example
///
/// ```rust
/// use bitcoin::secp256k1::Secp256k1;
/// use rand::rngs::OsRng;
///
/// let secp = Secp256k1::new();
/// let mut rng = OsRng;
///
/// let secret_key = generate_private_key(&mut rng);
/// let public_key = derive_public_key(&secp, &secret_key);
/// let address    = derive_address(&public_key);
///
/// // Testnet P2PKH addresses always start with 'm' or 'n'
/// let addr_str = address.to_string();
/// assert!(addr_str.starts_with('m') || addr_str.starts_with('n'));
/// ```
fn derive_address(public_key: &PublicKey) -> Address {
    Address::p2pkh(public_key, Network::Testnet)
}
```

**AI Response Summary:**
Produced formal /// doc comments with complete structured sections for all
three functions. Highlights: exact curve order n in hex with probability
calculation (~1 in 2^128), three security rules for generate_private_key
(OsRng only, zeroize crate, never log), address format comparison table
covering P2PKH/P2SH/P2WPKH with version bytes, and runnable examples with
assert! that cargo test will verify automatically.

**Helpfulness Evaluation:**
Extremely helpful for the API surface — params, returns, panics, and security
rules were all accurate and production-quality. Weak on inline logic
explanations inside the function bodies — that required Prompt 2.

**Key Takeaway:**
Prompt 1 documents the CONTRACT — what each function promises to its callers.
Formal /// sections are best for API reference that other developers will read
in generated HTML docs via `cargo doc`.

---

## Prompt #8 — Intent and Logic Explanation
**Date:** 25th March 2026
**Technique:** Code Documentation Exercise — Prompt 2

**Prompt Used:**
> "I need help documenting the intent and logic behind this Rust Bitcoin wallet
> code. Please:
> [pasted generate_private_key, derive_public_key, derive_address]
>
> 1. Explain what this code is trying to accomplish at a high level
> 2. Break down the logic step-by-step
> 3. Identify any assumptions or edge cases in the implementation
> 4. Suggest inline comments for complex parts
> 5. Note any potential improvements while maintaining the original functionality"

**AI Full Response:**

```rust
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Address, Network, PublicKey};
use rand::rngs::OsRng;
use rand::RngCore;

// =============================================================================
// WHAT THIS CODE DOES (high level)
// =============================================================================
// These three functions implement the core of a Bitcoin key generation pipeline:
//
//   random bytes  →  private key  →  public key  →  Bitcoin address
//
// Each step is a one-way transformation — you can move forward through the
// chain but never backward. Knowing the address tells you nothing about the
// public key. Knowing the public key tells you nothing about the private key.
// This asymmetry is the foundation of Bitcoin's security model.
//
// The pipeline is split into three functions (rather than one) because each
// step has a different security profile, different inputs, and may be reused
// independently — for example, deriving multiple addresses from one key.
// =============================================================================

/// Generates a cryptographically secure Bitcoin private key.
///
/// A private key is a random 256-bit integer in the range [1, n-1], where n
/// is the order of the secp256k1 elliptic curve:
///   n = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
///
/// The entire security of a Bitcoin wallet depends on this number being both
/// random and secret. If it is predictable, the wallet can be stolen. If it
/// is lost, the funds are gone forever.
fn generate_private_key(rng: &mut OsRng) -> SecretKey {
    // Allocate a 32-byte (256-bit) buffer on the stack, zeroed initially.
    // Stack allocation is intentional — heap allocation (Vec) would leave
    // memory at a less predictable address and is unnecessary for a fixed size.
    let mut key_bytes = [0u8; 32];

    // Fill the buffer with entropy from the OS.
    //
    // OsRng calls the platform's cryptographic entropy source:
    //   Linux/macOS : getrandom() syscall
    //   Windows     : BCryptGenRandom
    //
    // This is the only acceptable source for key material. Do not substitute
    // thread_rng() — it is seeded from the OS but designed for simulation,
    // not cryptography, and its output is potentially predictable.
    rng.fill_bytes(&mut key_bytes);

    // Validate that the random bytes form a legal secp256k1 private key.
    //
    // Two conditions would make a key invalid:
    //   1. All bytes are zero (the integer 0 is not a valid private key)
    //   2. The integer value is >= n (outside the curve's field)
    //
    // Both conditions occur with probability ~1 in 2^128 — less likely than
    // cosmic ray corruption of your hardware. The .expect() panic exists to
    // satisfy the type system, not to handle a realistic failure.
    //
    // NOTE: key_bytes sits on the stack until this function returns. It is
    // not explicitly zeroed after use. For high-security applications, the
    // `zeroize` crate can clear sensitive stack memory before it is reclaimed.
    SecretKey::from_slice(&key_bytes)
        .expect("Failed to create secret key")
}

/// Derives a compressed public key from a private key using elliptic curve
/// multiplication.
///
/// The operation is: P = k × G
///   k = private key (a 256-bit integer)
///   G = the secp256k1 generator point (a fixed constant)
///   P = public key (a point on the curve, with x and y coordinates)
///
/// This is a one-way function. Given P and G, recovering k requires solving
/// the elliptic curve discrete logarithm problem — computationally infeasible
/// with current and foreseeable technology.
fn derive_public_key(
    secp: &Secp256k1<bitcoin::secp256k1::All>,
    secret_key: &SecretKey,
) -> PublicKey {
    // Perform the elliptic curve point multiplication: public_key = secret_key × G.
    //
    // The secp context is passed in (not created here) because Secp256k1::new()
    // precomputes lookup tables and allocates memory — it is too expensive to
    // recreate per call in a wallet that derives many keys. One context, shared
    // across all operations, is the correct pattern.
    //
    // This operation cannot fail: every valid private key has exactly one
    // corresponding public key. No error handling is needed.
    let inner = bitcoin::secp256k1::PublicKey::from_secret_key(secp, secret_key);

    // Wrap the raw curve point in bitcoin::PublicKey, which carries a
    // `compressed` flag used by later operations (address derivation,
    // transaction signing).
    //
    // Compressed format (33 bytes): 1-byte parity prefix + 32-byte x coordinate
    //   Prefix 0x02 = y coordinate is even
    //   Prefix 0x03 = y coordinate is odd
    //   (Only x is stored; y can be recomputed from x and the parity bit)
    //
    // Uncompressed format (65 bytes): 0x04 + 32-byte x + 32-byte y  [legacy]
    //
    // Always use compressed = true. Uncompressed keys produce different
    // addresses, larger transactions, and are not accepted by modern wallets.
    PublicKey { compressed: true, inner }
}

/// Derives a P2PKH (Pay-to-Public-Key-Hash) Bitcoin address for testnet.
///
/// P2PKH is Bitcoin's original address format (BIP-13, 2011). Rather than
/// encoding the public key directly, it encodes a hash of the public key —
/// producing shorter addresses and keeping the public key hidden until the
/// first spend (a mild form of quantum resistance).
///
/// The derivation chain performed internally by the bitcoin crate:
///   1. SHA-256 hash the 33-byte compressed public key
///   2. RIPEMD-160 hash the result → 20-byte public key hash
///   3. Prepend testnet version byte: 0x6F
///   4. Checksum: first 4 bytes of SHA-256(SHA-256(versioned_hash))
///   5. Base58Check encode (versioned_hash + checksum)
///
/// Testnet P2PKH addresses always begin with 'm' or 'n'.
/// Mainnet P2PKH addresses always begin with '1'.
fn derive_address(public_key: &PublicKey) -> Address {
    // ASSUMPTION: public_key.compressed == true.
    //
    // Compressed and uncompressed variants of the same key produce DIFFERENT
    // addresses. This function and derive_public_key() are consistent —
    // derive_public_key() always sets compressed: true — but there is no
    // type-level enforcement of this contract. If you pass a PublicKey
    // constructed elsewhere with compressed: false, you will get a valid but
    // different address, with no error or warning.
    //
    // A future improvement would use a `CompressedPublicKey` newtype that
    // makes the compression guarantee part of the type system itself.

    // HARDCODED NETWORK: This always produces a testnet address.
    //
    // Sending mainnet (real) funds to a testnet address is an irreversible
    // loss — the addresses look similar and no tool will warn you.
    //
    // To support both networks, accept `network: Network` as a parameter:
    //   fn derive_address(public_key: &PublicKey, network: Network) -> Address
    //
    // NOTE ON ADDRESS FORMAT: P2PKH is chosen here for simplicity and broad
    // compatibility. For new wallets, P2WPKH (native SegWit) is preferred:
    //   Address::p2wpkh(public_key, network)  // starts with tb1q on testnet
    // SegWit addresses produce ~30% cheaper transactions and are standard
    // in all wallets built after 2018.
    Address::p2pkh(public_key, Network::Testnet)
}
```

**AI Response Summary:**
Produced a top-level pipeline comment showing the full system shape:
OsRng → generate_private_key → derive_public_key → derive_address, with the
key insight that each arrow is one-way. Added inline // comments naming the
actual OS syscalls (getrandom on Linux/macOS, BCryptGenRandom on Windows),
explained P = k × G with all symbols defined, identified the compressed key
implicit contract as medium-risk assumption, and provided concrete improvement
code snippets including CompressedPublicKey newtype and network parameter.

**Helpfulness Evaluation:**
Most valuable for understanding WHY each line exists. The assumption risk
identification was the most unique contribution — the compressed key contract
between derive_public_key and derive_address had no compiler enforcement and
writing it down made the design gap immediately obvious.

**Key Takeaway:**
Prompt 2 documents the REASONING — why each line exists. The two-prompt
approach revealed that contract and reasoning are genuinely different things.
Neither prompt alone produces complete documentation.

---

## Prompt #9 — Combine Best of Both Versions
**Date:** 25th March 2026
**Technique:** Code Documentation Exercise — Combine Prompt

**Prompt Used:**
> "I have two versions of documentation for the same Rust Bitcoin wallet code.
> Please combine the best elements of both into one final version:
>
> Version 1 strengths: formal /// doc comments, complete parameter descriptions,
> security warnings, address format comparison table, example usage with assertions.
>
> Version 2 strengths: top-level pipeline comment showing the full flow
> (random bytes → private key → public key → address), inline // comments
> explaining each line of logic, explicit assumption risk table, improvement
> suggestions with exact code.
>
> The final version should:
> 1. Keep the top-level pipeline comment from Version 2
> 2. Keep the formal /// doc comments from Version 1 for each function
> 3. Add the inline // comments from Version 2 inside each function body
> 4. Keep the address format table from Version 1
> 5. Keep the improvement suggestions from Version 2
> 6. Be production-quality Rust documentation a senior developer would be proud of"

**AI Full Response:**

```rust
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Address, Network, PublicKey};
use rand::rngs::OsRng;
use rand::RngCore;

// =============================================================================
// Bitcoin Key Generation Pipeline
// =============================================================================
//
//   OsRng  ──►  generate_private_key  ──►  derive_public_key  ──►  derive_address
//   (entropy)      (SecretKey)               (PublicKey)            (Address)
//
// Each arrow is a one-way cryptographic transformation:
//   - Address  reveals nothing about the public key
//   - PublicKey reveals nothing about the private key
//
// This asymmetry is the foundation of Bitcoin's security model. Ownership of
// funds is proven by knowledge of the private key alone, while the address
// (and eventually the public key) can be freely shared.
//
// The pipeline is split into three functions because each step has a distinct
// security profile and may be reused independently. In an HD wallet, for
// example, derive_public_key and derive_address are called hundreds of times
// against keys derived from a single root secret — never regenerating the
// entropy or reconstructing the secp256k1 context each time.
// =============================================================================

/// Generates a cryptographically secure random private key suitable for use
/// in a Bitcoin wallet.
///
/// Fills a 32-byte buffer with entropy from the OS, then constructs and
/// validates a [`SecretKey`] from those bytes. A valid secp256k1 private key
/// is a non-zero 256-bit integer strictly less than the curve order `n`.
///
/// The entire security of the resulting wallet depends on this value being
/// both unpredictable and secret. A predictable key can be stolen; a lost
/// key means permanent loss of funds.
///
/// # Parameters
///
/// - `rng`: A mutable reference to an [`OsRng`] instance. Passed in rather
///   than constructed internally so a single entropy source can be shared
///   across multiple calls without re-initialising the OS interface each time.
///
/// # Returns
///
/// A [`SecretKey`] containing 32 bytes of cryptographically random key
/// material, guaranteed valid on the secp256k1 curve.
///
/// # Panics
///
/// Panics if the randomly generated bytes do not form a valid secp256k1 key.
/// A valid key must satisfy:
///
/// ```text
/// 0 < key < n
/// n = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
/// ```
///
/// The probability of a random 32-byte value being invalid is approximately
/// 1 in 2^128 — less likely than cosmic ray corruption of your hardware
/// during the same operation. The panic exists to satisfy the type system,
/// not to handle a realistic failure mode.
///
/// # Security
///
/// - **Always use [`OsRng`]** for key material. Never substitute `thread_rng()`
///   or any non-cryptographic RNG — doing so produces predictable keys.
/// - **Key bytes are not zeroed.** The raw 32-byte buffer lives briefly on the
///   stack and is dropped without explicit zeroing. For hardware wallets or
///   high-security applications, add the `zeroize` crate and call
///   `key_bytes.zeroize()` before the function returns.
/// - **Never log or serialise** the returned [`SecretKey`] in production.
///
/// # Example
///
/// ```rust
/// use rand::rngs::OsRng;
///
/// let mut rng = OsRng;
/// let secret_key = generate_private_key(&mut rng);
/// // Ready for derive_public_key — do not log or display secret_key
/// ```
fn generate_private_key(rng: &mut OsRng) -> SecretKey {
    // Allocate a 32-byte (256-bit) zeroed buffer on the stack.
    // Stack allocation is intentional: heap allocation (Vec) is unnecessary
    // for a fixed-size value and leaves key material at a less predictable
    // memory address.
    let mut key_bytes = [0u8; 32];

    // Fill the buffer with entropy from the OS cryptographic source:
    //   Linux / macOS  →  getrandom(2) syscall
    //   Windows        →  BCryptGenRandom
    //
    // Do not substitute thread_rng() here. It is seeded from the OS but is
    // designed for simulation workloads, not cryptography — its internal state
    // is potentially observable and its output may be predictable.
    rng.fill_bytes(&mut key_bytes);

    // Validate that the bytes represent a legal secp256k1 scalar.
    // Invalid conditions (both astronomically rare):
    //   1. All bytes are zero — the scalar 0 is not a valid private key
    //   2. Value >= n — outside the finite field of the curve
    //
    // .expect() is appropriate here: the probability of failure is ~1 in 2^128.
    // In production code requiring retry logic, replace with a loop that calls
    // fill_bytes again on Err rather than panicking.
    SecretKey::from_slice(&key_bytes)
        .expect("Failed to create secret key — random bytes were not a valid secp256k1 scalar")
}

/// Derives a compressed secp256k1 public key from a private key.
///
/// Performs elliptic curve scalar multiplication:
///
/// ```text
/// P = k × G
///
///   k  =  private key (256-bit integer, the scalar)
///   G  =  secp256k1 generator point (a fixed curve constant)
///   P  =  public key (a point on the curve defined by x and y coordinates)
/// ```
///
/// This is a one-way function. Recovering `k` from `P` and `G` requires
/// solving the elliptic curve discrete logarithm problem (ECDLP), which is
/// computationally infeasible with current and foreseeable classical hardware.
///
/// The returned key is always in **compressed** format: a 1-byte parity prefix
/// followed by the 32-byte x-coordinate (33 bytes total). This is the standard
/// for all modern Bitcoin usage and produces smaller, cheaper transactions than
/// the legacy 65-byte uncompressed format.
///
/// # Parameters
///
/// - `secp`: A reference to an initialised [`Secp256k1`] context. Building
///   this context precomputes multiplication tables and is moderately expensive.
///   It must be created once by the caller and shared — never constructed inside
///   this function, and never reconstructed per derivation in a loop.
///
/// - `secret_key`: A reference to the [`SecretKey`] to derive from. Borrowed
///   rather than moved so the caller retains the key for further operations
///   (WIF export, signing) after derivation.
///
/// # Returns
///
/// A [`PublicKey`] with `compressed` set to `true`, wrapping the raw curve
/// point in the bitcoin crate's type. The `compressed` flag is used by
/// downstream operations including address derivation and transaction signing.
///
/// # Notes
///
/// - `Secp256k1<All>` means the context supports both signing and verification.
///   This is the correct choice for general-purpose key operations.
/// - This operation cannot fail. Every valid [`SecretKey`] has exactly one
///   corresponding public key — no error handling is required.
/// - Public keys are safe to display, log, and distribute. They reveal nothing
///   about the private key.
///
/// # Example
///
/// ```rust
/// use bitcoin::secp256k1::Secp256k1;
/// use rand::rngs::OsRng;
///
/// let secp = Secp256k1::new();
/// let mut rng = OsRng;
///
/// let secret_key = generate_private_key(&mut rng);
/// let public_key = derive_public_key(&secp, &secret_key);
/// // secret_key is still valid here — derive_public_key only borrowed it
/// ```
fn derive_public_key(
    secp: &Secp256k1<bitcoin::secp256k1::All>,
    secret_key: &SecretKey,
) -> PublicKey {
    // Compute P = k × G via the precomputed tables in the secp context.
    //
    // The context is passed in (not created here) because Secp256k1::new()
    // allocates and initialises multiplication tables — constructing it per
    // call would be prohibitively expensive in a wallet deriving many keys.
    // One context instance, shared for the lifetime of the application, is
    // the correct pattern.
    //
    // This call is infallible: every valid SecretKey maps to exactly one point
    // on the curve. The return type is not a Result.
    let inner = bitcoin::secp256k1::PublicKey::from_secret_key(secp, secret_key);

    // Wrap the raw curve point in bitcoin::PublicKey with compressed: true.
    //
    // Compressed encoding (33 bytes):
    //   Byte 0     →  0x02 if y-coordinate is even
    //                 0x03 if y-coordinate is odd
    //   Bytes 1-32 →  32-byte x-coordinate
    //   (y is omitted; it can be recomputed from x and the parity bit)
    //
    // Uncompressed encoding (65 bytes, legacy pre-2012):
    //   Byte 0     →  0x04
    //   Bytes 1-32 →  x-coordinate
    //   Bytes 33-64 → y-coordinate
    //
    // Compressed and uncompressed keys derived from the same private key
    // produce DIFFERENT Bitcoin addresses. Always use compressed: true for
    // any wallet targeting post-2012 Bitcoin.
    PublicKey { compressed: true, inner }
}

/// Derives a P2PKH (Pay-to-Public-Key-Hash) Bitcoin address for testnet from
/// a compressed public key.
///
/// P2PKH is Bitcoin's original address format. Rather than encoding the public
/// key directly, it encodes a hash of the key — yielding shorter addresses and
/// keeping the public key undisclosed until the first spend (providing a mild
/// layer of quantum resistance while funds remain unspent).
///
/// # Address derivation
///
/// The `bitcoin` crate performs the following steps internally:
///
/// 1. SHA-256 hash the 33-byte compressed public key
/// 2. RIPEMD-160 hash the result → 20-byte public key hash
/// 3. Prepend the testnet version byte (`0x6F`)
/// 4. Compute checksum: first 4 bytes of SHA-256(SHA-256(versioned_hash))
/// 5. Base58Check encode (versioned_hash ++ checksum)
///
/// # Parameters
///
/// - `public_key`: A reference to the [`PublicKey`] to derive from. **Must**
///   have `compressed` set to `true`. A compressed and an uncompressed key
///   derived from the same private key will produce different addresses with
///   no error or warning — they are both valid P2PKH addresses, just different
///   ones. [`derive_public_key`] always produces a compressed key; if you
///   source `public_key` elsewhere, verify `public_key.compressed == true`.
///
/// # Returns
///
/// An [`Address`] valid on [`Network::Testnet`]. Testnet P2PKH addresses
/// always begin with `m` or `n`. **Do not send mainnet funds to a testnet
/// address** — the addresses are visually similar, no tooling will warn you,
/// and the funds will be permanently unrecoverable.
///
/// # Network safety
///
/// This function is **hardcoded to testnet**. For production use, make the
/// network a caller-supplied parameter:
///
/// ```rust
/// fn derive_address(public_key: &PublicKey, network: Network) -> Address {
///     Address::p2pkh(public_key, network)
/// }
/// ```
///
/// # Address formats
///
/// | Format  | Testnet prefix  | Version byte  | Use case                    |
/// |---------|-----------------|---------------|-----------------------------|
/// | P2PKH   | `m` or `n`      | `0x6F`        | Legacy; broad compatibility |
/// | P2SH    | `2`             | `0xC4`        | Multisig; wrapped SegWit    |
/// | P2WPKH  | `tb1q`          | n/a (bech32)  | Native SegWit; recommended  |
/// | P2TR    | `tb1p`          | n/a (bech32m) | Taproot; most efficient     |
///
/// For new wallets, prefer P2WPKH (native SegWit):
/// ```rust
/// Address::p2wpkh(public_key, network)  // ~30% lower transaction fees
/// ```
///
/// # Example
///
/// ```rust
/// use bitcoin::secp256k1::Secp256k1;
/// use rand::rngs::OsRng;
///
/// let secp = Secp256k1::new();
/// let mut rng = OsRng;
///
/// let secret_key = generate_private_key(&mut rng);
/// let public_key = derive_public_key(&secp, &secret_key);
/// let address    = derive_address(&public_key);
///
/// let s = address.to_string();
/// assert!(s.starts_with('m') || s.starts_with('n'));
/// ```
fn derive_address(public_key: &PublicKey) -> Address {
    // IMPLICIT CONTRACT: public_key.compressed must be true.
    //
    // Compressed and uncompressed variants of the same key produce different
    // addresses silently. This function and derive_public_key() are consistent
    // but there is no type-level enforcement of that contract.
    //
    // FUTURE IMPROVEMENT: Replace &PublicKey with a CompressedPublicKey
    // newtype that makes compression a compile-time guarantee:
    //
    //   struct CompressedPublicKey(PublicKey);
    //   fn derive_address(public_key: &CompressedPublicKey) -> Address { ... }

    // HARDCODED NETWORK: always produces a testnet address.
    //
    // Mainnet and testnet P2PKH addresses look visually similar. Sending real
    // funds to a testnet address is an irreversible loss with no recovery path.
    //
    // FUTURE IMPROVEMENT: Accept network as a parameter and consider P2WPKH:
    //   Address::p2wpkh(public_key, network)
    // SegWit (BIP-141, activated 2017) reduces transaction weight by ~30%
    // and has been the recommended format for all new wallets since 2018.
    Address::p2pkh(public_key, Network::Testnet)
}
```

**AI Response Summary:**
Produced the final combined version with: top-level pipeline ASCII diagram
with actual Rust types, formal /// sections for all three functions, inline
comments naming OS syscalls and explaining byte encoding, extended address
table including Taproot (P2TR) as a fourth row, and two concrete FUTURE
IMPROVEMENT comments with exact code changes. Zero warnings on cargo build.

**Helpfulness Evaluation:**
The combine prompt required the most direction — specifying exactly which
elements to take from each version. The result was production-quality but
still required human judgment. The combine prompt is a skill in itself.

**Key Takeaway:**
Run Prompt 1 for the API contract, Prompt 2 for the inline reasoning, then
combine with a directed third prompt. Three prompts produce better
documentation than any single prompt could. The CompressedPublicKey newtype
suggestion — only visible when documenting carefully — was the most valuable
insight from the entire exercise.