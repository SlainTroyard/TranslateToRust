use std::io::{self, Read};

fn number_of_substrings(s: &str, k: i32) -> i32 {
    let s_bytes = s.as_bytes();
    let mut cnt = [0; 26];
    let mut ans = 0;
    let mut left = 0;

    for &c in s_bytes {
        // Convert current character to index (0-25)
        let idx = (c - b'a') as usize;
        cnt[idx] += 1;

        // Move left pointer while current character's count exceeds k
        while cnt[idx] >= k {
            let left_idx = (s_bytes[left] - b'a') as usize;
            cnt[left_idx] -= 1;
            left += 1;
        }

        // All substrings ending at current position with start < left are valid
        ans += left as i32;
    }

    ans
}

fn main() {
    // Read all input at once and split by any whitespace
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut parts = input.split_whitespace();
    
    // Extract string and k (matching original cin >> s >> k behavior)
    let s = parts.next().expect("No string provided");
    let k: i32 = parts.next().expect("No k provided")
        .parse().expect("k must be an integer");
    
    println!("{}", number_of_substrings(s, k));
}