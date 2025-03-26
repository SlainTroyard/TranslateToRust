use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut count = vec![0; 26];
        for c in word2.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        let n = word1.len();
        let mut pre_count = vec![vec![0; 26]; n + 1];
        for i in 1..=n {
            pre_count[i] = pre_count[i - 1].clone();
            let char_idx = (word1.as_bytes()[i - 1] - b'a') as usize;
            pre_count[i][char_idx] += 1;
        }

        // Binary search function to find the rightmost position that contains all characters
        let get = |l: usize, r: usize| -> usize {
            let border = l;
            let mut left = l;
            let mut right = r;
            
            while left < right {
                let m = left + (right - left) / 2;
                let mut f = true;
                
                for i in 0..26 {
                    if pre_count[m][i] - pre_count[border - 1][i] < count[i] {
                        f = false;
                        break;
                    }
                }
                
                if f {
                    right = m;
                } else {
                    left = m + 1;
                }
            }
            
            left
        };

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
    
    // Read length of word1 (though not used in the function)
    let _len1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read word1
    let word1 = lines.next().unwrap().unwrap();
    
    // Read length of word2 (though not used in the function)
    let _len2: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read word2
    let word2 = lines.next().unwrap().unwrap();
    
    // Call the solution function and output the result
    let s = Solution {};
    println!("{}", Solution::valid_substring_count(word1, word2));
}