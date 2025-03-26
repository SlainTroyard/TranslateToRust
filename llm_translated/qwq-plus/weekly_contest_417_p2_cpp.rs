use std::io;

pub struct Solution;

impl Solution {
    const VOWEL_MASK: u32 = 1065233; // Binary has bits set for vowels (a, e, i, o, u)

    fn f(&self, word: &str, k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut cnt1 = [0i32; 21]; // Tracks counts of each vowel (0 to 'u' - 'a' = 20)
        let mut size1 = 0i32; // Number of vowel types present
        let mut cnt2 = 0i32; // Count of consonants
        let mut left = 0usize;

        for &b in word.as_bytes() {
            let offset = (b as i32) - ('a' as u8 as i32);
            if (Self::VOWEL_MASK >> offset) & 1 != 0 {
                // Vowel: update vowel counts
                let idx = offset as usize;
                if cnt1[idx] == 0 {
                    size1 += 1;
                }
                cnt1[idx] += 1;
            } else {
                // Consonant: increment consonant count
                cnt2 += 1;
            }

            // Adjust window to maintain conditions
            while size1 == 5 && cnt2 >= k {
                let out_char = word.as_bytes()[left];
                let out_offset = (out_char as i32) - ('a' as u8 as i32);
                if (Self::VOWEL_MASK >> out_offset) & 1 != 0 {
                    // Outgoing character is a vowel: decrement its count
                    let out_idx = out_offset as usize;
                    cnt1[out_idx] -= 1;
                    if cnt1[out_idx] == 0 {
                        size1 -= 1;
                    }
                } else {
                    // Outgoing character is consonant: decrement consonant count
                    cnt2 -= 1;
                }
                left += 1;
            }
            ans += left as i64; // Add valid substrings ending at current position
        }
        ans
    }

    pub fn count_of_substrings(&self, word: String, k: i32) -> i64 {
        self.f(&word, k) - self.f(&word, k + 1)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");

    let mut parts = input.split_whitespace();

    let word_size: usize = parts.next().expect("No word size provided")
        .parse().expect("Invalid word size");
    let word: String = parts.next().expect("No word provided").to_string();
    let k: i32 = parts.next().expect("No k provided")
        .parse().expect("Invalid k");

    let sol = Solution {};
    let ans = sol.count_of_substrings(word, k);
    println!("{}", ans);
}