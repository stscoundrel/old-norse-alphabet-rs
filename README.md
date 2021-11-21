# Old Norse Alphabet

Old Norse alphabet constants & sort for Rust

### Motivation

Old Norse constains letters that may be hard to type with most keyboards. Prime examples being þ, ð and ǫ. There are also some letters "missing", like c and q. This package offers the alphabet & tricky individual letters as JS constants.

Also provides sorting function to get the old norse alphabet order just right.


### Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
old_norse_alphabet = "1.0.0"
```


### Usage

The crate offers Old Norse alphabet in lower/uppercases, and a function for sorting by old norse alphabetical order.

#### Sort

The crate exposes custom compare function for getting old norse alphabetical order just right.

```rust
use old_norse_alphabet::old_norse_sort;

// Sample vec of old norse words.
let mut words = vec!["öðli", "ógnan", "æðrask", "aðili", "þakkan", "áfir", "á-auki", "él-ligr", "maðka", "ef-lauss", "œgir", "áðr", "maðr", "madr", "mæðr"];
words.sort_by(|a, b| old_norse_sort(a, b));

// Sorted vec is: "aðili", "á-auki", "áðr", "áfir", "ef-lauss", "él-ligr", "madr", "maðka", "maðr", "mæðr", "ógnan", "þakkan", "æðrask", "œgir", "öðli"
```

#### Alphabet

To use the alphabet constants:

```rust
use old_norse_alphabet::{get_lowercase, get_uppercase};

// Outputs are [char; 34]
let lower = get_lowercase();
let upper = get_uppercase();

for (index, letter) in lower.iter().enumerate() {
    println!("{}. letter: {}", index + 1, letter) // eg. 5. letter: ð
}
```

Exposed special characters:

```rust
use old_norse_alphabet::{ETH, THORN, O_CAUDATA, SLASHED_O, AE, OE, ASH};

println!("{}", ETH); // ð
println!("{}", THORN); // þ
println!("{}", O_CAUDATA); // ǫ
println!("{}", SLASHED_O); // ø
println!("{}", AE); // æ
println!("{}", OE); // œ
println!("{}", ASH); // Alternative export of AE
```


### About Old Norse

[Old Norse](https://en.wikipedia.org/wiki/Old_Norse) was a North Germanic language that was spoken by inhabitants of Scandinavia and their overseas settlements from about the 7th to the 15th centuries.
