struct Solution;

impl Solution {
    fn has_special_substring(s: String, k: usize) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut cnt = 0;
        for i in 0..n {
            cnt += 1;
            if i == n - 1 || chars[i] != chars[i + 1] {
                if cnt == k {
                    return true;
                }
                cnt = 0;
            }
        }
        false
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 2 {
        panic!("Input must have exactly two parts");
    }

    let s = parts[0].to_string();
    let k: usize = parts[1].parse()
        .expect("Second part must be an integer");

    let result = Solution::has_special_substring(s, k);
    println!("{}", if result { 1 } else { 0 });
}