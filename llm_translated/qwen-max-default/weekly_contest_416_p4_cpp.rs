use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let len1: usize = input.trim().parse().expect("Please type a number!");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let word1 = input.trim().to_string();
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let len2: usize = input.trim().parse().expect("Please type a number!");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let word2 = input.trim().to_string();

    let solution = Solution;
    let result = solution.valid_substring_count(word1, word2);
    println!("{}", result);
}

struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut diff = vec![0; 26];
        for c in word2.chars() {
            diff[c as usize - 'a' as usize] -= 1;
        }

        let mut res: i64 = 0;
        let mut cnt = diff.iter().filter(|&&c| c < 0).count();
        
        let update = |c: usize, add: i32| {
            diff[c] += add;
            if add == 1 && diff[c] == 0 {
                // 表明 diff[c] 由 -1 变为 0
                cnt -= 1;
            } else if add == -1 && diff[c] == -1 {
                // 表明 diff[c] 由 0 变为 -1
                cnt += 1;
            }
        };

        let mut l = 0;
        let mut r = 0;
        while l < word1.len() {
            while r < word1.len() && cnt > 0 {
                update(word1.as_bytes()[r] as usize - 'a' as usize, 1);
                r += 1;
            }
            if cnt == 0 {
                res += (word1.len() - r + 1) as i64;
            }
            update(word1.as_bytes()[l] as usize - 'a' as usize, -1);
            l += 1;
        }
        res
    }
}