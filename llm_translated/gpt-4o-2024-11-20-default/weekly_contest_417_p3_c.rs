use std::io::{self, Read};
use std::collections::HashMap;

const VOWEL_MASK: u32 = 1065233; // A bitmask for vowels: a = 1 << 0, e = 1 << 4, i = 1 << 8, o = 1 << 14, u = 1 << 20

fn count_of_substrings(word: &str, k: usize) -> i64 {
    let mut ans: i64 = 0;

    // Maps and counters for vowel frequency and consonants
    let mut cnt_vowel1: HashMap<u8, usize> = HashMap::new();
    let mut cnt_vowel2: HashMap<u8, usize> = HashMap::new();
    let mut size_vowel1 = 0;
    let mut size_vowel2 = 0;
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;

    let mut left1 = 0;
    let mut left2 = 0;

    let bytes = word.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        let letter = b - b'a';

        if (VOWEL_MASK >> letter) & 1 != 0 {
            // Update window1
            *cnt_vowel1.entry(letter).or_insert(0) += 1;
            if cnt_vowel1[&letter] == 1 {
                size_vowel1 += 1;
            }
            // Update window2
            *cnt_vowel2.entry(letter).or_insert(0) += 1;
            if cnt_vowel2[&letter] == 1 {
                size_vowel2 += 1;
            }
        } else {
            // It's a consonant
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }

        // Shrink window1 to maintain cnt_consonant1 >= k
        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out = bytes[left1] - b'a';
            if (VOWEL_MASK >> out) & 1 != 0 {
                let entry = cnt_vowel1.get_mut(&out).unwrap();
                *entry -= 1;
                if *entry == 0 {
                    size_vowel1 -= 1;
                }
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }

        // Shrink window2 to maintain cnt_consonant2 > k
        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out = bytes[left2] - b'a';
            if (VOWEL_MASK >> out) & 1 != 0 {
                let entry = cnt_vowel2.get_mut(&out).unwrap();
                *entry -= 1;
                if *entry == 0 {
                    size_vowel2 -= 1;
                }
            } else {
                cnt_consonant2 -= 1;
            }
            left2 += 1;
        }

        // Add the number of valid substrings ending at the current character
        ans += (left1 - left2) as i64;
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut lines = input.lines();

    let word_size: usize = lines
        .next()
        .expect("Expected word size")
        .parse()
        .expect("Invalid word size");

    let word = lines.next().expect("Expected word");
    assert_eq!(word.len(), word_size, "Word length doesn't match word size");

    let k: usize = lines
        .next()
        .expect("Expected k")
        .parse()
        .expect("Invalid k value");

    let result = count_of_substrings(word, k);
    println!("{}", result);
}