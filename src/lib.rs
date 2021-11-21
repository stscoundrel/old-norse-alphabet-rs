//! Old Norse alphabet constants & sort for Rust.
//! 
//! Old Norse constains letters that may be hard to type with most keyboards. Prime examples being þ, ð and ǫ. There are also some letters "missing", like c and q. This package offers the alphabet & tricky individual letters as JS constants.
//! 
//! Also provides sorting function to get the old norse alphabet order just right.

mod alphabet;
mod sort;

// Alphaber getters.
pub use alphabet::{get_lowercase, get_uppercase, get_valid_as_first};

// Special characters.
pub use alphabet::{ETH, THORN, O_CAUDATA, ASH, SLASHED_O, AE, OE };

// Sorting algorithm.
pub use sort::old_norse_sort;