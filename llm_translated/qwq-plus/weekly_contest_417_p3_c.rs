use std::io;

const VOWEL_MASK: u32 = 1065233;

fn count_of_substrings(word: &str, k: i32) -> i64 {
    let mut ans: i64 = 0;

    let mut cnt_vowel1 = [0; 26];
    let mut cnt_vowel2 = [0; 26];
    let mut size_vowel1: usize = 0;
    let mut size_vowel2: usize = 0;
    let mut cnt_consonant1: usize = 0;
    let mut cnt_consonant2: usize = 0;
    let mut left1: usize = 0;
    let mut left2: usize = 0;

    let length = word.len();

    for (i, &ch) in word.as_bytes().iter().enumerate() {
        let b = (ch - b'a') as usize;

        if (VOWEL_MASK & (1 << b)) != 0 {
            // Update window1 vowel counts
            if cnt_vowel1[b] == 0 {
                size_vowel1 += 1;
            }
            cnt_vowel1[b] += 1;
            // Update window2 vowel counts
            if cnt_vowel2[b] == 0 {
                size_vowel2 += 1;
            }
            cnt_vowel2[b] += 1;
        } else {
            // Update consonant counts
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }

        // Shrink window1 to maintain consonant count >=k and all vowels present
        while size_vowel1 == 5 && cnt_consonant1 >= k as usize {
            let out_char = word.as_bytes()[left1];
            let out = (out_char - b'a') as usize;
            if (VOWEL_MASK & (1 << out)) != 0 {
                cnt_vowel1[out] -= 1;
                if cnt_vowel1[out] == 0 {
                    size_vowel1 -= 1;
                }
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }

        // Shrink window2 to maintain consonant count >k and all vowels present
        while size_vowel2 == 5 && cnt_consonant2 > k as usize {
            let out_char = word.as_bytes()[left2];
            let out = (out_char - b'a') as usize;
            if (VOWEL_MASK & (1 << out)) != 0 {
                cnt_vowel2[out] -= 1;
                if cnt_vowel2[out] == 0 {
                    size_vowel2 -= 1;
                }
            } else {
                cnt_consonant2 -= 1;
            }
            left2 += 1;
        }

        // Add valid substrings ending at current character
        ans += (left1 as i64) - (left2 as i64);
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut parts = input.split_whitespace();
    let word_size: usize = parts.next().unwrap().parse().unwrap();
    let word = parts.next().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    // Ensure the word length matches the given wordSize
    assert!(word.len() == word_size);

    let ans = count_of_substrings(word, k);
    println!("{}", ans);
}