const ALPHABET_UPPER: [char; 34] = ['A', 'Á', 'B', 'D', 'Ð', 'E', 'É', 'F', 'G', 'H', 'I', 'Í', 'J', 'K', 'L', 'M', 'N', 'O', 'Ó', 'P', 'R', 'S', 'T', 'U', 'Ú', 'V', 'W', 'Y', 'Ý', 'Þ', 'Æ', 'Ǫ', 'Ø', 'Œ'];
const ALPHABET_LOWER: [char; 34] = ['a', 'á', 'b', 'd', 'ð', 'e', 'é', 'f', 'g', 'h', 'i', 'í', 'j', 'k', 'l', 'm', 'n', 'o', 'ó', 'p', 'r', 's', 't', 'u', 'ú', 'v', 'w', 'y', 'ý', 'þ', 'æ', 'ǫ', 'ø', 'œ'];

pub fn get_uppercase() -> [char; 34] {
    ALPHABET_UPPER
}

pub fn get_lowercase() -> [char; 34] {
    ALPHABET_LOWER
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
}