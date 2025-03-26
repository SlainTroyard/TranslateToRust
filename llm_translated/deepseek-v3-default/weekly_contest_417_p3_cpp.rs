use std::collections::HashMap;
use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let word_size: usize = parts.next().unwrap().parse().expect("Invalid word size");
    let word = parts.next().unwrap().to_string();
    let k: i32 = parts.next().unwrap().parse().expect("Invalid k");

    // Compute the answer
    let ans = count_of_substrings(word, k);

    // Print the result
    println!("{}", ans);
}

fn count_of_substrings(word: String, k: i32) -> i64 {
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

    for (i, c) in word.chars().enumerate() {
        let b = (c as u8 - b'a') as i32;
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