use std::io;
use std::io::Read;

struct Solution {
    vowel_mask: i32,
}

impl Solution {
    fn new() -> Self {
        Solution { vowel_mask: 1065233 }
    }

    fn f(&self, word: &String, k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut cnt1: [i32; 26] = [0; 26];
        let mut size1: i32 = 0;
        let mut cnt2: i32 = 0;
        let mut left: usize = 0;

        for (right, &b) in word.as_bytes().iter().enumerate() {
            let b = (b - b'a') as usize;

            if (self.vowel_mask >> b) & 1 == 1 {
                if cnt1[b] == 0 {
                    size1 += 1;
                }
                cnt1[b] += 1;
            } else {
                cnt2 += 1;
            }

            while size1 == 5 && cnt2 >= k {
                let out = (word.as_bytes()[left] - b'a') as usize;

                if (self.vowel_mask >> out) & 1 == 1 {
                    cnt1[out] -= 1;
                    if cnt1[out] == 0 {
                        size1 -= 1;
                    }
                } else {
                    cnt2 -= 1;
                }
                left += 1;
            }
            ans += left as i64;
        }

        ans
    }

    fn count_of_substrings(&self, word: String, k: i32) -> i64 {
        self.f(&word, k) - self.f(&word, k + 1)
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let word_size: i32 = lines.next().unwrap().parse().unwrap();
    let word: String = lines.next().unwrap().to_string();
    let k: i32 = lines.next().unwrap().parse().unwrap();

    let sol = Solution::new();
    println!("{}", sol.count_of_substrings(word, k));

    Ok(())
}