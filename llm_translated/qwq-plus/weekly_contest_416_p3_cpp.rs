struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> u64 {
        // Count the frequency of each character in word2
        let mut count = [0; 26];
        for c in word2.chars() {
            let idx = (c as u8 - b'a') as usize;
            count[idx] += 1;
        }

        let n = word1.len();
        // pre_count[i] holds the counts up to the i-th character (1-based)
        let mut pre_count = vec![[0; 26]; n + 1];

        for i in 1..=n {
            pre_count[i] = pre_count[i - 1];
            let c = word1.chars().nth(i - 1).unwrap();
            let idx = (c as u8 - b'a') as usize;
            pre_count[i][idx] += 1;
        }

        // Closure to find the minimal m where the substring [l, m] (1-based) has counts >= word2's counts
        let get = |l: usize, r: usize| -> usize {
            let border = l;
            let mut left = l;
            let mut right = r;
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

        let mut res: u64 = 0;
        // Iterate over each starting position l, find the minimal r where substring [l, r] is valid
        for l in 1..=n {
            let r = get(l, n + 1);
            res += (n as u64) - (r as u64) + 1;
        }
        res
    }
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    let mut iter = input.split_whitespace();

    // Read len1 (ignored), then word1, len2 (ignored), then word2
    let _len1: usize = iter.next().unwrap().parse().expect("Invalid len1");
    let word1 = iter.next().unwrap().to_string();
    let _len2: usize = iter.next().unwrap().parse().expect("Invalid len2");
    let word2 = iter.next().unwrap().to_string();

    let solution = Solution;
    let result = solution.valid_substring_count(word1, word2);
    println!("{}", result);
}