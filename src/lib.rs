use lazy_static;
use packed_simd_2::u8x32;
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::str;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn simd_split_whitespace<'a>(text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut start = 0;
    let bytes = text.as_bytes();
    let space = ' ' as u8;

    let simd_space = u8x32::splat(space);
    let len = bytes.len() / 32 * 32;

    for i in (0..len).step_by(32) {
        let chunk = u8x32::from_slice_unaligned(&bytes[i..i + 32]);
        let mask = chunk.eq(simd_space);

        for j in 0..32 {
            if mask.extract(j) {
                if start < i + j {
                    result.push(str::from_utf8(&bytes[start..i + j]).unwrap());
                }
                start = i + j + 1;
            }
        }
    }

    for i in len..bytes.len() {
        if bytes[i] == space {
            if start < i {
                result.push(str::from_utf8(&bytes[start..i]).unwrap());
            }
            start = i + 1;
        }
    }

    if start < bytes.len() {
        result.push(str::from_utf8(&bytes[start..]).unwrap());
    }

    result
}

pub fn rustlatin(sentence: &str) -> String {
    let res: Vec<String> = sentence
        .split_whitespace()
        .map(|word| {
            let first_char = word.chars().nth(0).unwrap();
            if VOWELS.contains(&first_char) {
                return String::from("sr") + &word;
            }
            return String::from(word) + "rs";
        })
        .collect(); //::<Vec<String>>()
                    // .join(" ");
    res.join(" ")
}

lazy_static::lazy_static! {
    static ref VOWELS_MAP: HashSet<char> = {
        let mut set = HashSet::new();
        set.insert('a');
        set.insert('e');
        set.insert('i');
        set.insert('o');
        set.insert('u');
        set
    };
}

pub fn rustlatin_map(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| {
            let first_char = word.chars().next().unwrap();
            let mut transformed_word = String::with_capacity(word.len() + 2);
            if VOWELS_MAP.contains(&first_char) {
                transformed_word.push_str("sr");
            }
            transformed_word.push_str(word);
            if !VOWELS_MAP.contains(&first_char) {
                transformed_word.push_str("rs");
            }
            transformed_word
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn rustlatin_faster(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut transformed_words = VecDeque::with_capacity(words.len());
    let mut result = String::with_capacity(sentence.len() + words.len() * 2);

    for word in words {
        let first_char = word.chars().next().unwrap();
        let mut transformed_word = String::with_capacity(word.len() + 2);
        if VOWELS.contains(&first_char) {
            transformed_word.push_str("sr");
        }
        transformed_word.push_str(word);
        if !VOWELS.contains(&first_char) {
            transformed_word.push_str("rs");
        }
        transformed_words.push_back(transformed_word);
    }

    while let Some(transformed_word) = transformed_words.pop_front() {
        result.push_str(&transformed_word);
        if !transformed_words.is_empty() {
            result.push(' ');
        }
    }

    result
}

pub fn rustlatin_fastest(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut result = String::with_capacity(sentence.len() + words.len() * 2);

    for (i, word) in words.iter().enumerate() {
        let first_char = word.chars().next().unwrap();
        let is_vowel = VOWELS.contains(&first_char);

        if is_vowel {
            result.push_str("sr");
        }
        result.push_str(word);
        if !is_vowel {
            result.push_str("rs");
        }

        if i < words.len() - 1 {
            result.push(' ');
        }
    }

    result
}

pub fn rustlatin_fastest_simd(sentence: &str) -> String {
    let words = simd_split_whitespace(&sentence);

    let mut result = String::with_capacity(sentence.len() + words.len() * 2);

    for (i, word) in words.iter().enumerate() {
        let first_char = word.chars().next().unwrap();
        let is_vowel = VOWELS.contains(&first_char);

        if is_vowel {
            result.push_str("sr");
        }
        result.push_str(word);
        if !is_vowel {
            result.push_str("rs");
        }

        if i < words.len() - 1 {
            result.push(' ');
        }
    }

    result
}

pub fn rustlatin_fastest_match(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut result = String::with_capacity(sentence.len() + words.len() * 2);

    for (i, word) in words.iter().enumerate() {
        let first_char = word.chars().next().unwrap();
        let is_vowel = is_vowel_fast(first_char);

        if is_vowel {
            result.push_str("sr");
        }
        result.push_str(word);
        if !is_vowel {
            result.push_str("rs");
        }

        if i < words.len() - 1 {
            result.push(' ');
        }
    }

    result
}

pub fn rustlatin_fastest_map(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut result = String::with_capacity(sentence.len() + words.len() * 2);

    for (i, word) in words.iter().enumerate() {
        let first_char = word.chars().next().unwrap();
        let is_vowel = VOWELS_MAP.contains(&first_char);

        if is_vowel {
            result.push_str("sr");
        }
        result.push_str(word);
        if !is_vowel {
            result.push_str("rs");
        }

        if i < words.len() - 1 {
            result.push(' ');
        }
    }

    result
}

#[inline(always)]
fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c)
}

#[inline(always)]
fn is_vowel_fast(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

pub fn rustlatin_fastester(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut result = String::with_capacity(sentence.len() + words.len() * 2);

    for (index, word) in words.iter().enumerate() {
        let first_char = word.chars().next().unwrap();
        let vowel = is_vowel(first_char);
        if vowel {
            result.push_str("sr");
        }
        result.push_str(word);
        if !vowel {
            result.push_str("rs");
        }
        if index < words.len() - 1 {
            result.push(' ');
        }
    }

    result
}

pub fn rustlatin_rayon(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let transformed_words: Vec<String> = words
        .par_iter()
        .map(|&word| {
            let first_char = word.chars().next().unwrap();
            let mut transformed_word = String::with_capacity(word.len() + 2);
            let contains = VOWELS.contains(&first_char);
            if contains {
                transformed_word.push_str("sr");
            }
            transformed_word.push_str(word);
            if !contains {
                transformed_word.push_str("rs");
            }
            transformed_word
        })
        .collect();

    transformed_words.join(" ")
}

pub fn rustlatin_rayon_map(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let transformed_words: Vec<String> = words
        .par_iter()
        .map(|&word| {
            let first_char = word.chars().next().unwrap();
            let mut transformed_word = String::with_capacity(word.len() + 2);
            let contains = VOWELS_MAP.contains(&first_char);
            if contains {
                transformed_word.push_str("sr");
            }
            transformed_word.push_str(word);
            if !contains {
                transformed_word.push_str("rs");
            }
            transformed_word
        })
        .collect();

    transformed_words.join(" ")
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(rustlatin("abc".to_string()), "srabc".to_string());
//         assert_eq!(rustlatin("efg".to_string()), "srefg".to_string());
//         assert_eq!(
//             rustlatin("scope is hot".to_string()),
//             "scopers sris hotrs".to_string()
//         );
//     }
// }
