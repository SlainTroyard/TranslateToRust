use std::io::{self, Read};
use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: &str, word2: &str) -> i64 {
        let mut count = vec![0; 26];
        for c in word2.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        let n = word1.len();
        let mut pre_count = vec![vec![0; 26]; n + 1];
        let word1_bytes = word1.as_bytes();

        for i in 1..=n {
            pre_count[i].clone_from_slice(&pre_count[i - 1]);
            pre_count[i][(word1_bytes[i - 1] - b'a') as usize] += 1;
        }

        let get = |l: usize, r: usize| -> usize {
            let mut left = l;
            let mut right = r;
            let border = l;

            while left < right {
                let m = (left + right) / 2;
                let mut valid = true;

                for i in 0..26 {
                    if pre_count[m][i] - pre_count[border - 1][i] < count[i] {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    right = m;
                } else {
                    left = m + 1;
                }
            }

            left
        };

        let mut res = 0i64;
        for l in 1..=n {
            let r = get(l, n + 1);
            res += (n - r + 1) as i64;
        }

        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Read the first input line
    let len1: usize = lines.next().unwrap().parse().unwrap();
    let word1 = lines.next().unwrap();

    // Read the second input line
    let len2: usize = lines.next().unwrap().parse().unwrap();
    let word2 = lines.next().unwrap();

    // Ensure the lengths match the input strings
    assert_eq!(len1, word1.len());
    assert_eq!(len2, word2.len());

    let solution = Solution;
    let result = solution.valid_substring_count(word1, word2);

    // Output the result
    println!("{}", result);
}