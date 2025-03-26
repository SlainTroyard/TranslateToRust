struct Solution;

impl Solution {
    fn count_of_substrings(word: String, k: i32) -> i64 {
        const VOWEL_MASK: u32 = 1065233; // Binary: 1000000000000000000000001, represents vowels a, e, i, o, u
        let mut ans: i64 = 0;
        let mut cnt_vowel1 = [0; 21]; // Indexes 0-20 correspond to 'a'-'u'
        let mut cnt_vowel2 = [0; 21];
        let mut size_vowel1 = 0;
        let mut size_vowel2 = 0;
        let mut cnt_consonant1 = 0;
        let mut cnt_consonant2 = 0;
        let mut left1 = 0;
        let mut left2 = 0;

        for (i, c) in word.chars().enumerate() {
            let b = (c as u8 - b'a') as usize;
            if (VOWEL_MASK >> b) & 1 != 0 {
                // It's a vowel
                if cnt_vowel1[b] == 0 {
                    size_vowel1 += 1;
                }
                cnt_vowel1[b] += 1;

                if cnt_vowel2[b] == 0 {
                    size_vowel2 += 1;
                }
                cnt_vowel2[b] += 1;
            } else {
                // It's a consonant
                cnt_consonant1 += 1;
                cnt_consonant2 += 1;
            }

            // Adjust the first window
            while size_vowel1 == 5 && cnt_consonant1 >= k {
                let out_char = word[left1];
                let out = (out_char as u8 - b'a') as usize;
                if (VOWEL_MASK >> out) & 1 != 0 {
                    cnt_vowel1[out] -= 1;
                    if cnt_vowel1[out] == 0 {
                        size_vowel1 -= 1;
                    }
                } else {
                    cnt_consonant1 -= 1;
                }
                left1 += 1;
            }

            // Adjust the second window
            while size_vowel2 == 5 && cnt_consonant2 > k {
                let out_char = word[left2];
                let out = (out_char as u8 - b'a') as usize;
                if (VOWEL_MASK >> out) & 1 != 0 {
                    cnt_vowel2[out] -= 1;
                    if cnt_vowel2[out] == 0 {
                        size_vowel2 -= 1;
                    }
                } else {
                    cnt_consonant2 -= 1;
                }
                left2 += 1;
            }

            // Add the number of valid substrings ending at current position
            ans += (left1 - left2) as i64;
        }

        ans
    }
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let word_size = parts[0].parse::<usize>().unwrap();
    let word = parts[1].to_string();
    let k = parts[2].parse::<i32>().unwrap();

    let ans = Solution::count_of_substrings(word, k);
    println!("{}", ans);
}