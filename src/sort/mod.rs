use std::cmp::Ordering;
use crate::alphabet::get_sorting_alphabet;

fn length(content: &str) -> usize {
    content.chars().count()
}

fn get_character_by_index(string: &str, index: &usize) -> char {
    string.to_lowercase().chars().nth(*index).unwrap()
}

fn get_index_of_character(character: &char, alphabet: &[&char]) -> i32 {
    let result = alphabet.iter().position(|letter| letter.eq(&character));

    match result {
        Some(letter_index) => letter_index as i32,
        None => -1,
    }
}

pub fn compare(a: &str, b: &str, index: usize, alphabet: Vec<&char>) -> Ordering {
    if a.to_lowercase() == b.to_lowercase() {
        return Ordering::Equal;
      }
    
      if length(a) <= index {
          return Ordering::Less;
      }

      if length(b) <= index {
          return Ordering::Greater;
      }

      let a_letter: char = get_character_by_index(a, &index);
      let b_letter: char = get_character_by_index(b, &index);
      let index_a: i32 = get_index_of_character(&a_letter, &alphabet);
      let index_b: i32 = get_index_of_character(&b_letter, &alphabet);
    
      if index_a == -1 && index_b != -1 {
          return Ordering::Greater
      }

      if index_b == -1 && index_a != -1 {
          return Ordering::Less
      }
    
      if index_a == index_b {
        let next_index = index + 1;
    
        return compare(a, b, next_index, alphabet);
      }

      index_a.cmp(&index_b)
}

pub fn old_norse_sort(a: &str, b: &str) -> Ordering {
    let alphabet = get_sorting_alphabet();

    compare(a, b, 0, alphabet)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn vecs_are_equal<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }

    #[test]
    fn sorts_by_old_norse_alphabet() {
        let mut words = vec!["öðli", "ógnan", "æðrask", "aðili", "þakkan", "áfir", "á-auki", "él-ligr", "maðka", "ef-lauss", "œgir", "áðr", "maðr", "madr", "mæðr"];
        let expected = vec!["aðili", "á-auki", "áðr", "áfir", "ef-lauss", "él-ligr", "madr", "maðka", "maðr", "mæðr", "ógnan", "þakkan", "æðrask", "œgir", "öðli"];
        
        words.sort_by(|a, b| old_norse_sort(a, b));

        assert!(vecs_are_equal(&words, &expected));        
    }

    #[test]
    fn sorts_with_identical_words() {
        let mut words = vec!["aðili", "maðka", "þakkan", "maðka"];
        let expected = vec!["aðili", "maðka", "maðka", "þakkan"];
        
        words.sort_by(|a, b| old_norse_sort(a, b));

        assert!(vecs_are_equal(&words, &expected));        
    }

    #[test]
    fn sorts_mixed_upper_and_lower_letters() {
        let mut words = vec!["aðili", "maðka", "þakkan", "adal", "ADAL", "maðka"];
        let expected = vec!["adal", "ADAL", "aðili", "maðka", "maðka", "þakkan"];
        
        words.sort_by(|a, b| old_norse_sort(a, b));

        assert!(vecs_are_equal(&words, &expected));        
    }

    #[test]
    fn word_length_applies_when_sorting() {
        let mut words = vec!["aðild", "AÐAL", "abbast", "aðal-vellir", "AÐA", "abbindi"];
        let expected = vec!["abbast", "abbindi", "AÐA", "AÐAL", "aðal-vellir", "aðild"];
        
        words.sort_by(|a, b| old_norse_sort(a, b));

        for (_, word) in words.iter().enumerate() {
            println!("{}", word);
        }

        assert!(vecs_are_equal(&words, &expected));        
    }

    #[test]
    fn sorts_uncommon_and_loaned_letters() {
        let mut words = vec!["Gerzkr", "ger", "hal-dreki", "ÆZLI", "gervi", "eyxn", "halzi", "æxling"];
        let expected = vec!["eyxn", "ger", "gervi", "Gerzkr", "hal-dreki", "halzi", "æxling", "ÆZLI"];
        
        words.sort_by(|a, b| old_norse_sort(a, b));

        assert!(vecs_are_equal(&words, &expected));        
    }
}