const ALPHABET_UPPER: [char; 34] = ['A', 'Á', 'B', 'D', 'Ð', 'E', 'É', 'F', 'G', 'H', 'I', 'Í', 'J', 'K', 'L', 'M', 'N', 'O', 'Ó', 'P', 'R', 'S', 'T', 'U', 'Ú', 'V', 'W', 'Y', 'Ý', 'Þ', 'Æ', 'Ǫ', 'Ø', 'Œ'];
const ALPHABET_LOWER: [char; 34] = ['a', 'á', 'b', 'd', 'ð', 'e', 'é', 'f', 'g', 'h', 'i', 'í', 'j', 'k', 'l', 'm', 'n', 'o', 'ó', 'p', 'r', 's', 't', 'u', 'ú', 'v', 'w', 'y', 'ý', 'þ', 'æ', 'ǫ', 'ø', 'œ'];

// Additional non-standard letters & signs needed for sorting.
const ADDITIONAL_CHARS: [char; 3] = ['ö', 'x', 'z'];

// Letters valid as first characters of a word.
const VALID_AS_FIRST: [char; 32] = ['a', 'á', 'b', 'd', 'e', 'é', 'f', 'g', 'h', 'i', 'í', 'j', 'k', 'l', 'm', 'n', 'o', 'ó', 'p', 'r', 's', 't', 'u', 'ú', 'v', 'y', 'ý', 'þ', 'æ', 'ǫ', 'ø', 'œ'];

// "Special" letters.
pub const ETH: char = 'ð';
pub const THORN: char = 'þ';
pub const O_CAUDATA: char = 'ǫ';
pub const ASH: char = 'æ';
pub const SLASHED_O: char = 'Ø';
pub const OE: char = 'œ';
pub const AE: char = ASH;

pub fn get_uppercase() -> [char; 34] {
    ALPHABET_UPPER
}

pub fn get_lowercase() -> [char; 34] {
    ALPHABET_LOWER
}

pub fn get_valid_as_first() -> [char; 32] {
    VALID_AS_FIRST
}

pub fn get_sorting_alphabet() -> Vec<&'static char> {
   let mut combined: Vec<&char> = vec!(&'-');

   for (_, letter) in ALPHABET_LOWER.iter().enumerate() {
       combined.push(letter)
   }

   for (_, letter) in ADDITIONAL_CHARS.iter().enumerate() {
    combined.push(letter)
}

   combined
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upper_and_lower_contain_same_characters_in_same_spots() {
        let upper = get_uppercase();
        let lower = get_lowercase();
        
        for (index, letter) in upper.iter().enumerate() {
            let lowercase_version = letter.to_lowercase().nth(0).unwrap();
            assert_eq!(lowercase_version, lower[index]);
        }
    }

    #[test]
    fn valid_as_first_does_not_contain_eth() {
        let valid_as_first = get_valid_as_first();
        
        for (_, letter) in valid_as_first.iter().enumerate() {            
            assert_ne!(letter, &ETH);
        }
    }
}