use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the length of word1
    let len1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read word1
    let word1 = lines.next().unwrap().unwrap();
    
    // Read the length of word2
    let len2: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read word2
    let word2 = lines.next().unwrap().unwrap();
    
    // Calculate and print the result
    let result = valid_substring_count(&word1, &word2);
    println!("{}", result);
}

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    // Count the frequency of each character in word2
    let mut count = [0; 26];
    for c in word2.chars() {
        count[(c as u8 - b'a') as usize] += 1;
    }

    let n = word1.len();
    // Precompute the prefix counts for each character in word1
    let mut pre_count = vec![[0; 26]; n + 1];
    for i in 1..=n {
        pre_count[i] = pre_count[i - 1];
        let c = word1.chars().nth(i - 1).unwrap();
        pre_count[i][(c as u8 - b'a') as usize] += 1;
    }

    // Binary search function to find the earliest index r where the substring word1[l..r] contains all characters of word2
    let get = |l: usize, r: usize| {
        let mut l = l;
        let mut r = r;
        let border = l;
        while l < r {
            let m = (l + r) / 2;
            let mut f = true;
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

    let mut res = 0;
    // Iterate over all possible starting positions l in word1
    for l in 1..=n {
        let r = get(l, n + 1);
        res += (n - r + 1) as i64;
    }
    res
}