// Problem: Weekly Contest 416 Problem 3
use std::io;
use std::io::BufRead;

fn valid_substring_count(word1: String, word2: String) -> i64 {
    // Count character frequencies in word2
    let mut count = [0; 26];
    for c in word2.chars() {
        count[(c as u8 - b'a') as usize] += 1;
    }

    let n = word1.len();
    // pre_count[i][j] stores the count of character 'a' + j in word1[0...i-1]
    let mut pre_count: Vec<[i32; 26]> = vec![[0; 26]; n + 1];
    for i in 1..=n {
        pre_count[i] = pre_count[i - 1]; // Copy previous counts
        pre_count[i][(word1.as_bytes()[i - 1] - b'a') as usize] += 1; // Increment count for current char
    }

    // Binary search function to find the smallest r such that word1[l-1...r-1] is valid
    let get = |l: usize, r_bound: usize| -> usize {
        let mut l_bound = l;
        let mut r = r_bound;
        let border = l_bound; // Start of substring

        while l_bound < r {
            let m = l_bound + (r - l_bound) / 2;
            let mut f = true;
            for i in 0..26 {
                if pre_count[m][i] - pre_count[border - 1][i] < count[i] {
                    f = false;
                    break;
                }
            }
            if f {
                r = m; // Valid, try smaller r
            } else {
                l_bound = m + 1; // Not valid, need larger r
            }
        }
        l_bound // Return smallest valid r
    };

    let mut res: i64 = 0;
    for l in 1..=n {
        let r = get(l, n + 1);
        res += (n - r + 1) as i64; // Add count of valid substrings starting at l
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let len1: usize = lines.next().unwrap().expect("Failed to read line").trim().parse().expect("Failed to parse len1");
    let word1: String = lines.next().unwrap().expect("Failed to read line").trim().parse().expect("Failed to parse word1");
    let len2: usize = lines.next().unwrap().expect("Failed to read line").trim().parse().expect("Failed to parse len2");
    let word2: String = lines.next().unwrap().expect("Failed to read line").trim().parse().expect("Failed to parse word2");

    let result = valid_substring_count(word1, word2);
    println!("{}", result);
}