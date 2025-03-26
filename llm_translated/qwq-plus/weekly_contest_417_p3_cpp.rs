use std::io;

struct Solution;

impl Solution {
    fn count_of_substrings(word: String, k: i32) -> i64 {
        const VOWEL_MASK: u32 = 1065233;
        let mut ans: i64 = 0;
        let mut cnt_vowel1 = [0i32; 21];
        let mut cnt_vowel2 = [0i32; 21];
        let mut size_vowel1 = 0;
        let mut size_vowel2 = 0;
        let mut cnt_consonant1 = 0;
        let mut cnt_consonant2 = 0;
        let mut left1 = 0;
        let mut left2 = 0;

        for (&current_char, _) in word.as_bytes().iter().enumerate() {
            let b = (current_char as u8 - b'a') as usize;
            if (VOWEL_MASK >> b as u32) & 1 != 0 {
                // Handle vowel
                if cnt_vowel1[b] == 0 {
                    size_vowel1 += 1;
                }
                cnt_vowel1[b] += 1;

                if cnt_vowel2[b] == 0 {
                    size_vowel2 += 1;
                }
                cnt_vowel2[b] += 1;
            } else {
                // Handle consonant
                cnt_consonant1 += 1;
                cnt_consonant2 += 1;
            }

            // Adjust left1 to satisfy first condition
            while size_vowel1 == 5 && cnt_consonant1 >= k {
                let out_char = word.as_bytes()[left1] as u8 - b'a';
                let out = out_char as usize;
                if (VOWEL_MASK >> out as u32) & 1 != 0 {
                    cnt_vowel1[out] -= 1;
                    if cnt_vowel1[out] == 0 {
                        size_vowel1 -= 1;
                    }
                } else {
                    cnt_consonant1 -= 1;
                }
                left1 += 1;
            }

            // Adjust left2 to satisfy second condition
            while size_vowel2 == 5 && cnt_consonant2 > k {
                let out_char = word.as_bytes()[left2] as u8 - b'a';
                let out = out_char as usize;
                if (VOWEL_MASK >> out as u32) & 1 != 0 {
                    cnt_vowel2[out] -= 1;
                    if cnt_vowel2[out] == 0 {
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
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let word_size: usize = parts.next().unwrap().parse().unwrap();
    let word: String = parts.next().unwrap().to_string();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    let ans = Solution::count_of_substrings(word, k);
    println!("{}", ans);
}