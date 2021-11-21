mod alphabet;
mod sort;

// Alphaber getters.
pub use alphabet::{get_lowercase, get_uppercase, get_valid_as_first};

// Special characters.
pub use alphabet::{ETH, THORN, O_CAUDATA, ASH, SLASHED_O, AE, OE };

// Sorting algorithm.
pub use sort::old_norse_sort;