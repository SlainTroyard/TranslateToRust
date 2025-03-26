use std::io::{self, Read};

struct Solution;

impl Solution {
    pub fn count_of_substrings(word: &str, k: usize) -> i64 {
        const VOWEL_MASK: u32 = 1065233; // Bitmask to check vowels
        let mut ans = 0i64;

        let mut cnt_vowel1 = [0; ('u' as usize - 'a' as usize) + 1];
        let mut cnt_vowel2 = [0; ('u' as usize - 'a' as usize) + 1];

        let mut size_vowel1 = 0;
        let mut size_vowel2 = 0;
        let mut cnt_consonant1 = 0;
        let mut cnt_consonant2 = 0;

        let mut left1 = 0;
        let mut left2 = 0;

        for (right, &b) in word.as_bytes().iter().enumerate() {
            let b = (b - b'a') as usize;

            if (VOWEL_MASK >> b) & 1 == 1 {
                if cnt_vowel1[b] == 0 {
                    size_vowel1 += 1;
                }
                cnt_vowel1[b] += 1;

                if cnt_vowel2[b] == 0 {
                    size_vowel2 += 1;
                }
                cnt_vowel2[b] += 1;
            } else {
                cnt_consonant1 += 1;
                cnt_consonant2 += 1;
            }

            while size_vowel1 == 5 && cnt_consonant1 >= k {
                let out = (word.as_bytes()[left1] - b'a') as usize;
                if (VOWEL_MASK >> out) & 1 == 1 {
                    if cnt_vowel1[out] == 1 {
                        size_vowel1 -= 1;
                    }
                    cnt_vowel1[out] -= 1;
                } else {
                    cnt_consonant1 -= 1;
                }
                left1 += 1;
            }

            while size_vowel2 == 5 && cnt_consonant2 > k {
                let out = (word.as_bytes()[left2] - b'a') as usize;
                if (VOWEL_MASK >> out) & 1 == 1 {
                    if cnt_vowel2[out] == 1 {
                        size_vowel2 -= 1;
                    }
                    cnt_vowel2[out] -= 1;
                } else {
                    cnt_consonant2 -= 1;
                }
                left2 += 1;
            }

            ans += (left1 - left2) as i64;
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.lines();

    let word_size: usize = lines.next().unwrap().parse().expect("Invalid input");
    let word = lines.next().unwrap();
    let k: usize = lines.next().unwrap().parse().expect("Invalid input");

    // Ensure `word` matches the length in `word_size`
    assert_eq!(word.len(), word_size);

    // Use the count_of_substrings method and print the result
    let solution = Solution;
    let ans = solution.count_of_substrings(word, k);
    println!("{}", ans);
}