use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the word size (though it's not used in the function)
    let _word_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Read the word
    let word = lines.next().unwrap().unwrap();
    
    // Read k
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Compute the answer
    let ans = count_of_substrings(&word, k);
    
    // Print the answer
    println!("{}", ans);
}

fn count_of_substrings(word: &str, k: i32) -> i64 {
    const VOWEL_MASK: i32 = 1065233;
    let mut ans: i64 = 0;
    let mut cnt_vowel1 = HashMap::new();
    let mut cnt_vowel2 = HashMap::new();
    let mut size_vowel1 = 0;
    let mut size_vowel2 = 0;
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;
    let mut left1 = 0;
    let mut left2 = 0;

    for (right, ch) in word.chars().enumerate() {
        let b = (ch as u8 - b'a') as i32;
        if (VOWEL_MASK >> b) & 1 == 1 {
            *cnt_vowel1.entry(b).or_insert(0) += 1;
            if cnt_vowel1[&b] == 1 {
                size_vowel1 += 1;
            }
            *cnt_vowel2.entry(b).or_insert(0) += 1;
            if cnt_vowel2[&b] == 1 {
                size_vowel2 += 1;
            }
        } else {
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }

        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out = (word.chars().nth(left1).unwrap() as u8 - b'a') as i32;
            if (VOWEL_MASK >> out) & 1 == 1 {
                *cnt_vowel1.get_mut(&out).unwrap() -= 1;
                if cnt_vowel1[&out] == 0 {
                    size_vowel1 -= 1;
                }
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }

        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out = (word.chars().nth(left2).unwrap() as u8 - b'a') as i32;
            if (VOWEL_MASK >> out) & 1 == 1 {
                *cnt_vowel2.get_mut(&out).unwrap() -= 1;
                if cnt_vowel2[&out] == 0 {
                    size_vowel2 -= 1;
                }
            } else {
                cnt_consonant2 -= 1;
            }
            left2 += 1;
        }

        ans += (left1 - left2) as i64;
    }

    ans
}