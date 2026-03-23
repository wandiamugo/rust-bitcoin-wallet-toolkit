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

---

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


## Prompt #4 — *(coming next)*

---

## Prompt #5 — *(coming next)*