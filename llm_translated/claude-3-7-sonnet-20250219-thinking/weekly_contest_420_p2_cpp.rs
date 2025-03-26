use std::io;

struct Solution;

impl Solution {
    fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26];
        let s_bytes = s.as_bytes();
        
        for &c in s_bytes {
            let idx = (c - b'a') as usize;
            cnt[idx] += 1;
            
            while cnt[idx] >= k {
                let left_char = s_bytes[left] - b'a';
                cnt[left_char as usize] -= 1;
                left += 1;
            }
            
            ans += left as i32;
        }
        
        ans
    }
}

fn main() {
    // Read the input string
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read string");
    let s = s.trim().to_string();
    
    // Read the k value
    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read k");
    let k: i32 = k_str.trim().parse().expect("Failed to parse k as integer");
    
    // Create solution and print result
    let sol = Solution;
    println!("{}", sol.number_of_substrings(s, k));
}