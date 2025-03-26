use std::io;

struct Solution;

impl Solution {
    fn valid_substring_count(word1: String, word2: String) -> i64 {
        // Count the frequency of each character in word2
        let mut count = [0; 26];
        for c in word2.chars() {
            count[(c as usize - 'a' as usize)] += 1;
        }

        let n = word1.len();
        // Prefix sums for each character in word1
        let mut pre_count = vec![[0; 26]; n + 1];
        for i in 1..=n {
            pre_count[i] = pre_count[i - 1];
            let c = word1.chars().nth(i - 1).unwrap();
            pre_count[i][(c as usize - 'a' as usize)] += 1;
        }

        // Helper closure to find the minimal right for a given left using binary search
        let get = |l: usize| -> usize {
            let mut l = l;
            let mut r = n + 1;
            while l < r {
                let m = (l + r) >> 1;
                let mut valid = true;
                for i in 0..26 {
                    if pre_count[m][i] - pre_count[l - 1][i] < count[i] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            l
        };

        let mut res = 0;
        for l in 1..=n {
            let r = get(l);
            res += (n - r + 1) as i64;
        }
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let lines: Vec<&str> = input.lines().collect();

    let word1 = lines[1].trim().to_string();
    let word2 = lines[3].trim().to_string();

    let solution = Solution;
    println!("{}", solution.valid_substring_count(word1, word2));
}