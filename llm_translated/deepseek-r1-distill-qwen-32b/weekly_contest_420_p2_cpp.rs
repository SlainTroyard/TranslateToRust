struct Solution;

impl Solution {
    fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26];
        for (i, c) in s.chars().enumerate() {
            let idx = (c as u8 - b'a') as usize;
            cnt[idx] += 1;
            while cnt[idx] >= k {
                let left_char = s.as_bytes()[left] as char;
                let left_idx = (left_char as u8 - b'a') as usize;
                cnt[left_idx] -= 1;
                left += 1;
            }
            ans += left;
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0].to_string();
    let k = parts[1].parse::<i32>().expect("Invalid k");
    let sol = Solution;
    println!("{}", sol.number_of_substrings(s, k));
}