use std::cmp::Ordering;
use crate::alphabet::get_sorting_alphabet;

fn get_character_by_index(string: &str, index: &usize) -> char {
    string.chars().nth(*index).unwrap()
}

fn get_index_of_character(character: &char, alphabet: &Vec<&char>) -> i32 {
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
    
      if a.len() <= index {
          return Ordering::Less;
      }

      if b.len() <= index {
          return Ordering::Greater;
      }

      let a_letter: char = get_character_by_index(&a, &index);
      let b_letter: char = get_character_by_index(&b, &index);
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
}