struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0i32; 26]; // Array of 26 integers initialized to 0
        for c in s.bytes() {
            let idx = (c - b'a') as usize;
            cnt[idx] += 1;
            // Adjust left pointer until current character's count is below k
            while cnt[idx] >= k {
                let left_char = s.as_bytes()[left];
                let left_idx = (left_char - b'a') as usize;
                cnt[left_idx] -= 1;
                left += 1;
            }
            ans += left; // Add all valid starting positions (0..left)
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let s = parts.next().expect("No string").to_string();
    let k: i32 = parts.next().expect("No k").parse().expect("Invalid k");
    println!("{}", Solution::number_of_substrings(s, k));
}