use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn valid_substring_count(word1: String, word2: String) -> i64 {
        // Count frequency of each character in word2
        let mut count = vec![0; 26];
        for c in word2.bytes() {
            count[(c - b'a') as usize] += 1;
        }

        let n = word1.len();
        // Prefix sum of character counts in word1
        let mut pre_count = vec![vec![0; 26]; n + 1];
        
        for i in 1..=n {
            pre_count[i] = pre_count[i - 1].clone();
            pre_count[i][(word1.as_bytes()[i - 1] - b'a') as usize] += 1;
        }

        // Binary search function to find the minimum valid substring end
        let get = |l: usize, r: usize| {
            let border = l;
            let mut l = l;
            let mut r = r;
            
            while l < r {
                let m = (l + r) >> 1;
                let mut f = true;
                
                // Check if the substring from border to m contains all required characters
                for i in 0..26 {
                    if pre_count[m][i] - pre_count[border - 1][i] < count[i] {
                        f = false;
                        break;
                    }
                }
                
                if f {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            
            l
        };

        // Calculate the number of valid substrings
        let mut res: i64 = 0;
        for l in 1..=n {
            let r = get(l, n + 1);
            res += (n - r + 1) as i64;
        }
        
        res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read length of word1 (not used for parsing)
    let len1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    // Read word1
    let word1 = lines.next().unwrap().unwrap();
    
    // Read length of word2 (not used for parsing)
    let len2: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    // Read word2
    let word2 = lines.next().unwrap().unwrap();
    
    println!("{}", Solution::valid_substring_count(word1, word2));
}